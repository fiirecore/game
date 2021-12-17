use std::collections::HashMap;

use crate::saves::{GamePokemon, PlayerData};
use crate::Sender;
use crate::{engine::log::info, world::WorldActions};
use crate::{
    engine::{
        graphics::Texture,
        gui::MessagePage,
        input::controls::{pressed, Control},
        util::{Completable, Entity},
        {graphics::Color, Context},
    },
    world::npc::NpcTypes,
};
use crate::{
    pokedex::{context::PokedexClientData, moves::MoveId},
    world::npc::color,
};
use firecore_battle::pokedex::{
    item::Item,
    moves::Move,
    pokemon::{party::Party, Pokemon},
    Dex,
};
use firecore_battle_gui::pokedex::engine::log::warn;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use worldlib::{
    character::{
        npc::{MessageColor, NpcType, NpcTypeId},
        player::PlayerCharacter,
        sprite::SpriteIndexes,
        Movement,
    },
    map::{
        chunk::Connection,
        manager::{TryMoveResult, WorldMapManager},
        Brightness, WorldMap,
    },
    positions::{Coordinate, Direction, Location},
    serialized::SerializedWorld,
};

use crate::world::{
    gui::TextWindow,
    map::{input::PlayerInput, texture::WorldTextures, warp::WarpTransition},
    RenderCoords,
};

pub mod script;

pub struct GameWorldMapManager {
    world: WorldMapManager,

    textures: WorldTextures,
    npc_types: NpcTypes,

    warper: WarpTransition,
    input: PlayerInput,
    screen: RenderCoords,

    sender: Sender<WorldActions>,

    pub randoms: Randoms,
}

pub struct Randoms {
    pub wild: SmallRng,
    pub npc: SmallRng,
}

impl Default for Randoms {
    fn default() -> Self {
        let rng = SmallRng::seed_from_u64(0);
        Self {
            wild: rng.clone(),
            npc: rng,
        }
    }
}

impl Randoms {
    pub fn seed(&mut self, seed: u64) {
        let rng = SmallRng::seed_from_u64(seed);
        self.wild = rng.clone();
        self.npc = rng;
    }
}

impl GameWorldMapManager {
    pub(crate) fn new(ctx: &mut Context, sender: Sender<WorldActions>) -> Self {
        Self {
            world: WorldMapManager::default(),

            npc_types: Default::default(),
            textures: WorldTextures::new(ctx),
            warper: WarpTransition::new(),
            input: PlayerInput::default(),
            screen: RenderCoords::default(),

            sender,

            randoms: Randoms::default(),
        }
    }

    pub fn load(&mut self, ctx: &mut Context, world: SerializedWorld) {
        self.textures.setup(ctx, &mut self.warper, world.textures);

        info!("Finished loading textures!");

        let (textures, types): (
            crate::world::map::texture::npc::NpcTextures,
            crate::world::npc::NpcTypes,
        ) = world
            .npc_types
            .into_iter()
            .map(|npc_type| {
                (
                    (
                        npc_type.config.identifier,
                        Texture::new(ctx, &npc_type.texture).unwrap(),
                    ),
                    (
                        npc_type.config.identifier,
                        NpcType {
                            sprite: match npc_type.config.sprite {
                                worldlib::character::sprite::SpriteIndexType::Still => {
                                    SpriteIndexes::STILL
                                }
                                worldlib::character::sprite::SpriteIndexType::Walk => {
                                    SpriteIndexes::WALK
                                }
                            },
                            message: npc_type.config.text_color,
                            trainer: npc_type.config.trainer,
                        },
                    ),
                )
            })
            .unzip();

        self.npc_types = types;
        self.textures.npcs.set(textures);

        // self.world_map.add_locations(world.map_gui_locs);

        self.world = world.manager;
    }

    pub fn on_start(
        &mut self,
        ctx: &mut Context,
        player: &mut PlayerCharacter,
        party: &mut Party<GamePokemon>,
    ) {
        self.map_start(ctx, player, true);
        if let Some(map) = self.world.maps.get_mut(&player.location) {
            on_tile(
                map,
                player,
                party,
                &mut self.randoms,
                &mut self.textures,
                &self.sender,
            );
        }
    }

