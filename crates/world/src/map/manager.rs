use std::ops::Deref;

use rand::{prelude::IteratorRandom, Rng};

use pokedex::{
    item::Item,
    moves::Move,
    moves::MoveId,
    pokemon::Pokemon,
    trainer::{InitTrainer, Trainer},
};

use crate::{
    character::{action::ActionQueue, player::PlayerCharacter, Activity, DoMoveResult, CharacterState},
    map::{MovementId, WarpDestination, WorldMap},
    positions::{BoundingBox, Direction, Location},
    random::WorldRandoms,
    script::WorldScriptingEngine,
    state::{map::{MapState, MapEvent}, WorldState},
};

use super::{
    data::WorldMapData,
    movement::{Elevation, MapMovementResult},
};

pub struct WorldMapManager<S: WorldScriptingEngine> {
    pub data: WorldMapData,
    /// Scripting engine
    pub scripting: S,
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Move(Direction),
    Interact,
}

impl<S: WorldScriptingEngine> WorldMapManager<S> {
    pub fn new(data: WorldMapData, scripting: S) -> Self {
        Self { data, scripting }
    }

    pub fn contains(&self, location: &Location) -> bool {
        self.data.maps.contains_key(location)
    }

    pub fn get(&self, location: &Location) -> Option<&WorldMap> {
        self.data.maps.get(location)
    }

    pub fn on_warp<R: Rng, P, B>(
        &self,
        state: &mut MapState,
        randoms: &mut WorldRandoms<R>,
        trainer: &Trainer<P, B>,
    ) {
        self.on_map_change(state);
        self.on_tile(state, randoms, trainer);
    }

    pub fn on_map_change(&self, state: &mut MapState) {
        if let Some(map) = self.data.maps.get(&state.location) {
            self.on_change(map, state);
        }
    }

    pub fn on_change(&self, map: &WorldMap, state: &mut MapState) {
        state.events.push(MapEvent::PlayMusic(Some(map.music)));
        state.update_objects(&self.data);
        // check for cave here and add last spot non cave for escape rope
    }

    pub fn input(&self, state: &mut MapState, input: InputEvent) {
        match input {
            InputEvent::Move(direction) => self.try_move_player(state, direction),
            InputEvent::Interact => state.player.character.queue_interact(true),
        }
    }

    pub fn try_interact(&self, state: &mut MapState) {
        if let Some(map) = self.data.maps.get(&state.location) {
            let pos = if map
                .tile(state.player.character.position.coords)
                .map(|tile| {
                    self.data
                        .palettes
                        .get(tile.palette(&map.palettes))
                        .map(|data| (tile.id(), data))
                })
                .flatten()
                .map(|(tile, data)| data.forwarding.contains(&tile))
                .unwrap_or_default()
            {
                state.player.character.position.next()
            } else {
                state.player.character.position
            };

            for (id, npc) in state
                .entities
                .get_mut(&state.location)
                .map(|state| state.npcs.iter_mut())
                .into_iter()
                .flatten()
            {
                if npc.interact_from(&pos) {
                    state.player.character.input_lock.increment();
                    break;
                }
            }

            let forward = state.player.character.position.forwards();

            // if let Some(object) = map.object_at(&forward) {
                
            // }

            // if let Some(item) = map.item_at(&forward) {
            //     item.pickup(&map.id, forward, state);
            // }
        }
    }

