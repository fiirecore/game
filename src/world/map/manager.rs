use engine::{
    graphics::byte_texture,
    input::{debug_pressed, pressed, Control, DebugBind},
    tetra::{graphics::Color, Context},
    text::MessagePage,
    util::{Completable, Entity},
    EngineContext,
};
use log::info;
use pokedex::{context::PokedexClientContext, gui::{bag::BagGui, party::PartyGui}, moves::MoveId};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use saves::PlayerData;
use std::rc::Rc;
use worldlib::{
    character::{npc::NpcType, sprite::SpriteIndexes, Movement},
    map::{
        manager::{TryMoveResult, WorldMapManager},
        Brightness, World, WorldMap,
    },
    positions::{Coordinate, Direction, Location},
    serialized::SerializedWorld,
};

use crate::{game::{battle_glue::BattleEntryRef, is_debug}, state::{Action, MainStates}};

use crate::world::{
    gui::{StartMenu, TextWindow},
    map::{input::PlayerInput, texture::WorldTextures, warp::WarpTransition},
    RenderCoords,
};

pub mod command;
pub mod script;

pub struct WorldManager {
    pub world: WorldMapManager,

    textures: WorldTextures,

    menu: StartMenu,
    text: TextWindow,
    warper: WarpTransition,

    input: PlayerInput,

