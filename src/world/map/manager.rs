use crate::util::graphics::texture::byte_texture;
use crate::world::GameWorld;
use firecore_audio::play_music;
use firecore_world::map::chunk::world_chunk_map::WorldChunkMap;
use firecore_world::map::set::manager::WorldMapSetManager;
use firecore_world::World;
use firecore_world::test_move_code;
use crate::gui::Focus;
use crate::gui::GuiComponent;
use crate::util::Input;
use crate::world::NpcTextures;
use crate::world::TileTextures;
use crate::world::gui::map_window_manager::MapWindowManager;
use macroquad::prelude::KeyCode;
use macroquad::prelude::info;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::warn;
use crate::world::gui::player_world_gui::PlayerWorldGui;
use firecore_audio::music::Music;
use firecore_input::{self as input, Control};
use firecore_util::Entity;
use firecore_util::Direction;
use crate::io::data::player::PlayerData;
use crate::world::player::BASE_SPEED;
use crate::world::player::Player;
use crate::world::player::RUN_SPEED;
use firecore_world::warp::WarpEntry;
use super::RenderCoords;

pub struct WorldManager {

    chunk_map: WorldChunkMap,
    map_sets: WorldMapSetManager,
    chunk_active: bool,

    tile_textures: TileTextures,
    npc_textures: NpcTextures,

    current_music: Music,

    player: Player,

    // warp_transition: WarpTransition,

    player_gui: PlayerWorldGui,
    window_manager: MapWindowManager,

    noclip_toggle: bool,

}

impl WorldManager {

    pub fn new() -> Self {        
        Self {
            chunk_map: WorldChunkMap::new(),
            map_sets: WorldMapSetManager::default(),
            chunk_active: true,
            player: Player::default(),
            current_music: Music::default(),
            player_gui: PlayerWorldGui::new(),
            window_manager: MapWindowManager::default(),
            tile_textures: TileTextures::new(),
            npc_textures: NpcTextures::new(),
            noclip_toggle: false,
        }
    }

    pub async fn load(&mut self) {
        if let Some(message) = crate::gui::MESSAGE.lock().take() {
            info!("WorldManager cleared previous global message: {:?}", message);
        }
        self.tile_textures.setup();
        let maps = crate::io::data::map::v2::load_maps_v2(&mut self.tile_textures, &mut self.npc_textures).await;
        self.chunk_map = maps.0;
        self.map_sets = maps.1;
    }

    pub async fn on_start(&mut self) {
        PlayerData::load_selected_data().await;
        self.load_player();
        if self.chunk_active {
            self.load_chunk_map_at_player();
        }
        self.play_music();
    }

    pub fn play_music(&mut self) {
        let music = self.get_map_music();
        if music != self.current_music {
            self.current_music = music;  
            play_music(self.current_music);
        }
        if let Some(playing_music) = firecore_audio::get_music_playing() {
            if music != playing_music {
                self.current_music = music;  
                play_music(self.current_music);
            }
        }
    }

