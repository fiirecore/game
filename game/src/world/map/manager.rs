use std::rc::Rc;

use crate::{
    util::{
        Entity, 
        Completable, 
        Direction,
        Coordinate,
    },
    storage::{
        data, data_mut, player::PlayerSave,
    },
    input::{
        down, pressed, Control,
        debug_pressed, DebugBind,
    },
    pokedex::{
        moves::FieldMoveId,
        item::{ItemStack, ItemId},
    },
    tetra::Context,
    log::info,
    battle_glue::BattleEntryRef,
    gui::{
        party::PartyGui,
        bag::BagGui,
    },
    state::GameStateAction,
    is_debug, keybind,
};

use worldlib::{
    serialized::SerializedWorld,
    map::{
        World,
        manager::{WorldMapManager, TryMoveResult},
    },
    character::{
        MoveType,
        npc::npc_type::NPCType,
    },
};

use crate::world::{
    GameWorld, 
    WorldTextures,
    RenderCoords,
    gui::{
        TextWindow,
        StartMenu,
    },
    battle::random_wild_battle,
};

use super::warp::WarpTransition;

const PLAYER_MOVE_TIME: f32 = 0.12;

pub struct WorldManager {

    pub map_manager: WorldMapManager,

    textures: WorldTextures,

    warp_transition: WarpTransition,

    start_menu: StartMenu,
    text_window: TextWindow,

    // Other

    pub render_coords: RenderCoords,
    // noclip_toggle: bool,

    first_direction: Direction,
    player_move_accumulator: f32,

}

impl WorldManager {

    pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {        
        Self {

            map_manager: WorldMapManager::default(),

            textures: WorldTextures::new(ctx),

            warp_transition: WarpTransition::new(),
            start_menu: StartMenu::new(ctx, party, bag),
            text_window: TextWindow::new(ctx),
            first_direction: Direction::default(),
            render_coords: RenderCoords::default(),
            // noclip_toggle: false,
            player_move_accumulator: 0.0,
        }
    }

    pub fn load(&mut self, ctx: &mut Context, world: SerializedWorld) {

        self.textures.setup(ctx, world.textures, &world.npc_types);
        
        info!("Finished loading textures!");

        unsafe { crate::world::npc::NPC_TYPES = 
            Some(
                world.npc_types.into_iter().map(|npc_type| {
                    self.textures.npcs.add_npc_type(ctx, &npc_type);
                    (
                        npc_type.config.identifier,
                        NPCType {
                            sprite: firecore_world_lib::character::sprite::SpriteIndexes::from_index(npc_type.config.sprite),
                            text_color: npc_type.config.text_color,
                            trainer: npc_type.config.trainer,
                        }
                    )
                }
                ).collect()
            ); 
        }

        self.map_manager = world.manager;
    
    }

    pub fn load_with_data(&mut self) {
        self.load_player(data());
    }

    pub fn on_start(&mut self, ctx: &mut Context, battle: BattleEntryRef) {
        self.map_start(ctx, true);
        if self.map_manager.chunk_active {
            on_tile(ctx, &mut self.textures, &mut self.map_manager.chunk_map, battle, &mut self.map_manager.player);
        } else {
            on_tile(ctx, &mut self.textures, &mut self.map_manager.map_set_manager, battle, &mut self.map_manager.player);
        }
    }