    pub fn map_start(&mut self, ctx: &mut Context, player: &mut PlayerCharacter, music: bool) {
        on_start(ctx, &mut self.world, player, music);
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        player: &mut PlayerCharacter,
        party: &mut Party<GamePokemon>,
        text: &mut TextWindow,
        delta: f32,
    ) {
        // } else if self.world_map.alive() {
        //     self.world_map.update(ctx);
        //     if pressed(ctx, Control::A) {
        //         if let Some(location) = self.world_map.despawn_get() {
        //             self.warp_to_location(location);
        //         }
        //     }
        // Input

        // if !input_lock {

        // if is_debug() {
        //     self.debug_input(ctx, save)
        // }

        if let Some(direction) = self.input.update(player, ctx, delta) {
            if let Some(result) = self.world.try_move(player, direction) {
                match result {
                    TryMoveResult::MapUpdate => self.map_start(ctx, player, true),
                    TryMoveResult::TrySwim => {
                        const SURF: MoveId = unsafe { MoveId::new_unchecked(1718777203) };

                        if party
                            .iter()
                            .flat_map(|pokemon| pokemon.moves.iter())
                            .any(|m| m.0.id == SURF)
                        {
                            player.movement = Movement::Swimming;
                            player.pathing.queue.push(direction);
                        }
                    }
                    TryMoveResult::StartWarpOnTile(tile, coords) => {
                        self.warper.queue(&mut self.world, player, tile, coords);
                    }
                }
            }
        }
        // }

        // Update

        if self.warper.alive() {
            if let Some(music) = self.warper.update(&mut self.world, player, delta) {
                self.map_start(ctx, player, music);
            }
        } else if player.world.warp.is_some() {
            self.warper.spawn();
            player.input_frozen = true;
        }

        if player.do_move(delta) {
            self.stop_player(player, party);
        }

        self.textures.tiles.update(delta);
        self.textures.player.update(delta, player);

        if let Some(map) = self.world.maps.get_mut(&player.location) {
            update1(
                ctx,
                delta,
                map,
                player,
                &self.sender,
                &self.npc_types,
                text,
                &mut self.warper,
                &mut self.randoms,
            );
        }

        self.screen = RenderCoords::new(&player);
    }

    fn stop_player(&mut self, player: &mut PlayerCharacter, party: &mut Party<GamePokemon>) {
        player.stop_move();

        if let Some(map) = self.world.maps.get_mut(&player.location) {
            if let Some(destination) = map.warp_at(player.position.coords) {
                // Warping does not trigger tile actions!
                player.world.warp = Some(*destination);
            } else if map.in_bounds(player.position.coords) {
                on_tile(
                    map,
                    player,
                    party,
                    &mut self.randoms,
                    &mut self.textures,
                    &self.sender,
                );
            }
        }
    }

    // #[deprecated]
    // fn debug_input(&mut self, ctx: &Context, save: &mut PlayerData) {
    //     if is_key_pressed(ctx, Key::F3) {
    //         info!("Local Coordinates: {}", self.player.position.coords);

    //         match self.world.tile(self.player.position.coords) {
    //             Some(tile) => info!("Current Tile ID: {:x}", tile),
    //             None => info!("Currently out of bounds"),
    //         }

    //         info!("Player is {:?}", self.player.movement);
    //     }

    //     if is_key_pressed(ctx, Key::F5) {
    //         if let Some(map) = self.world.get() {
    //             info!("Resetting battled trainers in this map! ({})", map.name);
    //             save.world.get_map(&map.id).battled.clear();
    //         }
    //     }
    // }

    fn warp_to_location(&mut self, player: &mut PlayerCharacter, location: Location) {
        if let Some(map) = self.world.maps.get(&location) {
            info!("Warping to {}", map.name);
            player.location = location;
            let coordinate = if let Some(coord) = map.settings.fly_position {
                coord
            // } else if let Some(coord) = worldlib::character::pathfind::tenth_walkable_coord(map) {
            //     coord
            } else {
                Coordinate::default()
            };

            let pos = &mut player.position;
            pos.coords = coordinate;
            pos.direction = Direction::Down;
        }
    }