    fn get_map_music(&self) -> Music {
        if self.chunk_active {
            self.chunk_map.current_music
        } else {
            self.map_sets.map_set().map().music
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.tile_textures.update(delta);

        // Check global message to see if anything is there

        {
            let mut message = crate::gui::MESSAGE.lock();
            if message.is_some() {
                let message = message.take().unwrap();
                self.window_manager.spawn();
                self.window_manager.set_text(message);
            }
        }

        if self.chunk_active {
            self.chunk_map.update(delta, &mut self.player, &mut self.window_manager);
        } else {
            self.map_sets.update(delta, &mut self.player, &mut self.window_manager);
        }

        if !self.player.frozen {
            self.player_movement(delta);
        }   

        
    }

    pub fn render(&self) {
        let coords = RenderCoords::new(&self.player);
        if self.chunk_active {
            self.chunk_map.render(&self.tile_textures, &self.npc_textures, coords, true);
        } else {
            self.map_sets.render(&self.tile_textures, &self.npc_textures, coords, true);
        }
        self.player.render();
        self.player_gui.render(); 
        self.window_manager.render();
        
    }

    pub fn save_data(&self, player_data: &mut PlayerData) {
        if self.chunk_active {
            player_data.location.map_id = String::from("world");
            player_data.location.map_index = 0;
        } else {
            player_data.location.map_id = self.map_sets.current_map_set.clone();
            player_data.location.map_index = self.map_sets.map_set().current_map as u16;
		}
		player_data.location.position = self.player.position;
    }

    pub fn load_chunk_map_at_player(&mut self) {
        let x = self.player.position.get_x();
        let y = self.player.position.get_y();
        if let Some(chunk) = self.chunk_map.chunk_id_at(x, y) {
            self.chunk_map.change_chunk(chunk, &mut self.player.position);
        } else {
            warn!("Could not load chunk at ({}, {})", x, y);
            self.chunk_map.change_chunk(2, &mut self.player.position);
        }
    }

    pub fn load_map_set(&mut self, id: &String, index: u16) {
        self.map_sets.set(id);
        self.map_sets.set_index(index as usize);
    }

    pub fn input(&mut self, delta: f32) {

        if cfg!(debug_assertions) {
            self.debug_input()
        }

        if input::pressed(Control::Start) {
            self.player_gui.toggle();
        }

        if self.window_manager.is_alive() {
            self.window_manager.input(delta);
        } else if self.player_gui.in_focus() {
            self.player_gui.input(delta);
        } else {

            if self.chunk_active {
                self.chunk_map.input(delta, &mut self.player);
            } else {
                self.map_sets.input(delta, &mut self.player);
            }
    
            if self.player.position.local.offset.x == 0.0 && self.player.position.local.offset.y == 0.0 && !self.player.frozen {
                self.player.moving = true;

                if input::down(Control::B) {
                    if !self.player.running {
                        self.player.running = true;
                        self.player.speed = if self.player.noclip {
                            RUN_SPEED << 1
                        } else {
                            RUN_SPEED
                        };
                    }
                } else if self.player.running {
                    self.player.running = false;
                    self.player.speed = BASE_SPEED;
                }

                if !input::down(crate::util::keybind(self.player.position.local.direction)) {
                    for direction in firecore_util::DIRECTIONS {
                        let direction = *direction;
                        if input::down(crate::util::keybind(direction)) {
                            self.move_direction(direction, delta);
                            break;
                        }
                    }
                } else if input::down(crate::util::keybind(self.player.position.local.direction)) {
                    self.move_direction(self.player.position.local.direction, delta);
                } else {
                    self.player.moving = false;
                }

            }

        }
        
    }

    fn move_direction(&mut self, direction: Direction, delta: f32) {
        self.player.on_try_move(direction);
        let offset = direction.offset();
        let x = self.player.position.local.coords.x + offset.0 as isize;
        let y = self.player.position.local.coords.y + offset.1 as isize;
        let move_code = if self.chunk_active {
            if self.chunk_map.in_bounds(x, y) {
                if let Some(entry) = self.chunk_map.check_warp(x, y) {
                    self.warp(entry);
                    return;
                }            
                self.chunk_map.walkable(x, y)
            } else {
                // convert x and y to global coordinates
                let x = x + self.player.position.offset.x;
                let y = y + self.player.position.offset.y;
                self.chunk_map.walk_connections(&mut self.player.position, x, y)
            }            
        } else {
            if self.map_sets.in_bounds(x, y) {
                if let Some(entry) = self.map_sets.check_warp(x, y) {
                    self.warp(entry);
                    return;
                }
                self.map_sets.walkable(x, y)
            } else {
                1
            }
        };

        let in_bounds = if self.chunk_active {
            self.chunk_map.in_bounds(x, y)
        } else {
            self.map_sets.in_bounds(x, y)
        };

        let jump = if in_bounds {
            let tile_id = if self.chunk_active {
                self.chunk_map.tile(x, y)
            } else {
                self.map_sets.tile(x, y)
            };
            match direction {
                Direction::Up => false,
                Direction::Down => {
                    tile_id == 135 || tile_id == 176 || tile_id == 177 || tile_id == 143 || tile_id == 151 || tile_id == 184 || tile_id == 185 || tile_id == 192 || tile_id == 193 || tile_id == 217 || tile_id == 1234
                }
                Direction::Left => {
                    tile_id == 133
                }
                Direction::Right => {
                    tile_id == 134
                }
            }
        } else {
            false
        };
        if test_move_code(move_code, jump || self.player.noclip) {
            let mult = (self.player.speed as f32) * 60.0 * delta;
            self.player.position.local.offset.x = offset.0 * mult;
            self.player.position.local.offset.y = offset.1 * mult;
        }
    }

    fn player_movement(&mut self, delta: f32) {
        if self.player.position.local.offset.x != 0.0 || self.player.position.local.offset.y != 0.0 {
            match self.player.position.local.direction {
                Direction::Up => {
                    self.player.position.local.offset.y -= (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.local.offset.y <= -16.0 {
                        self.player.position.local.coords.y -= 1;
                        self.player.position.local.offset.y = 0.0;
                        self.stop_player();
                    }
                }
                Direction::Down => {
                    self.player.position.local.offset.y += (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.local.offset.y >= 16.0 {
                        self.player.position.local.coords.y += 1;
                        self.player.position.local.offset.y = 0.0;
                        self.stop_player();
                    }
                }
                Direction::Left => {
                    self.player.position.local.offset.x -= (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.local.offset.x <= -16.0 {
                        self.player.position.local.coords.x -= 1;
                        self.player.position.local.offset.x = 0.0;
                        self.stop_player();
                    }
                }
                Direction::Right => {
                    self.player.position.local.offset.x += (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.local.offset.x >= 16.0 {
                        self.player.position.local.coords.x += 1;
                        self.player.position.local.offset.x = 0.0;
                        self.stop_player();
                    }
                }
            }
            // self.player.move_update(delta);
        }
    }

    fn stop_player(&mut self) {
        //self.player.moving = false;
        // self.player.on_stopped_moving();
        let x = self.player.position.local.coords.x;
        let y = self.player.position.local.coords.y;
        if self.noclip_toggle {
            self.noclip_toggle = false;
            self.player.noclip = !self.player.noclip;
            if self.player.noclip {
                self.player.speed *= 4;
            } else {
                self.player.speed /= 4;
            }
            info!("No clip toggled!");
        }
        if self.chunk_active {
            if self.chunk_map.in_bounds(x, y) {
                self.chunk_map.on_tile(&mut self.player);
            }
        } else {
            if self.map_sets.in_bounds(x, y) {
                self.map_sets.on_tile(&mut self.player);
            }
        }        
    }

    pub fn warp(&mut self, entry: WarpEntry) {
        // spawn warp transition here
        if entry.destination.map_id.as_str().eq("world") {
            self.warp_to_chunk_map(entry);
        } else {
            self.warp_to_map_set(entry);
        }
    }

    pub fn warp_to_chunk_map(&mut self, entry: WarpEntry) {
        if !self.chunk_active {
            self.chunk_active = true;
        }
        if let Some(chunk) = self.chunk_map.update_chunk(&entry.destination.map_index) {
            self.player.position.offset.x = chunk.x;
            self.player.position.offset.y = chunk.y;
            self.player.position.local.coords.x = entry.destination.x;
            self.player.position.local.coords.y = entry.destination.y;
            self.play_music();
        }
        
    }

    pub fn warp_to_map_set(&mut self, entry: WarpEntry) {
        if self.chunk_active {
            self.chunk_active = false;
        }
        self.load_map_set(&entry.destination.map_id, entry.destination.map_index);
        self.player.position.offset.x = 0;
        self.player.position.offset.y = 0;
        self.player.position.local.coords.x = entry.destination.x;
        self.player.position.local.coords.y = entry.destination.y;
        self.play_music();
    }

    pub fn current_map(&self) -> &super::WorldMap {
        if self.chunk_active {
            &self.chunk_map.current_chunk().map
        } else {
            self.map_sets.map_set().map()
        }
    }

    pub fn current_map_mut(&mut self) -> &mut super::WorldMap {
        if self.chunk_active {
            &mut self.chunk_map.current_chunk_mut().map
        } else {
            self.map_sets.map_set_mut().map_mut()
        }
    }

    pub fn load_player(&mut self) {
        if let Some(player_data) = macroquad::prelude::collections::storage::get::<PlayerData>() {
            self.player = Player::new(&player_data);
            self.player.walking_texture = Some(byte_texture(include_bytes!("../../../build/assets/player.png")));
            self.player.running_texture = Some(byte_texture(include_bytes!("../../../build/assets/player_running.png")));
            // self.player.load_textures();
            if player_data.location.map_id.as_str().eq("world") {
                self.chunk_active = true;
            } else {
                self.chunk_active = false;
                self.load_map_set(&player_data.location.map_id, player_data.location.map_index);
            }
        }        
    }

    fn debug_input(&mut self) {
        if is_key_pressed(KeyCode::F1) {
            crate::util::battle_data::random_wild_battle();
        }

        if is_key_pressed(KeyCode::F2) {
            self.noclip_toggle = true;
        }

        if is_key_pressed(KeyCode::F3) {
            info!("Local Coordinates: ({}, {})", self.player.position.local.coords.x, self.player.position.local.coords.y);
            info!("Global Coordinates: ({}, {})", self.player.position.get_x(), self.player.position.get_y());
            let tile = if self.chunk_active {
                self.chunk_map.tile(self.player.position.local.coords.x, self.player.position.local.coords.y)
            } else {
                self.map_sets.tile(self.player.position.local.coords.x, self.player.position.local.coords.y)
            };
            info!("Current Tile ID: {}", tile);
        }

        if is_key_pressed(KeyCode::F4) {
            
        }

        if is_key_pressed(KeyCode::F5) {
            if let Some(mut data) = macroquad::prelude::collections::storage::get_mut::<PlayerData>() {
                let name = &self.current_map_mut().name;
                info!("Resetting battled trainers in this map! ({})", name);
                data.world_status.get_or_create_map_data(name).battled.clear();
            }
        }

        if is_key_pressed(KeyCode::F6) {
            if let Some(mut data) = macroquad::prelude::collections::storage::get_mut::<PlayerData>() {
                info!("Clearing world events in player data!");
                data.world_status.completed_events.clear();
            }
        }
        
    }
    
}