    screen: RenderCoords,

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

impl WorldManager {
    pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
        Self {
            world: WorldMapManager {
                maps: Default::default(),
                data: Default::default(),
            },

            textures: WorldTextures::new(ctx),

            menu: StartMenu::new(party, bag),
            text: TextWindow::new(ctx),
            warper: WarpTransition::new(),

            input: PlayerInput::default(),

            screen: RenderCoords::default(),

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
                        byte_texture(ctx, &npc_type.texture),
                    ),
                    (
                        npc_type.config.identifier,
                        NpcType {
                            sprite: SpriteIndexes::from_index(npc_type.config.sprite),
                            text_color: npc_type.config.text_color,
                            trainer: npc_type.config.trainer,
                        },
                    ),
                )
            })
            .unzip();

        unsafe {
            crate::world::npc::NPC_TYPES = Some(types);
        }
        self.textures.npcs.set(textures);

        // self.world_map.add_locations(world.map_gui_locs);

        self.world = world.manager;
    }

    pub fn load_with_data(&mut self, save: &PlayerData) {
        self.load_player(save);
    }

    pub fn on_start(&mut self, ctx: &mut EngineContext, save: &mut PlayerData, battle: BattleEntryRef) {
        self.map_start(ctx, true);
        on_tile(
            &mut self.world,
            save,
            &mut self.randoms,
            &mut self.textures,
            battle,
        );
    }

    pub fn map_start(&mut self, ctx: &mut EngineContext, music: bool) {
        on_start(&mut self.world, ctx, music);
    }

    pub fn update<'d>(
        &mut self,
        ctx: &mut EngineContext,
        dex: &PokedexClientContext<'d>,
        save: &mut PlayerData<'d>,
        delta: f32,
        input_lock: bool,
        battle: BattleEntryRef,
        action: &mut Option<Action>,
    ) {
        if self.menu.alive() {
            self.menu.update(ctx, dex, save, delta, input_lock, action);
        // } else if self.world_map.alive() {
        //     self.world_map.update(ctx);
        //     if pressed(ctx, Control::A) {
        //         if let Some(location) = self.world_map.despawn_get() {
        //             self.warp_to_location(location);
        //         }
        //     }
        } else {
            // Input

            if !input_lock {
                if pressed(ctx, Control::Start) {
                    self.menu.spawn();
                }

                if is_debug() {
                    self.debug_input(ctx, save)
                }

                if let Some(direction) = self.input.update(&mut self.world.player, ctx, delta) {
                    if let Some(result) = self.world.try_move(direction) {
                        match result {
                            TryMoveResult::MapUpdate => self.map_start(ctx, true),
                            TryMoveResult::TrySwim => {
                                const SURF: MoveId =
                                    unsafe { MoveId::new_unchecked(1718777203) };

                                if save
                                    .party
                                    .iter()
                                    .flat_map(|pokemon| {
                                        pokemon
                                            .moves
                                            .iter()
                                    })
                                    .any(|m| m.0.id == SURF)
                                {
                                        self.world.player.movement = Movement::Swimming;
                                        self.world.player.pathing.queue.push(direction);
                                }
                            }
                            TryMoveResult::StartWarpOnTile(tile, coords) => {
                                self.warper.queue(&mut self.world, tile, coords);
                            }
                        }
                    }
                }
            }

            // Update

            if self.warper.alive() {
                if let Some(music) = self.warper.update(&mut self.world, delta) {
                    self.map_start(ctx, music);
                }
            } else if self.world.warp.is_some() {
                self.warper.spawn();
                self.world.player.input_frozen = true;
            }

            if self.world.player.do_move(delta) {
                self.stop_player(save, battle);
            }

            self.textures.tiles.update(delta);
            self.textures.player.update(delta, &mut self.world.player);

            if let Some(map) = if let Some(loc) = &self.world.data.location {
                self.world.maps.get_mut(loc)
            } else {
                None
            } {
                update1(
                    ctx,
                    dex,
                    save,
                    delta,
                    map,
                    &mut self.world.data,
                    battle,
                    &mut self.text,
                    &mut self.warper,
                    &mut self.randoms,
                );
            }

            self.screen = RenderCoords::new(
                match self.world.get().map(|map| map.chunk.as_ref()).flatten() {
                    Some(chunk) => chunk.coords,
                    None => Coordinate::ZERO,
                },
                &self.world.player,
            );
        }
    }

    pub fn save_data(&self, save: &mut PlayerData) {
        if let Some(location) = self.world.location {
            save.location = location;
        }
        save.character = self.world.data.player.copy();
    }

    fn stop_player(&mut self, save: &mut PlayerData, battle: BattleEntryRef) {
        self.world.player.stop_move();

        if let Some(destination) = self.world.warp_at(self.world.player.position.coords) {
            // Warping does not trigger tile actions!
            self.world.warp = Some(*destination);
        } else if self.world.in_bounds(self.world.player.position.coords) {
            on_tile(
                &mut self.world,
                save,
                &mut self.randoms,
                &mut self.textures,
                battle,
            );
        }
    }

    pub fn load_player(&mut self, save: &PlayerData) {
        *self.world.player = save.character.copy();
        self.world.location = Some(save.location);
    }

    #[deprecated]
    fn debug_input(&mut self, ctx: &Context, save: &mut PlayerData) {
        if debug_pressed(ctx, DebugBind::F3) {
            info!("Local Coordinates: {}", self.world.player.position.coords);

            match self.world.tile(self.world.player.position.coords) {
                Some(tile) => info!("Current Tile ID: {:x}", tile),
                None => info!("Currently out of bounds"),
            }

            info!("Player is {:?}", self.world.player.movement);
        }

        if debug_pressed(ctx, DebugBind::F5) {
            if let Some(map) = self.world.get() {
                info!("Resetting battled trainers in this map! ({})", map.name);
                save.world.get_map(&map.id).battled.clear();
            }
        }
    }

    fn warp_to_location(&mut self, save: &mut PlayerData, location: Location) {
        if let Some(map) = self.world.maps.get(&location) {
            info!("Warping to {}", map.name);
            save.location = location;
            self.world.data.location = Some(location);
            let coordinate = if let Some(coord) = map.settings.fly_position {
                coord
            // } else if let Some(coord) = worldlib::character::pathfind::tenth_walkable_coord(map) {
            //     coord
            } else {
                Coordinate::default()
            };

            let pos = &mut self.world.player.position;
            pos.coords = coordinate;
            pos.direction = Direction::Down;
        }
    }

    pub fn draw<'d>(&self, ctx: &mut EngineContext, dex: &PokedexClientContext<'d>, save: &PlayerData) {
        if self.menu.fullscreen() {
            self.menu.draw(ctx, dex, save);
        // } else if self.world_map.alive() {
        //     self.world_map.draw(ctx);
        } else {
            if let Some(map) = self.world.get() {
                let color = match map.settings.brightness {
                    Brightness::Day => Color::WHITE,
                    Brightness::Night => Color::rgb(0.6, 0.6, 0.6),
                };

                let screen = match &map.chunk {
                    Some(chunk) => {
                        let screen = self.screen.offset(chunk.coords);
                        super::draw(map, &self.world, ctx, &self.textures, &screen, true, color);
                        for map in chunk
                            .connections
                            .iter()
                            .flat_map(|id| self.world.maps.get(id))
                        {
                            if let Some(chunk) = &map.chunk {
                                super::draw(
                                    map,
                                    &self.world,
                                    ctx,
                                    &self.textures,
                                    &self.screen.offset(chunk.coords),
                                    false,
                                    color,
                                );
                            }
                        }
                        screen
                    }
                    None => {
                        super::draw(
                            map,
                            &self.world,
                            ctx,
                            &self.textures,
                            &self.screen,
                            true,
                            color,
                        );
                        self.screen
                    }
                };

                self.warper.draw_door(ctx, &screen);
                self.textures.player.draw(ctx, &self.world.player, color);
                self.textures.player.bush.draw(ctx, &screen);
                self.warper.draw(ctx);
            }

            self.text.draw(ctx);
            self.menu.draw(ctx, dex, save);
        }
    }
}

