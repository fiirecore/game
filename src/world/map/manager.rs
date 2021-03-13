use firecore_audio::play_music;
use firecore_world::map::chunk::world_chunk_map::WorldChunkMap;
use firecore_world::map::set::manager::WorldMapSetManager;
use firecore_world::World;
use firecore_world::script::WorldActionKind;
use firecore_world::test_move_code;
use firecore_world::warp::WarpDestination;
use macroquad::prelude::KeyCode;
use macroquad::prelude::info;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::warn;
use firecore_util::music::Music;
use firecore_input::{self as input, Control};
use firecore_util::Entity;
use firecore_util::Direction;

use crate::io::data::player::PLAYER_DATA;
use crate::util::graphics::texture::byte_texture;
use crate::world::GameWorld;
use crate::gui::Focus;
use crate::gui::GuiComponent;
use crate::util::Input;
use crate::world::NpcTextures;
use crate::world::TileTextures;
use crate::world::gui::map_window_manager::MapWindowManager;
use crate::world::gui::player_world_gui::PlayerWorldGui;
use crate::io::data::player::PlayerData;
use crate::world::player::BASE_SPEED;
use crate::world::player::Player;
use crate::world::player::RUN_SPEED;

use super::RenderCoords;

pub struct WorldManager {

    pub chunk_map: WorldChunkMap,
    pub map_sets: WorldMapSetManager,
    pub chunk_active: bool,

    tile_textures: TileTextures,
    npc_textures: NpcTextures,

    current_music: Music,

    pub player: Player,

    // warp_transition: WarpTransition,

    player_gui: PlayerWorldGui,
    window_manager: MapWindowManager,

    // Other

    noclip_toggle: bool,
    // command_alive: bool,
    // command: Mutex<String>,
    // tp: Mutex<Option<Coordinate>>,

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
            self.chunk_map.current_chunk().map.music
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
        
        let mut warp = None;
        for script in self.current_map_mut().scripts.iter_mut() {
            if script.is_alive() {
                if let Some(action) = script.actions_clone.front() {
                    match action {
                        WorldActionKind::Warp(destination) => {
                            warp = Some(destination.clone());
                            script.despawn();
                        }
                        _ => (),
                    }
                }
            }
        }
        if let Some(warp) = warp {
            self.warp(warp);
            if self.chunk_active {
                self.chunk_map.on_tile(&mut self.player);
            } else {
                self.map_sets.on_tile(&mut self.player);
            }
        }

        if self.chunk_active {
            self.chunk_map.update(delta, &mut self.player, &mut self.window_manager);
        } else {
            self.map_sets.update(delta, &mut self.player, &mut self.window_manager);
        }

