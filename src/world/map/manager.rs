use firecore_util::Completable;
use firecore_world::map::manager::WorldMapManager;
use firecore_world::map::World;
use macroquad::prelude::KeyCode;
use macroquad::prelude::collections::storage::{get, get_mut};
use macroquad::prelude::info;
use macroquad::prelude::is_key_pressed;
use firecore_input::{self as input, Control};
use firecore_util::Entity;
use firecore_world::character::Character;

use crate::battle::data::BattleData;
use crate::battle::manager::BattleManager;
use firecore_data::player::{PlayerSave, PlayerSaves};
use crate::gui::game::party::PokemonPartyGui;
use crate::scene::scenes::SceneState;
use crate::util::graphics::byte_texture;
use crate::util::pokemon::PokemonTextures;
use crate::world::NPCTypes;
use crate::world::{GameWorld, TileTextures, NpcTextures, GuiTextures, RenderCoords};
use crate::world::gui::text_window::TextWindow;
use crate::world::gui::start_menu::StartMenu;
use crate::world::player::PlayerTexture;
use crate::world::warp_transition::WarpTransition;

pub struct WorldManager {

    pub map_manager: WorldMapManager,

    npc_types: NPCTypes,
    tile_textures: TileTextures,
    npc_textures: NpcTextures,
    gui_textures: GuiTextures,
    player_texture: PlayerTexture,

    warp_transition: WarpTransition,

    start_menu: StartMenu,
    text_window: TextWindow,

    // Other

    noclip_toggle: bool,
    

}

impl WorldManager {

    pub fn new() -> Self {        
        Self {

            map_manager: WorldMapManager::default(),

            npc_types: NPCTypes::new(),
            tile_textures: TileTextures::new(),
            npc_textures: NpcTextures::new(),
            gui_textures: GuiTextures::new(),
            player_texture: PlayerTexture::default(),

            warp_transition: WarpTransition::new(),
            start_menu: StartMenu::new(),
            text_window: TextWindow::default(),
            noclip_toggle: false,
        }
    }

    pub async fn load(&mut self, battle_manager: &mut BattleManager) {
        self.tile_textures.setup();
        self.map_manager = crate::data::map::load_maps(battle_manager, &mut self.tile_textures, &mut self.npc_textures, &mut self.npc_types).await;
        self.gui_textures.insert(0, byte_texture(include_bytes!("../../../build/assets/condition.png")));
    }

    pub fn on_start(&mut self) {
        self.load_player();
        self.map_start(true);
    }