fn on_tile(
    world: &mut WorldMapManager,
    save: &mut PlayerData,
    randoms: &mut Randoms,
    textures: &mut WorldTextures,
    battle: BattleEntryRef,
) {
    textures.player.bush.in_bush = world.tile(world.player.position.coords) == Some(0x0D);
    if textures.player.bush.in_bush {
        textures.player.bush.add(world.player.position.coords);
    }
    if let Some(map) = if let Some(loc) = &world.data.location {
        world.maps.get_mut(&loc)
    } else {
        None
    } {
        let world = &mut world.data;
        // check for wild encounter

        use crate::world::battle::wild_battle;

        if let Some(tile_id) = map.tile(world.player.position.coords) {
            if world.wild.encounters {
                if let Some(wild) = &map.wild {
                    if wild.should_encounter(&mut randoms.wild) {
                        if let Some(tiles) = wild.tiles.as_ref() {
                            for tile in tiles.iter() {
                                if &tile_id == tile {
                                    wild_battle(&mut randoms.wild, battle, wild);
                                    break;
                                }
                            }
                        } else {
                            wild_battle(&mut randoms.wild, battle, wild);
                        }
                    }
                }
            }

            // look for player

            if world.npc.active.is_none() {
                for (index, npc) in map.npcs.iter_mut().filter(|(_, npc)| npc.trainer.is_some()) {
                    find_battle(
                        save,
                        &map.id,
                        index,
                        npc,
                        &mut world.npc.active,
                        &mut world.player,
                    );
                }
            }

            // try running scripts

            if world.script.actions.is_empty() {
                'scripts: for script in map.scripts.iter() {
                    use worldlib::script::world::Condition;
                    for condition in &script.conditions {
                        match condition {
                            Condition::Location(location) => {
                                if !location.in_bounds(&world.player.position.coords) {
                                    continue 'scripts;
                                }
                            }
                            Condition::Activate(direction) => {
                                if world.player.position.direction.ne(direction) {
                                    continue 'scripts;
                                }
                            }
                            Condition::NoRepeat => {
                                if save.world.scripts.contains(&script.identifier) {
                                    continue 'scripts;
                                }
                            }
                            Condition::Script(script, happened) => {
                                if save.world.scripts.contains(script).ne(happened) {
                                    continue 'scripts;
                                }
                            }
                            Condition::PlayerHasPokemon(is_true) => {
                                if save.party.is_empty().eq(is_true) {
                                    continue 'scripts;
                                }
                            }
                        }
                    }
                    world.script.actions.extend_from_slice(&script.actions);
                    world
                        .script
                        .actions
                        .push(worldlib::script::world::WorldAction::Finish(
                            script.identifier,
                        ));
                    world.script.actions.reverse();
                    break;
                }
            }
        }
    }
}

fn on_start(world: &mut WorldMapManager, ctx: &mut EngineContext, music: bool) {
    if let Some(map) = get_mut(world) {
        // if let Some(saves) = get::<PlayerDatas>() {
        //     if let Some(data) = saves.get().world.map.get(&self.name) {
        //         for (index, state) in data.npcs.iter() {
        //             if let Some(npc) = self.NPC_manager.npcs.get_mut(index) {
        //                 // npc.alive = *state;
        //             }
        //         }
        //     }
        // }

        if music {
            if engine::audio::music::get_current_music(ctx)
                .map(|current| current != map.music)
                .unwrap_or(true)
            {
                engine::audio::play_music(ctx, map.music);
            }
        }
    }
}

fn get_mut(world: &mut WorldMapManager) -> Option<&mut WorldMap> {
    match world.data.location.as_ref() {
        Some(cur) => world.maps.get_mut(cur),
        None => None,
    }
}

