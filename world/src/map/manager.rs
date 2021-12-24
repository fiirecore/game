use std::ops::Deref;

use hashbrown::HashMap;
use rand::{Rng, SeedableRng};

use pokedex::{
    item::Item,
    moves::{Move, MoveId},
    pokemon::{owned::OwnedPokemon, Pokemon},
};
use serde::{Deserialize, Serialize};

use crate::{
    actions::WorldActions,
    character::{
        npc::{
            group::{MessageColor, NpcGroup, NpcGroupId},
            trainer::TrainerDisable,
            NpcInteract,
        },
        player::PlayerCharacter,
        Movement,
    },
    events::Sender,
    map::{
        movement::{can_move, can_swim, can_walk},
        MovementId, WarpDestination, WorldMap,
    },
    positions::{Coordinate, Direction, Location, Position},
};

use super::{
    battle::BattleEntry,
    chunk::Connection,
    movement::MovementResult,
    wild::{WildChances, WildEntry, WildType},
};

use self::{
    random::WorldRandoms,
    tile::{PaletteTileData, PaletteTileDatas},
};

pub mod state;
pub mod tile;

mod random;

pub type Maps = HashMap<Location, WorldMap>;

pub struct WorldMapManager<R: Rng + SeedableRng + Clone> {
    pub data: WorldMapData,
    sender: Sender<WorldActions>,
    randoms: WorldRandoms<R>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldMapData {
    pub maps: Maps,
    pub tiles: PaletteTileDatas,
    pub npcs: HashMap<NpcGroupId, NpcGroup>,
    pub wild: WildChances,
    pub default: (Location, Position),
}

impl<R: Rng + SeedableRng + Clone> WorldMapManager<R> {
    pub fn new(data: WorldMapData, sender: Sender<WorldActions>) -> Self {
        Self {
            data,
            sender,
            randoms: Default::default(),
        }
    }

    pub fn seed(&mut self, seed: u64) {
        self.randoms.seed(seed);
    }

    pub fn contains(&self, location: &Location) -> bool {
        self.data.maps.contains_key(location)
    }

    pub fn get(&self, location: &Location) -> Option<&WorldMap> {
        self.data.maps.get(location)
    }

    pub fn on_warp(&mut self, player: &mut PlayerCharacter) {
        self.on_map_change(player);
        self.on_tile(player);
    }

    pub fn on_map_change(&self, player: &PlayerCharacter) {
        if let Some(map) = self.data.maps.get(&player.location) {
            self.on_change(map);
        }
    }

    pub fn on_change(&self, map: &WorldMap) {
        self.sender.send(WorldActions::PlayMusic(map.music));
    }

    pub fn try_interact(&mut self, player: &mut PlayerCharacter) {
        if player.world.npc.active.is_none() {
            if let Some(map) = self.data.maps.get_mut(&player.location) {
                let pos = if map
                    .tile(player.position.coords)
                    .map(|tile| {
                        PaletteTileData::iter(&self.data.tiles, &map.palettes)
                            .any(|t| t.forwarding.contains(&tile))
                    })
                    .unwrap_or_default()
                {
                    player.position.in_direction(player.position.direction)
                } else {
                    player.position
                };
                for (id, npc) in map.npcs.iter_mut() {
                    if (npc.interact.is_some() || npc.trainer.is_some()) && npc.interact_from(&pos)
                    {
                        player.world.npc.active = Some(*id);
                    }
                }
            }
        }
    }

    pub fn post_battle<
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        player: &mut PlayerCharacter,
        party: &mut [OwnedPokemon<P, M, I>],
        winner: bool,
        trainer: bool,
    ) {
        player.unfreeze();
        if winner {
            if let Some(entry) = player.world.battle.battling.take() {
                if trainer {
                    if let Some(trainer) = self
                        .data
                        .maps
                        .get(&entry.location)
                        .map(|map| map.npcs.get(&entry.id).map(|npc| npc.trainer.as_ref()))
                        .flatten()
                        .flatten()
                    {
                        match &trainer.disable {
                            TrainerDisable::DisableSelf => {
                                player.world.battle.insert(&entry.location, entry.id);
                            }
                            TrainerDisable::Many(others) => {
                                player.world.battle.insert(&entry.location, entry.id);
                                player
                                    .world
                                    .battle
                                    .battled
                                    .get_mut(&entry.location)
                                    .unwrap()
                                    .extend(others);
                            }
                            TrainerDisable::None => (),
                        }
                    }
                }
            }
        } else {
            let loc = player.world.heal.unwrap_or(self.data.default);
            player.location = loc.0;
            player.position = loc.1;
            player.location = player.location;
            party.iter_mut().for_each(|o| o.heal(None, None));
        }
    }