    pub fn draw(&self, ctx: &mut Context, player: &PlayerCharacter) {
        let color = if let Some(map) = self.world.get(&player.location) {
            let color = match map.settings.brightness {
                Brightness::Day => Color::WHITE,
                Brightness::Night => Color::rgb(0.6, 0.6, 0.6),
            };

            match &map.chunk {
                Some(chunk) => {


                    super::draw(
                        map,
                        &player.world,
                        ctx,
                        &self.textures,
                        &self.npc_types,
                        &self.screen,
                        true,
                        color,
                    );
                    for (connection, direction, offset) in chunk.connections.iter().flat_map(
                        |(direction, Connection(location, offset))| {
                            self.world
                                .maps
                                .get(location)
                                .map(|map| (map, direction, offset))
                        },
                    ) {
                        fn map_offset(
                            direction: &Direction,
                            current: &WorldMap,
                            map: &WorldMap,
                            offset: i32,
                        ) -> Coordinate {
                            match direction {
                                Direction::Down => Coordinate::new(offset, current.height),
                                Direction::Up => Coordinate::new(offset, -map.height),
                                Direction::Left => Coordinate::new(-map.width, offset),
                                Direction::Right => Coordinate::new(current.width, offset),
                            }
                        }

                        super::draw(
                            map,
                            &player.world,
                            ctx,
                            &self.textures,
                            &self.npc_types,
                            &self
                                .screen
                                .offset(map_offset(direction, map, connection, *offset)),
                            false,
                            color,
                        );
                    }
                }
                None => {
                    super::draw(
                        map,
                        &player.world,
                        ctx,
                        &self.textures,
                        &self.npc_types,
                        &self.screen,
                        true,
                        color,
                    );
                }
            };

            color

        } else {
            Color::WHITE
        };

        self.warper.draw_door(ctx, &self.screen);
        self.textures.player.draw(ctx, player, color);
        self.textures.player.bush.draw(ctx, &self.screen);
        self.warper.draw(ctx);

    }
}

fn on_tile(
    map: &mut WorldMap,
    player: &mut PlayerCharacter,
    party: &mut Party<GamePokemon>,
    randoms: &mut Randoms,
    textures: &mut WorldTextures,
    sender: &Sender<WorldActions>,
) {
    textures.player.bush.in_bush = map.tile(player.position.coords) == Some(0x0D);
    if textures.player.bush.in_bush {
        textures.player.bush.add(player.position.coords);
    }
    // check for wild encounter

    use crate::world::battle::wild_battle;

    if let Some(tile_id) = map.tile(player.position.coords) {
        if player.world.wild.encounters {
            if let Some(wild) = &map.wild {
                if wild.should_encounter(&mut randoms.wild) {
                    if let Some(tiles) = wild.tiles.as_ref() {
                        for tile in tiles.iter() {
                            if &tile_id == tile {
                                sender.send(WorldActions::Battle(wild_battle(&mut randoms.wild, wild)));
                                break;
                            }
                        }
                    } else {
                        sender.send(WorldActions::Battle(wild_battle(&mut randoms.wild, wild)));
                    }
                }
            }
        }

        // look for player

        if player.world.npc.active.is_none() {
            for (index, npc) in map.npcs.iter_mut().filter(|(_, npc)| npc.trainer.is_some()) {
                find_battle(player, &map.id, index, npc);
            }
        }

        // try running scripts

        if player.world.scripts.actions.is_empty() {
            'scripts: for script in map.scripts.iter() {
                use worldlib::script::world::Condition;
                for condition in &script.conditions {
                    match condition {
                        Condition::Location(location) => {
                            if !location.in_bounds(&player.position.coords) {
                                continue 'scripts;
                            }
                        }
                        Condition::Activate(direction) => {
                            if player.position.direction.ne(direction) {
                                continue 'scripts;
                            }
                        }
                        Condition::NoRepeat => {
                            if player.world.scripts.executed.contains(&script.identifier) {
                                continue 'scripts;
                            }
                        }
                        Condition::Script(script, happened) => {
                            if player.world.scripts.executed.contains(script).ne(happened) {
                                continue 'scripts;
                            }
                        }
                        Condition::PlayerHasPokemon(is_true) => {
                            if party.is_empty().eq(is_true) {
                                continue 'scripts;
                            }
                        }
                    }
                }
                player
                    .world
                    .scripts
                    .actions
                    .extend_from_slice(&script.actions);
                player
                    .world
                    .scripts
                    .actions
                    .push(worldlib::script::world::WorldAction::Finish(
                        script.identifier,
                    ));
                player.world.scripts.actions.reverse();
                break;
            }
        }
    }
}