#[deprecated]
fn update1<'d>(
    ctx: &mut EngineContext,
    dex: &PokedexClientContext<'d>,
    save: &mut PlayerData<'d>,
    delta: f32,
    map: &mut WorldMap,
    world: &mut worldlib::map::manager::WorldMapData,
    battle: BattleEntryRef,
    window: &mut TextWindow,
    warper: &mut WarpTransition,
    randoms: &mut Randoms,
) {
    if is_debug() {
        debug_input(ctx, map);
    }

    if pressed(ctx, Control::A) && world.npc.active.is_none() {
        let pos = if map
            .tile(world.player.position.coords)
            .map(|tile| matches!(tile, 0x298 | 0x2A5))
            .unwrap_or_default()
        {
            world
                .player
                .position
                .in_direction(world.player.position.direction)
        } else {
            world.player.position
        };
        for (id, npc) in map.npcs.iter_mut() {
            if npc.interact.is_some() || npc.trainer.is_some() {
                if npc.interact_from(&pos) {
                    world.npc.active = Some(*id);
                }
            }
        }
    }

    // Move Npcs

    for npc in world
        .script
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

    match world.npc.timer > 0.0 {
        false => {
            world.npc.timer += 1.0;

            const NPC_MOVE_CHANCE: f64 = 1.0 / 12.0;

            for (index, npc) in map.npcs.iter_mut() {
                if !npc.character.moving() {
                    if randoms.npc.gen_bool(NPC_MOVE_CHANCE) {
                        match npc.movement {
                            NpcMovement::Still => (),
                            NpcMovement::LookAround => {
                                npc.character.position.direction =
                                    Direction::DIRECTIONS[randoms.npc.gen_range(0..4)];
                                find_battle(
                                    save,
                                    &map.id,
                                    index,
                                    npc,
                                    &mut world.npc.active,
                                    &mut world.player,
                                );
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
                                if worldlib::map::can_move(
                                    npc.character.movement,
                                    map.movements[npc.character.position.coords.x as usize
                                        + npc.character.position.coords.y as usize * map.width],
                                ) {
                                    npc.character.position.direction = direction;
                                    if !find_battle(
                                        save,
                                        &map.id,
                                        index,
                                        npc,
                                        &mut world.npc.active,
                                        &mut world.player,
                                    ) {
                                        if coords.y != world.player.position.coords.y {
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
        true => world.npc.timer -= delta,
    }

    script::update_script(ctx, dex, save, delta, map, world, battle, window, warper);

    // Npc window manager code

    let map_id = map.id;
    // #[deprecated(note = "rewrite active Npc code")]
    if let Some((id, npc)) = world
        .npc
        .active
        .as_ref()
        .map(|id| map.npcs.get_mut(id).map(|npc| (id, npc)))
        .flatten()
    {
        if window.text.alive() {
            if window.text.finished() {
                crate::world::battle::trainer_battle(save, battle, &mut world.battling, npc, &map_id, id);
                window.text.despawn();
                world.npc.active = None;
                world.player.unfreeze();
            } else {
                window.text.update(ctx, delta);
            }
        } else {
            if !npc.character.moving() {
                window.text.spawn();
                world.player.input_frozen = true;

                let mut message_ran = false;

                use worldlib::character::npc::NpcInteract;

                match &npc.interact {
                    NpcInteract::Message(pages) => {
                        window.text.set(pages.clone());
                        window
                            .text
                            .color(crate::world::npc::npc_type(&npc.type_id).text_color);
                        message_ran = true;
                    }
                    NpcInteract::Script(_) => todo!(),
                    NpcInteract::Nothing => (),
                }

                if !save.world.get_map(&map_id).battled.contains(id) {
                    if let Some(trainer) = npc.trainer.as_ref() {
                        if trainer.battle_on_interact {
                            let npc_type = crate::world::npc::npc_type(&npc.type_id);
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
                                window.text.color(npc_type.text_color);
                                message_ran = true;

                                // Play Trainer music

                                use engine::audio::{
                                    music::{get_current_music, get_music_id},
                                    play_music, play_music_named,
                                };

                                if let Some(encounter_music) = trainer_type.music.as_ref() {
                                    if let Some(playing_music) = get_current_music(ctx) {
                                        if let Some(music) = get_music_id(ctx, encounter_music) {
                                            if playing_music != music {
                                                play_music(ctx, music)
                                            }
                                        }
                                    } else {
                                        play_music_named(ctx, encounter_music)
                                    }
                                }
                            }
                        }
                    }
                }

                world.player.position.direction = npc.character.position.direction.inverse();
                if world.player.frozen() {
                    world.player.unfreeze();
                }

                if !message_ran {
                    window.text.despawn();
                    world.player.input_frozen = false;
                    world.npc.active = None;
                } else {
                    crate::game::text::process_messages(&mut window.text.message.pages, save);
                }
            }
        }
    }
}

fn debug_input(ctx: &Context, map: &mut WorldMap) {
    if debug_pressed(ctx, DebugBind::F8) {
        for (index, npc) in map.npcs.iter() {
            info!(
                "Npc {} (id: {}), is at {}, {}; looking {:?}",
                &npc.name,
                index,
                /*if npc.alive() {""} else {" (despawned)"},*/
                &npc.character.position.coords.x,
                &npc.character.position.coords.y,
                &npc.character.position.direction
            );
        }
    }
}

fn find_battle(
    save: &mut PlayerData,
    map: &Location,
    id: &worldlib::character::npc::NpcId,
    npc: &mut worldlib::character::npc::Npc,
    active: &mut Option<worldlib::character::npc::NpcId>,
    player: &mut worldlib::character::player::PlayerCharacter,
) -> bool {
    if active.is_none() {
        if !save.world.has_battled(map, &id) {
            if npc.find_character(player) {
                *active = Some(*id);
                return true;
            }
        }
    }
    false
}