        if !self.player.frozen {
            if self.player.do_move(delta) {
                self.stop_player();
            }
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

        // if self.command_alive {
        //     // macroquad::camera::set_camera(macroquad::camera::Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, macroquad::prelude::screen_width(), macroquad::prelude::screen_height())));
        //     Window::new(hash!(), vec2(0.0, 0.0), vec2(200.0, 100.0))
        //         .close_button(false)
        //         .label("Run commands")
        //         .ui(&mut macroquad::ui::root_ui(), |ui| {
        //             InputText::new(0).label("Command").ui(ui, &mut self.command.lock());
        //             if Button::new("Run").size(vec2(40.0, 50.0)).ui(ui) {
        //             }
        //         }
        //     );
        //     // macroquad::camera::set_camera(macroquad::camera::Camera2D::from_display_rect(crate::CAMERA_SIZE));
        // }
        
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
        let coords = self.player.position.absolute();
        if let Some(chunk) = self.chunk_map.chunk_id_at(&coords) {
            self.chunk_map.change_chunk(chunk, &mut self.player.position);
        } else {
            warn!("Could not load chunk at {}", coords);
            self.chunk_map.change_chunk(2, &mut self.player.position);
        }
        let music = self.chunk_map.current_chunk().map.music;
        if music != self.current_music {
            self.current_music = music;
            play_music(music);
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
        let offset = direction.tile_offset();
        let coords = self.player.position.local.coords.add(offset.0, offset.1);
        let move_code = if self.chunk_active {
            if self.chunk_map.in_bounds(&coords) {
                if let Some(entry) = self.chunk_map.check_warp(&coords) {
                    self.warp(entry.destination);
                    return;
                }            
                self.chunk_map.walkable(&coords)
            } else {
                // convert x and y to global coordinates
                let global = self.player.position.offset.add(coords.x, coords.y);
                let id = self.chunk_map.current_chunk;
                let move_code = self.chunk_map.walk_connections(&mut self.player.position, &global);
                if id != self.chunk_map.current_chunk {
                    let music = self.chunk_map.current_chunk().map.music;
                    if music != self.current_music {
                        self.current_music = music;
                        play_music(music);
                    }
                }
                move_code
            }            
        } else {
            if self.map_sets.in_bounds(&coords) {
                if let Some(entry) = self.map_sets.check_warp(&coords) {
                    self.warp(entry.destination);
                    return;
                }
                self.map_sets.walkable(&coords)
            } else {
                1
            }
        };

        let in_bounds = if self.chunk_active {
            self.chunk_map.in_bounds(&coords)
        } else {
            self.map_sets.in_bounds(&coords)
        };

        let jump = if in_bounds {
            let tile_id = if self.chunk_active {
                self.chunk_map.tile(&coords)
            } else {
                self.map_sets.tile(&coords)
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
            self.player.position.local.offset.x = offset.0 as f32 * mult;
            self.player.position.local.offset.y = offset.1 as f32 * mult;
        }
    }

    fn stop_player(&mut self) {
        //self.player.moving = false;
        // self.player.on_stopped_moving();
        let coords = &self.player.position.local.coords;
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
            if self.chunk_map.in_bounds(coords) {
                self.chunk_map.on_tile(&mut self.player);
            }
        } else {
            if self.map_sets.in_bounds(coords) {
                self.map_sets.on_tile(&mut self.player);
            }
        }        
    }

    pub fn warp(&mut self, destination: WarpDestination) {
        // spawn warp transition here
        if destination.map_id.as_str().eq("world") {
            self.warp_to_chunk_map(destination);
        } else {
            self.warp_to_map_set(destination);
        }
    }

    pub fn warp_to_chunk_map(&mut self, destination: WarpDestination) {
        if !self.chunk_active {
            self.chunk_active = true;
        }
        if let Some(chunk) = self.chunk_map.update_chunk(&destination.map_index) {
            self.player.position.offset.x = chunk.x;
            self.player.position.offset.y = chunk.y;
            self.player.position.local.coords.x = destination.x;
            self.player.position.local.coords.y = destination.y;
            self.play_music();
        }
        
    }

    pub fn warp_to_map_set(&mut self, destination: WarpDestination) {
        if self.chunk_active {
            self.chunk_active = false;
        }
        self.load_map_set(&destination.map_id, destination.map_index);
        self.player.position.offset.x = 0;
        self.player.position.offset.y = 0;
        self.player.position.local.coords.x = destination.x;
        self.player.position.local.coords.y = destination.y;
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
        if let Some(player_data) = crate::io::data::player::PLAYER_DATA.read().as_ref() {
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

        // if is_key_pressed(KeyCode::GraveAccent) {
        //     self.command_alive = !self.command_alive;
        // }

        if is_key_pressed(KeyCode::F1) {
            crate::util::battle_data::random_wild_battle();
        }

        if is_key_pressed(KeyCode::F2) {
            self.noclip_toggle = true;
        }

        if is_key_pressed(KeyCode::F3) {
            info!("Local Coordinates: {}", self.player.position.local.coords);
            info!("Global Coordinates: ({}, {})", self.player.position.get_x(), self.player.position.get_y());
            let tile = if self.chunk_active {
                self.chunk_map.tile(&self.player.position.local.coords)
            } else {
                self.map_sets.tile(&self.player.position.local.coords)
            };
            info!("Current Tile ID: {}", tile);
        }

        if is_key_pressed(KeyCode::F4) {
            if let Some(data) = PLAYER_DATA.read().as_ref() {
                for (slot, instance) in data.party.pokemon.iter().enumerate() {
                    if let Some(pokemon) = firecore_pokedex::POKEDEX.get(&instance.id) {
                        info!("Party Slot {}: Lv{} {}", slot, instance.level, pokemon.data.name);
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::F5) {
            if let Some(data) = PLAYER_DATA.write().as_mut() {
                let name = &self.current_map_mut().name;
                info!("Resetting battled trainers in this map! ({})", name);
                data.world_status.get_or_create_map_data(name).battled.clear();
            }
        }

        if is_key_pressed(KeyCode::F6) {
            if let Some(data) = PLAYER_DATA.write().as_mut() {
                info!("Clearing world events in player data!");
                data.world_status.completed_events.clear();
            }
        }
        
    }
    
}