    pub fn map_start(&mut self, music: bool) {
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.on_start(music);
        } else {
            self.map_manager.map_set_manager.on_start(music);
        }
    }

    pub fn update(&mut self, delta: f32, battle_data: &mut Option<BattleData>) {
        self.tile_textures.update(delta);
        
        if self.noclip_toggle && self.map_manager.player.position.local.offset.is_none() {
            self.noclip_toggle = false;
            self.map_manager.player.properties.noclip = !self.map_manager.player.properties.noclip;
            if self.map_manager.player.properties.noclip {
                self.map_manager.player.properties.speed = self.map_manager.player.properties.base_speed * 4.0;
            } else {
                self.map_manager.player.properties.speed = self.map_manager.player.properties.base_speed;
            }
            info!("No clip toggled!");
        }

        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.update(delta, &mut self.map_manager.player, battle_data, &mut self.map_manager.warp, &mut self.text_window, &self.npc_types);
        } else {
            self.map_manager.map_set_manager.update(delta, &mut self.map_manager.player, battle_data, &mut self.map_manager.warp, &mut self.text_window, &self.npc_types);
        }

        if self.warp_transition.is_alive() {
            self.warp_transition.update(delta);
            if self.warp_transition.switch() && !self.warp_transition.recognize_switch {
                self.warp_transition.recognize_switch = true;
                if let Some((destination, music)) = self.map_manager.warp.clone() {
                    self.player_texture.draw = !destination.transition.move_on_exit;
                    self.map_manager.warp(destination);
                    self.map_start(music);
                    if self.map_manager.chunk_active {
                        self.map_manager.chunk_map.on_tile(battle_data, &mut self.map_manager.player);
                    } else {
                        self.map_manager.map_set_manager.on_tile(battle_data, &mut self.map_manager.player);
                    }
                }
            }
            if self.warp_transition.is_finished() {
                self.player_texture.draw = true;
                self.warp_transition.despawn();
                self.map_manager.player.unfreeze();
                if let Some((destination, _)) = self.map_manager.warp.take() {
                    if destination.transition.move_on_exit {
                        self.map_manager.try_move(destination.position.direction.unwrap_or(self.map_manager.player.position.local.direction), delta);
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
                self.stop_player(battle_data);
            }
        }
        
    }

    pub fn render(&self) {
        let coords = RenderCoords::new(&self.map_manager.player);
        if self.map_manager.chunk_active {
            self.map_manager.chunk_map.render(&self.tile_textures, &self.npc_textures, &self.npc_types, &self.gui_textures, coords, true);
        } else {
            self.map_manager.map_set_manager.render(&self.tile_textures, &self.npc_textures, &self.npc_types, &self.gui_textures, coords, true);
        }
        self.player_texture.render(&self.map_manager.player);
        self.text_window.render();
        self.start_menu.render(); 
        self.warp_transition.render();
    }

    pub fn save_data(&self, player_data: &mut PlayerSave) {
        if self.map_manager.chunk_active {
            player_data.location.map = None;
            player_data.location.index = self.map_manager.chunk_map.current.unwrap_or(firecore_data::player::default_index());
        } else {
            player_data.location.map = Some(self.map_manager.map_set_manager.current.unwrap_or(firecore_data::player::default_map()));
            player_data.location.index = self.map_manager.map_set_manager.set().map(|map| map.current).flatten().unwrap_or(firecore_data::player::default_index());
		}
		player_data.location.position = self.map_manager.player.position;
    }

    pub fn input(&mut self, delta: f32, battle_data: &mut Option<BattleData>, party_gui: &mut PokemonPartyGui, textures: &PokemonTextures, scene_state: &mut SceneState) {

        if crate::debug() {
            self.debug_input(battle_data)
        }

        if input::pressed(Control::Start) {
            self.start_menu.toggle();
        }

        if self.text_window.is_alive() {
            self.text_window.input();
        } else if self.start_menu.is_alive() {
            self.start_menu.input(scene_state, party_gui, textures);
        } else {

            if self.map_manager.chunk_active {
                self.map_manager.chunk_map.input(delta, &mut self.map_manager.player);
            } else {
                self.map_manager.map_set_manager.input(delta, &mut self.map_manager.player);
            }
    
            if self.map_manager.player.position.local.offset.is_none() && !self.map_manager.player.is_frozen() {
                self.map_manager.player.properties.moving = true;

                if input::down(Control::B) {
                    if !self.map_manager.player.properties.running {
                        self.map_manager.player.properties.running = true;
                        self.map_manager.player.properties.speed = 
                            ((self.map_manager.player.properties.base_speed as u8) << (
                                if self.map_manager.player.properties.noclip {
                                    2
                                } else {
                                    1
                                }
                            )) as f32;
                    }
                } else if self.map_manager.player.properties.running {
                    self.map_manager.player.properties.running = false;
                    self.map_manager.player.properties.reset_speed();
                }

                if !input::down(crate::util::keybind(self.map_manager.player.position.local.direction)) {
                    for direction in &firecore_util::Direction::DIRECTIONS {
                        let direction = *direction;
                        if input::down(crate::util::keybind(direction)) {
                            if self.map_manager.try_move(direction, delta) {
                                self.map_start(true);
                            }
                            break;
                        }
                    }
                } else if input::down(crate::util::keybind(self.map_manager.player.position.local.direction)) {
                    if self.map_manager.try_move(self.map_manager.player.position.local.direction, delta) {
                        self.map_start(true);
                    }
                } else {
                    self.map_manager.player.properties.moving = false;
                    self.map_manager.player.properties.running = false;
                }

            }

        }
        
    }

    fn stop_player(&mut self, battle_data: &mut Option<BattleData>) {
        self.map_manager.player.stop_move();

        // if self.map_manager.chunk_active {
        //     self.map_manager.ch
        // }

        if self.map_manager.chunk_active {
            if let Some(destination) = self.map_manager.chunk_map.check_warp(self.map_manager.player.position.local.coords) { // Warping does not trigger tile actions!
                self.map_manager.warp = Some((destination, true));
            } else if self.map_manager.chunk_map.in_bounds(self.map_manager.player.position.local.coords) {
                self.map_manager.chunk_map.on_tile(battle_data, &mut self.map_manager.player);
            }
        } else {
            if let Some(destination) = self.map_manager.map_set_manager.check_warp(self.map_manager.player.position.local.coords) {
                self.map_manager.warp = Some((destination, true));
            } else if self.map_manager.map_set_manager.in_bounds(self.map_manager.player.position.local.coords) {
                self.map_manager.map_set_manager.on_tile(battle_data, &mut self.map_manager.player);
            }
        }        
    }

    pub fn load_player(&mut self) {
        if let Some(player_saves) = macroquad::prelude::collections::storage::get::<PlayerSaves>() {
            let data = player_saves.get();
            let location = &data.location;
            self.map_manager.player.position = location.position;
            self.player_texture.walking_texture = Some(byte_texture(include_bytes!("../../../build/assets/player/walking.png")));
            self.player_texture.running_texture = Some(byte_texture(include_bytes!("../../../build/assets/player/running.png")));

            if let Some(map) = location.map {
                self.map_manager.chunk_active = false;
                self.map_manager.update_map_set(map, location.index);
            } else {
                self.map_manager.chunk_active = true;
                self.map_manager.update_chunk(location.index);
            }
        }        
    }

    fn debug_input(&mut self, battle_data: &mut Option<BattleData>) {

        if is_key_pressed(KeyCode::F1) {
            crate::battle::data::random_wild_battle(battle_data);
        }

        if is_key_pressed(KeyCode::F2) {
            self.noclip_toggle = true;
        }

        if is_key_pressed(KeyCode::F3) {

            info!("Local Coordinates: {}", self.map_manager.player.position.local.coords);
            info!("Global Coordinates: ({}, {})", self.map_manager.player.position.get_x(), self.map_manager.player.position.get_y());

            if let Some(tile) = if self.map_manager.chunk_active {
                self.map_manager.chunk_map.tile(self.map_manager.player.position.local.coords)
            } else {
                self.map_manager.map_set_manager.tile(self.map_manager.player.position.local.coords)
            } {
                info!("Current Tile ID: {}", tile);
            } else {
                info!("Currently out of bounds");
            }
            
        }

        if is_key_pressed(KeyCode::F4) {
            if let Some(saves) = get::<PlayerSaves>() {
                for (slot, instance) in saves.get().party.iter().enumerate() {
                    if let Some(pokemon) = firecore_pokedex::pokedex().get(&instance.id) {
                        info!("Party Slot {}: Lv{} {}", slot, instance.data.level, pokemon.data.name);
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::F5) {
            if let Some(mut saves) = get_mut::<PlayerSaves>() {

                if let Some(map) = if self.map_manager.chunk_active {
                    self.map_manager.chunk_map.chunk().map(|chunk| &chunk.map)
                } else {
                    self.map_manager.map_set_manager.set().map(|map| map.map()).flatten()
                } {

                    let name = &map.name;
                    info!("Resetting battled trainers in this map! ({})", name);
                    saves.get_mut().world_status.get_or_create_map_data(name).battled.clear();

                }               
            }
        }

        if is_key_pressed(KeyCode::F6) {
            if let Some(mut saves) = get_mut::<PlayerSaves>() {
                info!("Clearing used scripts in player data!");
                saves.get_mut().world_status.ran_scripts.clear();
            }
        }
        
    }
    
}