    pub fn move_npcs<R: Rng>(
        &self,
        state: &mut MapState,
        randoms: &mut WorldRandoms<R>,
        delta: f32,
    ) {
        if let Some(map) = self.data.maps.get(&state.location) {
            // Move Npcs

            for (id, character) in state
                .entities
                .get_mut(&state.location)
                .map(|state| state.npcs.iter_mut())
                .into_iter()
                .flatten()
            {
                if let Some(r) = character.do_move(delta) {
                    state.npc.results.push((*id, r));
                }
            }

            use crate::character::npc::NpcMovement;

            match state.npc.timer > 0.0 {
                false => {
                    state.npc.timer += 1.0;

                    const NPC_MOVE_CHANCE: f64 = 1.0 / 12.0;

                    for (id, character) in state
                        .entities
                        .get_mut(&state.location)
                        .map(|state| state.npcs.iter_mut())
                        .into_iter()
                        .flatten()
                    {
                        if !character.moving() {
                            if randoms.npc.gen_bool(NPC_MOVE_CHANCE) {
                                let npc = self
                                    .data
                                    .maps
                                    .get(&state.location)
                                    .unwrap()
                                    .npcs
                                    .get(id)
                                    .unwrap();
                                for movement in npc.movement.iter() {
                                    match movement {
                                        NpcMovement::Look(directions) => {
                                            if let Some(direction) =
                                                directions.iter().choose(&mut randoms.npc)
                                            {
                                                character.position.direction = *direction;
                                            }
                                        }
                                        NpcMovement::Move(area) => {
                                            let next = character.position.forwards();

                                            let bb =
                                                BoundingBox::centered(npc.origin.coords, *area);

                                            if bb.contains(&next)
                                                && next != state.player.character.position.coords
                                            {
                                                if let Some(code) = map.movements.get(
                                                    character.position.coords.x as usize
                                                        + character.position.coords.y as usize
                                                            * map.width as usize,
                                                ) {
                                                    if Elevation::can_move(
                                                        character.position.elevation,
                                                        *code,
                                                    ) {
                                                        character.actions.queue.push(
                                                            ActionQueue::Move(
                                                                character.position.direction,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                true => state.npc.timer -= delta,
            }
        }
    }

    pub fn on_tile<R: Rng, P, B>(
        &self,
        state: &mut MapState,
        randoms: &mut WorldRandoms<R>,
        trainer: &Trainer<P, B>,
    ) {
        // state.events.push(MapEvent::OnTile);

        if !trainer.party.is_empty() && state.player.character.capabilities.contains(&CharacterState::ENCOUNTERS) {
            if let Some(map) = self.data.maps.get(&state.location) {
                map.try_wild_battle(&self.data, state, randoms);

                for (id, npc) in map.npcs.iter() {
                    if let Some(trainer) = &npc.trainer {
                        if let Some(character) = state
                            .entities
                            .get_mut(&state.location)
                            .and_then(|state| state.npcs.get_mut(id))
                        {
                            state
                                .player
                                .find_battle(&map.id, &npc.id, trainer, character);
                        }
                    }
                }
            }
        }
    }

    fn stop_player<R: Rng, P, B>(
        &self,
        state: &mut MapState,
        randoms: &mut WorldRandoms<R>,
        trainer: &Trainer<P, B>,
    ) {
        state.player.character.stop_move();

        if let Some(map) = self.data.maps.get(&state.location) {
            if let Some(destination) = map.warp_at(&state.player.character.position.coords) {
                // Warping does not trigger tile actions!
                state.warp = Some(*destination);
            } else if map.in_bounds(state.player.character.position.coords) {
                self.on_tile(state, randoms, trainer);
            }
        }
    }

    pub fn update<
        R: Rng,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        state: &mut WorldState<S>,
        trainer: &mut InitTrainer<P, M, I>,
        randoms: &mut WorldRandoms<R>,
        delta: f32,
    ) {
        if let Some(result) = state.map.player.update(&mut state.map.message, delta) {
            match result {
                DoMoveResult::Finished => self.stop_player(&mut state.map, randoms, trainer),
                DoMoveResult::Interact => self.try_interact(&mut state.map),
            }
        }
        self.move_npcs(&mut state.map, randoms, delta);
        self.scripting.update(
            &self.data,
            &mut state.map,
            trainer,
            randoms,
            &mut state.scripts,
        );
        // self.update_interactions(player);
    }

    pub fn try_move_player(&self, state: &mut MapState, direction: Direction) {
        state.player.character.on_try_move(direction);

        let offset = direction.tile_offset();
        let coords = state.player.character.position.coords + offset;

        if let Some(map) = self.get(&state.location) {
            // Check for warp on tile
            if state.warp.is_none() {
                if let Some(destination) = map.warp_at(&coords) {
                    state.warp = Some(*destination);
                    state.events.push(MapEvent::BeginWarpTransition(coords));
                    return;
                }
            };

            // Check for one-way tile
            if map
                .tile(coords)
                .map(|tile| {
                    self.data
                        .palettes
                        .get(tile.palette(&map.palettes))
                        .map(|data| {
                            data.cliffs
                                .get(&direction)
                                .map(|tiles| tiles.contains(&tile.id()))
                        })
                })
                .flatten()
                .flatten()
                .unwrap_or_default()
                && !state.player.character.capabilities.contains(&CharacterState::NOCLIP)
            {
                state.events.push(MapEvent::PlayerJump);
                return;
            }

            match map.chunk_movement(coords, state) {
                MapMovementResult::Option(code) => {
                    with_code(&mut state.player, code.unwrap_or(1), direction);
                }
                MapMovementResult::Chunk(direction, offset, connection) => {
                    if let Some((location, coords, code)) = self
                        .data
                        .connection_movement(direction, offset, connection, state)
                    {
                        if with_code(&mut state.player, code, direction) {
                            state.player.character.position.coords = coords;
                            state.location = location;
                            self.on_map_change(state);
                        }
                    }
                }
            }
        }

        fn with_code(player: &mut PlayerCharacter, code: MovementId, direction: Direction) -> bool {
            if Elevation::can_move(player.character.position.elevation, code)
                || player.character.capabilities.contains(&CharacterState::NOCLIP)
            {
                if Elevation::WATER == code {
                    if player.character.activity != Activity::Swimming {
                        const SURF: &MoveId = unsafe {
                            &MoveId(tinystr::TinyStr16::from_bytes_unchecked(
                                1718777203u128.to_ne_bytes(),
                            ))
                        };

                        if player.character.capabilities.contains(&CharacterState::SWIM)
                            || player.character.capabilities.contains(&CharacterState::NOCLIP)
                        {
                            player.character.activity = Activity::Swimming;
                        } else {
                            return false;
                        }
                    }
                } else if player.character.activity == Activity::Swimming {
                    player.character.activity = Activity::Walking;
                }
                Elevation::change(&mut player.character.position.elevation, code);
                player
                    .character
                    .actions
                    .queue
                    .push(ActionQueue::Move(direction));
                return true;
                // self.player.offset =
                //     direction.pixel_offset(self.player.speed() * 60.0 * delta);
            }
            false
        }
    }

    pub fn warp<R: Rng, P, B>(
        &self,
        state: &mut MapState,
        randoms: &mut WorldRandoms<R>,
        trainer: &Trainer<P, B>,
        destination: WarpDestination,
    ) {
        if self.data.warp(state, destination) {
            self.on_warp(state, randoms, trainer);
        }
    }
}