    pub fn map_start(&mut self, ctx: &mut Context, music: bool) {
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.on_start(ctx, music);
        } else {
            self.map_manager.map_set_manager.on_start(ctx, music);
        }
    }

    pub fn update(&mut self, ctx: &mut Context, delta: f32, battle: BattleEntryRef, action: &mut Option<GameStateAction>) {

        if self.start_menu.alive() {
            self.start_menu.update(ctx, delta, action);
        } else {

            if pressed(ctx, Control::Start) {
                self.start_menu.spawn();
            }

            if is_debug() {
                self.debug_input(ctx, battle)
            }

            if !(!self.map_manager.player.character.position.offset.is_zero() || self.map_manager.player.is_frozen() ) {

                if down(ctx, Control::B) {
                    if self.map_manager.player.character.move_type == MoveType::Walking {
                        self.map_manager.player.character.move_type = MoveType::Running;
                    }
                } else if self.map_manager.player.character.move_type == MoveType::Running {
                    self.map_manager.player.character.move_type = MoveType::Walking;
                }

                const SURF: FieldMoveId = unsafe { FieldMoveId::new_unchecked(1718777203) };

                if down(ctx, crate::keybind(self.first_direction)) {
                    if self.player_move_accumulator > PLAYER_MOVE_TIME {
                        if let Some(result) = self.map_manager.try_move(self.first_direction, delta) {
                            match result {
                                TryMoveResult::MapUpdate => self.map_start(ctx, true),
                                TryMoveResult::TrySwim => {
                                    for id in data().party.iter().map(|pokemon| pokemon.moves.iter().flat_map(|instance| &instance.move_ref.unwrap().field_id)).flatten() {
                                        if id == &SURF {
                                            self.map_manager.player.character.move_type = MoveType::Swimming;
                                            self.map_manager.try_move(self.first_direction, delta);
                                            break;
                                        }
                                    }
                                    // self.text_window.set_text(firecore_game::text::Message::new(firecore_game::text::TextColor::Black, vec![]));
                                }
                            }
                        }
                    } else {
                        self.player_move_accumulator += delta;
                    }
                } else {
                    let mut movdir: Option<Direction> = None;
                    for direction in &Direction::DIRECTIONS {
                        let direction = *direction;
                        if down(ctx, keybind(direction)) {
                            movdir = if let Some(dir) = movdir {
                                if dir.inverse() == direction {
                                    None
                                } else {
                                    Some(direction)
                                }
                            } else {
                                Some(direction)
                            };
                        }                        
                    }
                    if let Some(direction) = movdir {
                        self.map_manager.player.character.position.direction = direction;
                        self.first_direction = direction;
                    } else {
                        self.player_move_accumulator = 0.0;
                        self.map_manager.player.character.moving = false;
                    }
                }
            }

            // Update

            self.textures.tiles.update(delta);
            self.textures.player.update(delta, &mut self.map_manager.player.character);
    
            if self.map_manager.chunk_active {
                self.map_manager.chunk_map.update(ctx, delta, &mut self.map_manager.player, battle, &mut self.map_manager.warp, &mut self.text_window);
            } else {
                self.map_manager.map_set_manager.update(ctx, delta, &mut self.map_manager.player, battle, &mut self.map_manager.warp, &mut self.text_window);
            }
    
            if self.warp_transition.alive() {
                self.warp_transition.update(delta);
                if self.warp_transition.switch() {
                    if let Some(destination) = self.map_manager.warp.clone() {
                        self.textures.player.draw = !destination.transition.move_on_exit;
                        let change_music = destination.transition.change_music;
                        self.map_manager.warp(destination);
                        self.map_start(ctx, change_music);
                        if self.map_manager.chunk_active {
                            on_tile(ctx, &mut self.textures, &mut self.map_manager.chunk_map, battle, &mut self.map_manager.player);
                        } else {
                            on_tile(ctx, &mut self.textures, &mut self.map_manager.map_set_manager, battle, &mut self.map_manager.player);
                        }
                    }
                }
                if self.warp_transition.finished() {
                    self.textures.player.draw = true;
                    self.warp_transition.despawn();
                    self.map_manager.player.unfreeze();
                    if let Some(destination) = self.map_manager.warp.take() {
                        if destination.transition.move_on_exit {
                            self.map_manager.try_move(destination.position.direction.unwrap_or(self.map_manager.player.character.position.direction), delta);
                        }
                    }
                    
                }
            } else {
                if self.map_manager.warp.is_some() {
                    self.warp_transition.spawn();
                    self.map_manager.player.freeze_input();
                }
            }
    
            if !self.map_manager.player.is_frozen() {
                if self.map_manager.player.do_move(delta) {
                    self.stop_player(ctx, battle);
                }
            }
    
            let offset = if self.map_manager.chunk_active {
                self.map_manager.chunk_map.chunk().map(|chunk| chunk.coords)
            } else {
                None
            }.unwrap_or(Coordinate { x: 0, y: 0 });
    
            self.render_coords = RenderCoords::new(offset, &self.map_manager.player.character);

        }
        
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.draw(ctx, &self.textures, &self.render_coords, true);
        } else {
            self.map_manager.map_set_manager.draw(ctx, &self.textures, &self.render_coords, true);
        }
        self.textures.player.draw(ctx, &self.map_manager.player.character);

        {
            let coords = if self.map_manager.chunk_active {
                if let Some(coords) = self.map_manager.chunk_map.chunk().map(|chunk| chunk.coords) {
                    self.render_coords.offset(coords)
                } else {
                    self.render_coords
                }
            } else {
                self.render_coords
            };
            self.textures.player.bush.draw(ctx, &coords);
        }

        self.text_window.draw(ctx);
        self.start_menu.draw(ctx); 
        self.warp_transition.draw(ctx);
    }

    pub fn save_data(&self, save: &mut PlayerSave) {
        use crate::storage::player;
        save.location.map = (!self.map_manager.chunk_active).then(|| self.map_manager.map_set_manager.current.unwrap_or(player::default_map()));
        save.location.index = if self.map_manager.chunk_active {
            self.map_manager.chunk_map.current.unwrap_or(player::default_index())
        } else {
            self.map_manager.map_set_manager.set().map(|map| map.current).flatten().unwrap_or(player::default_index())
		};
		save.character = self.map_manager.player.character.clone();
    }

    fn stop_player(&mut self, ctx: &mut Context, battle: BattleEntryRef) {
        self.map_manager.player.character.stop_move();

        if self.map_manager.chunk_active {
            if let Some(destination) = self.map_manager.chunk_map.check_warp(self.map_manager.player.character.position.coords) { // Warping does not trigger tile actions!
                self.map_manager.warp = Some(destination);
            } else if self.map_manager.chunk_map.in_bounds(self.map_manager.player.character.position.coords) {
                on_tile(ctx, &mut self.textures, &mut self.map_manager.chunk_map, battle, &mut self.map_manager.player);
            }
        } else {
            if let Some(destination) = self.map_manager.map_set_manager.check_warp(self.map_manager.player.character.position.coords) {
                self.map_manager.warp = Some(destination);
            } else if self.map_manager.map_set_manager.in_bounds(self.map_manager.player.character.position.coords) {
                on_tile(ctx, &mut self.textures, &mut self.map_manager.map_set_manager, battle, &mut self.map_manager.player);
            }
        }        
    }

    pub fn load_player(&mut self, data: &PlayerSave) {

        self.map_manager.player.character = data.character.clone();

        if let Some(map) = data.location.map {
            self.map_manager.chunk_active = false;
            self.map_manager.update_map_set(map, data.location.index);
        } else {
            self.map_manager.chunk_active = true;
            self.map_manager.update_chunk(data.location.index);
        }     
    }

    fn debug_input(&mut self, ctx: &Context, battle: BattleEntryRef) {

        if debug_pressed(ctx, DebugBind::F1) {
            random_wild_battle(battle);
        }

        if debug_pressed(ctx, DebugBind::F2) {
            // self.noclip_toggle = true;
            self.map_manager.player.character.noclip = !self.map_manager.player.character.noclip;
        }

        if debug_pressed(ctx, DebugBind::F3) {

            info!("Local Coordinates: {}", self.map_manager.player.character.position.coords);
            // info!("Global Coordinates: ({}, {})", self.map_manager.player.position.get_x(), self.map_manager.player.position.get_y());

            if let Some(tile) = if self.map_manager.chunk_active {
                self.map_manager.chunk_map.tile(self.map_manager.player.character.position.coords)
            } else {
                self.map_manager.map_set_manager.tile(self.map_manager.player.character.position.coords)
            } {
                info!("Current Tile ID: {:x}", tile);
            } else {
                info!("Currently out of bounds");
            }

            info!("Player is {:?}", self.map_manager.player.character.move_type);
            
        }

        if debug_pressed(ctx, DebugBind::F4) {
            for (slot, instance) in data().party.iter().enumerate() {
                info!("Party Slot {}: Lv{} {}", slot, instance.level, instance.name());
            }
        }

        if debug_pressed(ctx, DebugBind::F5) {
            if let Some(map) = if self.map_manager.chunk_active {
                self.map_manager.chunk_map.chunk().map(|chunk| &chunk.map)
            } else {
                self.map_manager.map_set_manager.set().map(|map| map.map()).flatten()
            } {
                info!("Resetting battled trainers in this map! ({})", map.name);
                data_mut().world.get_map(&map.id).battled.clear();
            }
        }

        if debug_pressed(ctx, DebugBind::F6) {
            info!("Clearing used scripts in player data!");
            data_mut().world.scripts.clear();
        }

        if debug_pressed(ctx, DebugBind::F7) {
            self.map_manager.player.character.freeze();
            self.map_manager.player.character.unfreeze();
            self.map_manager.player.character.noclip = true;
            info!("Unfroze player!");
        }

        // F8 in use
        
        if debug_pressed(ctx, DebugBind::F9) {
            use std::sync::atomic::Ordering::Relaxed;
            let wild = !super::WILD_ENCOUNTERS.load(Relaxed);
            super::WILD_ENCOUNTERS.store(wild, Relaxed);
            info!("Wild Encounters: {}", wild);
        }

        if debug_pressed(ctx, DebugBind::H) {
            data_mut().party.iter_mut().for_each(|pokemon| {
                pokemon.heal();
            });
        }

        if debug_pressed(ctx, DebugBind::B) {
            data_mut().bag.add_item(ItemStack::new(&"pokeball".parse::<ItemId>().unwrap(), 50));
        }
        
    }
    
}

fn on_tile(ctx: &mut Context, textures: &mut WorldTextures, map: &mut impl GameWorld, battle: BattleEntryRef, player: &mut firecore_world_lib::character::player::PlayerCharacter) {
    textures.player.bush.in_bush = map.tile(player.character.position.coords) == Some(0x0D);
    if textures.player.bush.in_bush {
        textures.player.bush.add(player.character.position.coords);
    }
    map.on_tile(ctx, battle, player)
}