    pub fn move_npcs(&mut self, player: &mut PlayerCharacter, delta: f32) {
        if let Some(map) = self.data.maps.get_mut(&player.location) {
            // Move Npcs

            for npc in player
                .world
                .scripts
                .npcs
                .values_mut()
                .filter(|(location, ..)| map.contains(location))
                .map(|(.., npc)| npc)
            {
                npc.character.do_move(delta);
            }

            for npc in map.npcs.values_mut() {
                npc.character.do_move(delta);
            }

            use crate::{character::npc::NpcMovement, positions::Destination};

            match player.world.npc.timer > 0.0 {
                false => {
                    player.world.npc.timer += 1.0;

                    const NPC_MOVE_CHANCE: f64 = 1.0 / 12.0;

                    for (index, npc) in map.npcs.iter_mut() {
                        if !npc.character.moving() && self.randoms.npc.gen_bool(NPC_MOVE_CHANCE) {
                            match npc.movement {
                                NpcMovement::Still => (),
                                NpcMovement::LookAround => {
                                    npc.character.position.direction =
                                        Direction::DIRECTIONS[self.randoms.npc.gen_range(0..4)];
                                    player.find_battle(&map.id, index, npc);
                                }
                                NpcMovement::WalkUpAndDown(steps) => {
                                    let origin =
                                        npc.origin.get_or_insert(npc.character.position.coords);
                                    let direction = if npc.character.position.coords.y
                                        <= origin.y - steps as i32
                                    {
                                        Direction::Down
                                    } else if npc.character.position.coords.y
                                        >= origin.y + steps as i32
                                    {
                                        Direction::Up
                                    } else if self.randoms.npc.gen_bool(0.5) {
                                        Direction::Down
                                    } else {
                                        Direction::Up
                                    };
                                    let coords =
                                        npc.character.position.coords.in_direction(direction);
                                    if can_move(
                                        npc.character.movement,
                                        map.movements[npc.character.position.coords.x as usize
                                            + npc.character.position.coords.y as usize
                                                * map.width as usize],
                                    ) {
                                        npc.character.position.direction = direction;
                                        if !player.find_battle(&map.id, index, npc)
                                            && coords.y != player.position.coords.y
                                        {
                                            npc.character.pathing.extend(
                                                &npc.character.position,
                                                Destination::to(&npc.character.position, coords),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                true => player.world.npc.timer -= delta,
            }
        }
    }

    pub fn update_interactions(
        &mut self,
        player: &mut PlayerCharacter,
        active: bool,
        finished: bool,
    ) {
        if let Some(id) = player.world.npc.active.as_ref() {
            let npc = self
                .data
                .maps
                .get_mut(&player.location)
                .map(|map| map.npcs.get_mut(&id))
                .flatten();
            let npc = if let Some(npc) = npc {
                Some(npc)
            } else {
                player
                    .world
                    .scripts
                    .npcs
                    .get_mut(id)
                    .filter(|(location, ..)| &player.location == location)
                    .map(|(.., npc)| npc)
            };
            if let Some(npc) = npc {
                match active {
                    true => {
                        if finished {
                            if let Some(battle) = BattleEntry::trainer(
                                &mut player.world.battle,
                                &player.location,
                                id,
                                npc,
                            ) {
                                self.sender.send(WorldActions::Battle(battle));
                            }
                            if player.frozen() {
                                player.unfreeze();
                            }
                            player.world.npc.active = None;
                        }
                    }
                    false => {
                        if !npc.character.moving() {
                            if !player.world.battle.battled(&player.location, id) {
                                if let Some(group) = self.data.npcs.get(&npc.group) {
                                    if let Some(trainer) = npc.trainer.as_ref() {
                                        if trainer.battle_on_interact {
                                            // Spawn text window
                                            if let Some(music) = group
                                                .trainer
                                                .as_ref()
                                                .map(|trainer| trainer.music)
                                                .flatten()
                                            {
                                                self.sender.send(WorldActions::PlayMusic(music));
                                            }
                                            self.sender.send(WorldActions::Message(
                                                trainer.encounter.clone(),
                                                group.message,
                                            ));
                                            return player.position.direction =
                                                npc.character.position.direction.inverse();
                                        }
                                    }
                                }
                            }

                            match &npc.interact {
                                NpcInteract::Message(pages) => {
                                    self.sender.send(WorldActions::Message(
                                        pages.clone(),
                                        self.data
                                            .npcs
                                            .get(&npc.group)
                                            .map(|group| group.message)
                                            .unwrap_or(MessageColor::Black),
                                    ));
                                    return player.position.direction =
                                        npc.character.position.direction.inverse();
                                }
                                NpcInteract::Script(_) => todo!(),
                                NpcInteract::Nothing => (),
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn on_tile(
        &mut self,
        player: &mut PlayerCharacter,
        // party: &[OwnedPokemon<P, M, I>],
    ) {
        self.sender.send(WorldActions::OnTile);

        if let Some(map) = self.data.maps.get_mut(&player.location) {
            if player.world.wild.encounters {
                if let Some(current) = map.tile(player.position.coords) {
                    if PaletteTileData::iter(&self.data.tiles, &map.palettes)
                        .any(|t| t.wild.contains(&current))
                    {
                        let t = match player.movement {
                            Movement::Swimming => &WildType::Water,
                            _ => &WildType::Land,
                        };
                        if let Some(entry) =
                            &map.wild.as_ref().map(|entries| entries.get(t)).flatten()
                        {
                            if let Some(entry) = WildEntry::generate(
                                &self.data.wild,
                                t,
                                entry,
                                &mut self.randoms.wild,
                            ) {
                                player.freeze();
                                self.sender.send(WorldActions::Battle(entry));
                            }
                        }
                    }
                }
            }

            if player.world.npc.active.is_none() {
                for (id, npc) in map.npcs.iter_mut().filter(|(_, npc)| npc.trainer.is_some()) {
                    player.find_battle(&map.id, id, npc);
                }
            }
        }

        // if let Some(tile_id) = map.tile(player.position.coords) {
        //     // look for player

        //     // try running scripts

        //     if player.world.scripts.actions.is_empty() {
        //         'scripts: for script in map.scripts.iter() {
        //             use worldlib::script::world::Condition;
        //             for condition in &script.conditions {
        //                 match condition {
        //                     Condition::Location(location) => {
        //                         if !location.in_bounds(&player.position.coords) {
        //                             continue 'scripts;
        //                         }
        //                     }
        //                     Condition::Activate(direction) => {
        //                         if player.position.direction.ne(direction) {
        //                             continue 'scripts;
        //                         }
        //                     }
        //                     Condition::NoRepeat => {
        //                         if player.world.scripts.executed.contains(&script.identifier) {
        //                             continue 'scripts;
        //                         }
        //                     }
        //                     Condition::Script(script, happened) => {
        //                         if player.world.scripts.executed.contains(script).ne(happened) {
        //                             continue 'scripts;
        //                         }
        //                     }
        //                     Condition::PlayerHasPokemon(is_true) => {
        //                         if party.is_empty().eq(is_true) {
        //                             continue 'scripts;
        //                         }
        //                     }
        //                 }
        //             }
        //             player
        //                 .world
        //                 .scripts
        //                 .actions
        //                 .extend_from_slice(&script.actions);
        //             player.world.scripts.actions.push(
        //                 worldlib::script::world::WorldAction::Finish(script.identifier),
        //             );
        //             player.world.scripts.actions.reverse();
        //             break;
        //         }
        //     }
        // }
    }

    fn stop_player(&mut self, player: &mut PlayerCharacter) {
        player.stop_move();

        if let Some(map) = self.data.maps.get(&player.location) {
            if let Some(destination) = map.warp_at(player.position.coords) {
                // Warping does not trigger tile actions!
                player.world.warp = Some(*destination);
            } else if map.in_bounds(player.position.coords) {
                self.on_tile(player);
            }
        }
    }

    pub fn update(&mut self, player: &mut PlayerCharacter, delta: f32) {
        if player.do_move(delta) {
            self.stop_player(player);
        }
        self.move_npcs(player, delta);
    }

    pub fn try_move<
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        player: &mut PlayerCharacter,
        party: &[OwnedPokemon<P, M, I>],
        direction: Direction,
    ) {
        player.on_try_move(direction);

        let offset = direction.tile_offset();
        let coords = player.position.coords + offset;

        if let Some(map) = self.get(&player.location) {
            // Check for warp on tile
            if player.world.warp.is_none() {
                if let Some(destination) = map.warp_at(coords) {
                    player.world.warp = Some(*destination);
                    self.sender.send(WorldActions::BeginWarpTransition(coords));
                    return;
                }
            };

            // Check for one-way tile
            if map
                .tile(coords)
                .map(|tile| {
                    PaletteTileData::iter(&self.data.tiles, &map.palettes).any(|t| {
                        t.cliffs
                            .get(&direction)
                            .map(|tiles| tiles.contains(&tile))
                            .unwrap_or_default()
                    })
                })
                .unwrap_or_default()
                && !player.noclip
            {
                self.sender.send(WorldActions::PlayerJump);
                return;
            }

            match map.chunk_movement(coords) {
                MovementResult::Option(code) => {
                    with_code(player, party, code.unwrap_or(1), direction);
                }
                MovementResult::Chunk(direction, offset, connection) => {
                    if let Some((coords, code)) =
                        self.connection_movement(direction, offset, connection)
                    {
                        if with_code(player, party, code, direction) {
                            player.character.position.coords = coords;
                            player.location = connection.0;
                            self.on_map_change(&player);
                        }
                    }
                }
            }
        }

        fn with_code<
            P: Deref<Target = Pokemon>,
            M: Deref<Target = Move>,
            I: Deref<Target = Item>,
        >(
            player: &mut PlayerCharacter,
            party: &[OwnedPokemon<P, M, I>],
            code: MovementId,
            direction: Direction,
        ) -> bool {
            if player.movement == Movement::Swimming && can_walk(code) {
                player.movement = Movement::Walking
            }

            if can_swim(code) && player.movement != Movement::Swimming {
                const SURF: &MoveId = unsafe { &MoveId::new_unchecked(1718777203) };

                if party
                    .iter()
                    .flat_map(|pokemon| pokemon.moves.iter())
                    .any(|m| &m.0.id == SURF)
                    || player.noclip
                {
                    player.movement = Movement::Swimming;
                    player.pathing.queue.push(direction);
                    return true;
                }
            } else if can_move(player.movement, code) || player.noclip {
                player.pathing.queue.push(direction);
                return true;
                // self.player.offset =
                //     direction.pixel_offset(self.player.speed() * 60.0 * delta);
            }
            false
        }

        // // check if on unwalkable tile
        // let stuck = map.local_movement(player.position.coords)
        //             .map(|code| !can_move(player.movement, code))
        //             .unwrap_or(false);
        // println!("{}", stuck);
    }

    pub fn connection_movement(
        &self,
        direction: Direction,
        offset: i32,
        connection: &Connection,
    ) -> Option<(Coordinate, MovementId)> {
        self.data
            .maps
            .get(&connection.0)
            .map(|map| {
                let o = offset - connection.1;
                let position = Connection::offset(direction, map, o);
                let coords = position.in_direction(direction);
                map.local_movement(coords).map(|code| (position, code))
            })
            .flatten()
    }

    pub fn warp(&mut self, player: &mut PlayerCharacter, destination: WarpDestination) -> bool {
        match self.data.maps.contains_key(&destination.location) {
            true => {
                player.position.from_destination(destination.position);
                player.pathing.clear();
                player.location = destination.location;
                true
            }
            false => false,
        }
    }
}