fn on_start(ctx: &mut Context, world: &WorldMapManager, player: &PlayerCharacter, music: bool) {
    if let Some(map) = world.get(&player.location) {
        // if let Some(saves) = get::<PlayerDatas>() {
        //     if let Some(data) = saves.get().world.map.get(&self.name) {
        //         for (index, state) in data.npcs.iter() {
        //             if let Some(npc) = self.NPC_manager.npcs.get_mut(index) {
        //                 // npc.alive = *state;
        //             }
        //         }
        //     }
        // }

        use crate::engine::audio;

        if music {
            if audio::get_current_music(ctx)
                .map(|current| current != &map.music)
                .unwrap_or(true)
            {
                audio::play_music(ctx, &map.music);
            }
        }
    }
}

// fn get_mut(world: &mut WorldMapManager) -> Option<&mut WorldMap> {
//     match world.data.location.as_ref() {
//         Some(cur) => world.maps.get_mut(cur),
//         None => None,
//     }
// }

#[deprecated]
fn update1<'d>(
    ctx: &mut Context,
    delta: f32,
    map: &mut WorldMap,
    player: &mut PlayerCharacter,
    sender: &Sender<WorldActions>,
    npc_types: &NpcTypes,
    window: &mut TextWindow,
    warper: &mut WarpTransition,
    randoms: &mut Randoms,
) {
    if pressed(ctx, Control::A) && player.world.npc.active.is_none() {
        let pos = if map
            .tile(player.position.coords)
            .map(|tile| matches!(tile, 0x298 | 0x2A5))
            .unwrap_or_default()
        {
            player.position.in_direction(player.position.direction)
        } else {
            player.position
        };
        for (id, npc) in map.npcs.iter_mut() {
            if npc.interact.is_some() || npc.trainer.is_some() {
                if npc.interact_from(&pos) {
                    player.world.npc.active = Some(*id);
                }
            }
        }
    }

    // Move Npcs

    for npc in player
        .world
        .scripts
        .npcs
        .values_mut()
        .filter(|(loc, ..)| loc == &map.id)
        .map(|(.., npc)| npc)
    {
        npc.character.do_move(delta);
    }

    for npc in map.npcs.values_mut() {
        npc.character.do_move(delta);
    }

    use worldlib::{character::npc::NpcMovement, positions::Destination};

    match player.world.npc.timer > 0.0 {
        false => {
            player.world.npc.timer += 1.0;

            const NPC_MOVE_CHANCE: f64 = 1.0 / 12.0;

            for (index, npc) in map.npcs.iter_mut() {
                if !npc.character.moving() {
                    if randoms.npc.gen_bool(NPC_MOVE_CHANCE) {
                        match npc.movement {
                            NpcMovement::Still => (),
                            NpcMovement::LookAround => {
                                npc.character.position.direction =
                                    Direction::DIRECTIONS[randoms.npc.gen_range(0..4)];
                                find_battle(player, &map.id, index, npc);
                            }
                            NpcMovement::WalkUpAndDown(steps) => {
                                let origin =
                                    npc.origin.get_or_insert(npc.character.position.coords);
                                let direction = if npc.character.position.coords.y
                                    <= origin.y - steps as i32
                                {
                                    Direction::Down
                                } else if npc.character.position.coords.y >= origin.y + steps as i32
                                {
                                    Direction::Up
                                } else if randoms.npc.gen_bool(0.5) {
                                    Direction::Down
                                } else {
                                    Direction::Up
                                };
                                let coords = npc.character.position.coords.in_direction(direction);
                                if worldlib::map::movement::can_move(
                                    npc.character.movement,
                                    map.movements[npc.character.position.coords.x as usize
                                        + npc.character.position.coords.y as usize
                                            * map.width as usize],
                                ) {
                                    npc.character.position.direction = direction;
                                    if !find_battle(player, &map.id, index, npc) {
                                        if coords.y != player.position.coords.y {
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
            }
        }
        true => player.world.npc.timer -= delta,
    }

    script::update_script(ctx, delta, map, player, sender, npc_types, window, warper);

    // Npc window manager code

    let map_id = map.id;
    // #[deprecated(note = "rewrite active Npc code")]
    if let Some((id, npc)) = player
        .world
        .npc
        .active
        .as_ref()
        .map(|id| map.npcs.get_mut(id).map(|npc| (id, npc)))
        .flatten()
    {
        if window.text.alive() {
            if window.text.finished() {
                if let Some(battle) = crate::world::battle::trainer_battle(
                    npc_types,
                    &mut player.world.battle,
                    &map_id,
                    id,
                    npc,
                ) {
                    sender.send(WorldActions::Battle(battle));
                };
                window.text.despawn();
                player.world.npc.active = None;
                player.unfreeze();
            } else {
                window.text.update(ctx, delta);
            }
        } else {
            if !npc.character.moving() {
                window.text.spawn();
                player.input_frozen = true;

                let mut message_ran = false;

                use worldlib::character::npc::NpcInteract;

                match &npc.interact {
                    NpcInteract::Message(pages) => {
                        window.text.set(
                            pages
                                .iter()
                                .map(|lines| MessagePage {
                                    lines: lines.clone(),
                                    wait: None,
                                })
                                .collect(),
                        );
                        window.text.color(color(
                            npc_types
                                .get(&npc.type_id)
                                .map(|npc| &npc.message)
                                .unwrap_or(&MessageColor::Black),
                        ));
                        message_ran = true;
                    }
                    NpcInteract::Script(_) => todo!(),
                    NpcInteract::Nothing => (),
                }

                if !player.world.battle.battled(&map_id, id) {
                    if let Some(trainer) = npc.trainer.as_ref() {
                        if trainer.battle_on_interact {
                            if let Some(npc_type) = npc_types.get(&npc.type_id) {
                                if let Some(trainer_type) = npc_type.trainer.as_ref() {
                                    // Spawn text window
                                    window.text.set(
                                        trainer
                                            .encounter_message
                                            .iter()
                                            .map(|message| MessagePage {
                                                lines: message.clone(),
                                                wait: None,
                                            })
                                            .collect(),
                                    );
                                    window.text.color(color(&npc_type.message));
                                    message_ran = true;

                                    // Play Trainer music

                                    use crate::engine::audio::{get_current_music, play_music};

                                    if let Some(encounter_music) = trainer_type.music.as_ref() {
                                        if let Some(playing_music) = get_current_music(ctx) {
                                            if playing_music != encounter_music {
                                                play_music(ctx, encounter_music)
                                            }
                                        } else {
                                            play_music(ctx, encounter_music)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                player.position.direction = npc.character.position.direction.inverse();
                if player.frozen() {
                    player.unfreeze();
                }

                if !message_ran {
                    window.text.despawn();
                    player.input_frozen = false;
                    player.world.npc.active = None;
                } else {
                    warn!("process messages 1");
                    // crate::game::text::process_messages(&mut window.text.message.pages, save);
                }
            }
        }
    }
}

fn find_battle(
    player: &mut PlayerCharacter,
    map: &Location,
    id: &worldlib::character::npc::NpcId,
    npc: &mut worldlib::character::npc::Npc,
) -> bool {
    if player.world.npc.active.is_none() {
        if !player.world.battle.battled(map, &id) {
            if npc.find_character(player) {
                player.world.npc.active = Some(*id);
                return true;
            }
        }
    }
    false
}
