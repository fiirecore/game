use crate::{
    deps::str::tinystr8,
    util::{
        Entity, 
        Completable, 
        Direction,
        Coordinate,
    },
    storage::{
        data, data_mut, player::PlayerSave,
    },
    input::{down, pressed, Control},
    pokedex::{
        moves::get_game_move,
        item::{ItemStack, ItemId},
    },
    macroquad::{
        prelude::{
            KeyCode,
            info,
            is_key_pressed,
        }
    },
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

    pub fn new() -> Self {        
        Self {

            map_manager: WorldMapManager::default(),

            textures: WorldTextures::default(),

            warp_transition: WarpTransition::new(),
            start_menu: StartMenu::new(),
            text_window: TextWindow::default(),
            first_direction: Direction::default(),
            render_coords: RenderCoords::default(),
            // noclip_toggle: false,
            player_move_accumulator: 0.0,
        }
    }

    pub fn load(&mut self, world: SerializedWorld) {

        self.textures.setup(world.textures, &world.npc_types);
        
        info!("Finished loading textures!");

        unsafe { crate::world::npc::NPC_TYPES = 
            Some(
                world.npc_types.into_iter().map(|npc_type| {
                    self.textures.npcs.add_npc_type(&npc_type);
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

    pub fn on_start(&mut self, battle: BattleEntryRef) {
        self.map_start(true);
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.on_tile(battle, &mut self.map_manager.player);
        } else {
            self.map_manager.map_set_manager.on_tile(battle, &mut self.map_manager.player);
        }
    }

    pub fn map_start(&mut self, music: bool) {
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.on_start(music);
        } else {
            self.map_manager.map_set_manager.on_start(music);
        }
    }

    pub fn update(&mut self, delta: f32, battle: BattleEntryRef, party: &mut PartyGui, bag: &mut BagGui, action: &mut Option<GameStateAction>) {

        if self.start_menu.alive() {
            self.start_menu.update(delta, party, bag, action);
        } else {

            if pressed(Control::Start) {
                self.start_menu.spawn();
            }

            if is_debug() {
                self.debug_input(battle)
            }

            if !(!self.map_manager.player.character.position.offset.is_zero() || self.map_manager.player.is_frozen() ) {

                if down(Control::B) {
                    if self.map_manager.player.character.move_type == MoveType::Walking {
                        self.map_manager.player.character.move_type = MoveType::Running;
                    }
                } else if self.map_manager.player.character.move_type == MoveType::Running {
                    self.map_manager.player.character.move_type = MoveType::Walking;
                }

                if down(crate::keybind(self.first_direction)) {
                    if self.player_move_accumulator > PLAYER_MOVE_TIME {
                        if let Some(result) = self.map_manager.try_move(self.first_direction, delta) {
                            match result {
                                TryMoveResult::MapUpdate => self.map_start(true),
                                TryMoveResult::TrySwim => {
                                    let surf = tinystr8!("surf");
                                    for id in data().party.iter().map(|pokemon| pokemon.moves.iter().flat_map(|instance| get_game_move(&instance.move_ref.unwrap().id).map(|game_move| game_move.field_id).flatten())).flatten() {
                                        if id == surf {
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
                        if down(keybind(*direction)) {
                            movdir = if let Some(dir) = movdir {
                                if dir.inverse().eq(direction) {
                                    None
                                } else {
                                    Some(*direction)
                                }
                            } else {
                                Some(*direction)
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
                self.map_manager.chunk_map.update(delta, &mut self.map_manager.player, battle, &mut self.map_manager.warp, &mut self.text_window);
            } else {
                self.map_manager.map_set_manager.update(delta, &mut self.map_manager.player, battle, &mut self.map_manager.warp, &mut self.text_window);
            }
    
            if self.warp_transition.alive() {
                self.warp_transition.update(delta);
                if self.warp_transition.switch() {
                    if let Some(destination) = self.map_manager.warp.clone() {
                        self.textures.player.draw = !destination.transition.move_on_exit;
                        let change_music = destination.transition.change_music;
                        self.map_manager.warp(destination);
                        self.map_start(change_music);
                        if self.map_manager.chunk_active {
                            self.map_manager.chunk_map.on_tile(battle, &mut self.map_manager.player);
                        } else {
                            self.map_manager.map_set_manager.on_tile(battle, &mut self.map_manager.player);
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
                    self.stop_player(battle);
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

    pub fn render(&self) {
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.render(&self.textures, self.render_coords, true);
        } else {
            self.map_manager.map_set_manager.render(&self.textures, self.render_coords, true);
        }
        self.textures.player.render(&self.map_manager.player.character);
        self.text_window.render();
        self.start_menu.render(); 
        self.warp_transition.render();
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

    fn stop_player(&mut self, battle: BattleEntryRef) {
        self.map_manager.player.character.stop_move();

        if self.map_manager.chunk_active {
            if let Some(destination) = self.map_manager.chunk_map.check_warp(self.map_manager.player.character.position.coords) { // Warping does not trigger tile actions!
                self.map_manager.warp = Some(destination);
            } else if self.map_manager.chunk_map.in_bounds(self.map_manager.player.character.position.coords) {
                self.map_manager.chunk_map.on_tile(battle, &mut self.map_manager.player);
            }
        } else {
            if let Some(destination) = self.map_manager.map_set_manager.check_warp(self.map_manager.player.character.position.coords) {
                self.map_manager.warp = Some(destination);
            } else if self.map_manager.map_set_manager.in_bounds(self.map_manager.player.character.position.coords) {
                self.map_manager.map_set_manager.on_tile(battle, &mut self.map_manager.player);
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

    fn debug_input(&mut self, battle: BattleEntryRef) {

        if is_key_pressed(KeyCode::F1) {
            random_wild_battle(battle);
        }

        if is_key_pressed(KeyCode::F2) {
            // self.noclip_toggle = true;
            self.map_manager.player.character.noclip = !self.map_manager.player.character.noclip;
        }

        if is_key_pressed(KeyCode::F3) {

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

        if is_key_pressed(KeyCode::F4) {
            for (slot, instance) in data().party.iter().enumerate() {
                info!("Party Slot {}: Lv{} {}", slot, instance.level, instance.name());
            }
        }

        if is_key_pressed(KeyCode::F5) {
            if let Some(map) = if self.map_manager.chunk_active {
                self.map_manager.chunk_map.chunk().map(|chunk| &chunk.map)
            } else {
                self.map_manager.map_set_manager.set().map(|map| map.map()).flatten()
            } {
                info!("Resetting battled trainers in this map! ({})", map.name);
                data_mut().world.get_map(&map.id).battled.clear();
            }
        }

        if is_key_pressed(KeyCode::F6) {
            info!("Clearing used scripts in player data!");
            data_mut().world.scripts.clear();
        }

        if is_key_pressed(KeyCode::F7) {
            self.map_manager.player.character.freeze();
            self.map_manager.player.character.unfreeze();
            self.map_manager.player.character.noclip = true;
            info!("Unfroze player!");
        }

        // F8 in use
        
        if is_key_pressed(KeyCode::F9) {
            use std::sync::atomic::Ordering::Relaxed;
            let wild = !super::WILD_ENCOUNTERS.load(Relaxed);
            super::WILD_ENCOUNTERS.store(wild, Relaxed);
            info!("Wild Encounters: {}", wild);
        }

        if is_key_pressed(KeyCode::H) {
            data_mut().party.iter_mut().for_each(|pokemon| {
                pokemon.current_hp = pokemon.base.hp;
                for pmove in &mut pokemon.moves {
                    pmove.pp = pmove.move_ref.unwrap().pp;
                }
            });
        }

        if is_key_pressed(KeyCode::B) {
            data_mut().bag.add_item(ItemStack::new(&"pokeball".parse::<ItemId>().unwrap(), 50));
        }
        
    }
    
}