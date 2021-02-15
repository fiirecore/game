use crate::audio::play_music;
use crate::gui::Focus;
use crate::gui::GuiComponent;
use crate::util::Completable;
use crate::util::Input;
use crate::world::NpcTextures;
use crate::world::TileTextures;
use crate::world::gui::map_window_manager::MapWindowManager;
use macroquad::prelude::KeyCode;
use macroquad::prelude::info;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::warn;
use crate::util::input;
use crate::world::gui::player_world_gui::PlayerWorldGui;
use crate::audio::music::Music;
use crate::util::Render;
use crate::util::input::Control;
use crate::entity::Entity;
use crate::io::data::Direction;
use crate::io::data::player::PlayerData;
use crate::world::player::BASE_SPEED;
use crate::world::player::Player;
use crate::world::player::RUN_SPEED;
use crate::world::warp::WarpEntry;
use enum_iterator::IntoEnumIterator;
use super::RenderCoords;
use super::World;
use super::chunk::world_chunk_map::WorldChunkMap;
use super::set::world_map_set_manager::WorldMapSetManager;

pub struct WorldManager {

    chunk_map: WorldChunkMap,
    map_sets: WorldMapSetManager,
    textures: TileTextures,
    npc_textures: NpcTextures,

    current_music: Music,

    player: Player,

    player_gui: PlayerWorldGui,
    window_manager: MapWindowManager,

}

impl WorldManager {

    pub fn new(player_data: &PlayerData) -> Self {
        if let Some(message) = crate::gui::MESSAGE.lock().take() {
            info!("WorldManager cleared previous global message: {:?}", message);
        }
        let stuff = crate::io::data::map::load_maps();
        let mut this = Self {
            chunk_map: stuff.0,
            map_sets: stuff.1,
            player: Player::default(),
            current_music: Music::default(),
            player_gui: PlayerWorldGui::new(),
            window_manager: MapWindowManager::default(),
            textures: stuff.2,
            npc_textures: stuff.3,
        };
        
        this.load_player(player_data);
        return this;
    }

    pub fn on_start(&mut self) {
        if self.chunk_map.is_alive() {
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
        if let Some(playing_music) = crate::audio::get_music_playing() {
            if music != playing_music {
                self.current_music = music;  
                play_music(self.current_music);
            }
        }
    }

    fn get_map_music(&self) -> Music {
        if self.chunk_map.is_alive() {
            self.chunk_map.current_music
        } else {
            self.map_sets.map_set().map().music
        }
    }

    pub fn update(&mut self, delta: f32) {

        // Check global message to see if anything is there

        {
            let mut message = crate::gui::MESSAGE.lock();
            if message.is_some() {
                let message = message.take().unwrap();
                self.window_manager.spawn();
                self.window_manager.set_text(message);
            }
        }

        if self.window_manager.is_alive() {
            if self.window_manager.is_finished() {
                if let Some(index) = self.current_map_mut().npc_active.take() {
                    self.current_map_mut().npcs[index].after_interact();
                }
                self.window_manager.despawn();
            } else {
                self.window_manager.update(delta);
            }
        } else {
            if let Some(index) = self.current_map_mut().npc_active {

                // Play NPC music
                
                if let Some(trainer) = self.current_map_mut().npcs[index].trainer.as_ref() {
                    if let Some(music) = trainer.encounter_music {
                        if let Some(playing_music) = crate::audio::get_music_playing() {
                            if playing_music != music {
                                play_music(music);
                            }
                        } else {
                            play_music(music);
                        }
                    }
                }

                // Move npc or spawn textbox

                if self.current_map_mut().npcs[index].should_move() {
                    self.npc_movement(index, delta);
                } else {

                    self.window_manager.spawn();
                    self.current_map_mut().npcs[index].offset = None;
                    

                    let mut battled = false;

                    if let Some(trainer) = &self.current_map_mut().npcs[index].trainer {
                        let message_set = crate::io::data::text::MessageSet::new(
                            1, 
                            crate::io::data::text::color::TextColor::Blue, 
                            trainer.encounter_message.clone()
                        );
                        self.window_manager.set_text(message_set);
                        battled = true;
                    }

                    if battled {
                        if let Some(mut data) = macroquad::prelude::collections::storage::get_mut::<PlayerData>() {
                            let npc_name = self.current_map_mut().npcs[index].identifier.name.clone();
                            std::ops::DerefMut::deref_mut(&mut data).world_status.get_or_create_map_data(&self.current_map_mut().name).battled.insert(npc_name);
                        }
                    }

                    self.player.position.local.direction = self.current_map_mut().npcs[index].position.direction.inverse();
                    if self.player.frozen {
                        self.player.move_update(0.0);
                        self.player.frozen = false;
                    }
                }
            }
            if !self.player.frozen {
                self.player_movement(delta);
            }           
        }
    }

    pub fn render(&self) {
        let coords =  RenderCoords::new(&self.player);
        if self.chunk_map.is_alive() {
            self.chunk_map.render(&self.textures, &self.npc_textures, coords, true);
        } else if self.map_sets.is_alive() {
            self.map_sets.render(&self.textures, &self.npc_textures, coords, true);
        }
        self.player.render();
        self.player_gui.render(); 
        self.window_manager.render();     
    }

    pub fn save_data(&self, player_data: &mut PlayerData) {
        if self.chunk_map.is_alive() {
            player_data.location.map_id = String::from("world");
            player_data.location.map_index = 0;
        } else {
            player_data.location.map_id = self.map_sets.get().clone();
            player_data.location.map_index = *self.map_sets.get_index() as u16;
		}
		player_data.location.position = self.player.position;
    }

    pub fn load_chunk_map_at_player(&mut self) {
        let x = self.player.position.get_x();
        let y = self.player.position.get_y();
        if let Some(chunk) = self.chunk_map.chunk_id_at(x, y) {
            self.chunk_map.change_chunk(chunk, &mut self.player);
        } else {
            warn!("Could not load chunk at ({}, {})", x, y);
            self.chunk_map.change_chunk(2, &mut self.player);
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

            if self.chunk_map.is_alive() {
                self.chunk_map.input(delta, &mut self.player);
            } else {
                self.map_sets.input(delta, &mut self.player);
            }
    
            if self.player.position.local.x_offset == 0.0 && self.player.position.local.y_offset == 0.0 && !self.player.frozen {
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

                if !input::down(self.player.position.local.direction.keybind()) {
                    for direction in Direction::into_enum_iter() {
                        if input::down(direction.keybind()) {
                            self.move_direction(direction);
                            break;
                        }
                    }
                } else if input::down(self.player.position.local.direction.keybind()) {
                    self.move_direction(self.player.position.local.direction);
                } else {
                    self.player.moving = false;
                }

            }

        }
        
    }

    fn move_direction(&mut self, direction: Direction) {
        self.player.on_try_move(direction);
        let offset = direction.offset();
        let x = self.player.position.local.x + offset.0 as isize;
        let y = self.player.position.local.y + offset.1 as isize;
        let move_code = if self.chunk_map.is_alive() {
            if self.chunk_map.in_bounds(x, y) {
                if let Some(entry) = self.chunk_map.check_warp(x, y) {
                    self.warp(entry);
                    return;
                }            
                self.chunk_map.walkable(x, y)
            } else {
                // convert x and y to global coordinates
                let x = x + self.player.position.x_offset;
                let y = y + self.player.position.y_offset;
                self.chunk_map.walk_connections(&mut self.player, x, y)
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

        let in_bounds = if self.chunk_map.is_alive() {
            self.chunk_map.in_bounds(x, y)
        } else {
            self.map_sets.in_bounds(x, y)
        };

        let jump = if in_bounds {
            let tile_id = if self.chunk_map.is_alive() {
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
            self.player.position.local.x_offset = offset.0;
            self.player.position.local.y_offset = offset.1;
        }
    }

    fn player_movement(&mut self, delta: f32) {
        if self.player.position.local.x_offset != 0.0 || self.player.position.local.y_offset != 0.0 {
            match self.player.position.local.direction {
                Direction::Up => {
                    self.player.position.local.y_offset -= (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.local.y_offset <= -16.0 {
                        self.player.position.local.y -= 1;
                        self.player.position.local.y_offset = 0.0;
                        self.stop_player();
                    }
                }
                Direction::Down => {
                    self.player.position.local.y_offset += (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.local.y_offset >= 16.0 {
                        self.player.position.local.y += 1;
                        self.player.position.local.y_offset = 0.0;
                        self.stop_player();
                    }
                }
                Direction::Left => {
                    self.player.position.local.x_offset -= (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.local.x_offset <= -16.0 {
                        self.player.position.local.x -= 1;
                        self.player.position.local.x_offset = 0.0;
                        self.stop_player();
                    }
                }
                Direction::Right => {
                    self.player.position.local.x_offset += (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.local.x_offset >= 16.0 {
                        self.player.position.local.x += 1;
                        self.player.position.local.x_offset = 0.0;
                        self.stop_player();
                    }
                }
            }
            self.player.move_update(delta);
        }
    }

    fn npc_movement(&mut self, npc_index: usize, delta: f32) {
        let npc = &mut self.current_map_mut().npcs[npc_index];
        if npc.should_move() {
            match npc.position.direction {
                Direction::Up => {
                    npc.position.y_offset -= 60.0 * delta;
                    if npc.position.y_offset <= -16.0 {
                        npc.position.y -= 1;
                        npc.position.y_offset = 0.0;
                    }
                }
                Direction::Down => {
                    npc.position.y_offset += 60.0 * delta;
                    if npc.position.y_offset >= 16.0 {
                        npc.position.y += 1;
                        npc.position.y_offset = 0.0;
                    }
                }
                Direction::Left => {
                    npc.position.x_offset -= 60.0 * delta;
                    if npc.position.x_offset <= -16.0 {
                        npc.position.x -= 1;
                        npc.position.x_offset = 0.0;
                    }
                }
                Direction::Right => {
                    npc.position.x_offset += 60.0 * delta;
                    if npc.position.x_offset >= 16.0 {
                        npc.position.x += 1;
                        npc.position.x_offset = 0.0;
                    }
                }
            }
        }
    }

    fn stop_player(&mut self) {
        //self.player.moving = false;
        self.player.on_stopped_moving();
        let x = self.player.position.local.x;
        let y = self.player.position.local.y;
        if self.chunk_map.is_alive() {
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
        if !self.chunk_map.is_alive() {
            self.chunk_map.spawn();
            self.map_sets.despawn();
        }
        self.chunk_map.current_chunk = entry.destination.map_index;
        self.player.position.x_offset = self.chunk_map.current_chunk().x;
        self.player.position.y_offset = self.chunk_map.current_chunk().y;
        self.player.position.local.x = entry.destination.x;
        self.player.position.local.y = entry.destination.y;
        self.play_music();
    }

    pub fn warp_to_map_set(&mut self, entry: WarpEntry) {
        if !self.map_sets.is_alive() {
            self.map_sets.spawn();
            self.chunk_map.despawn();
        }
        self.load_map_set(&entry.destination.map_id, entry.destination.map_index);
        self.player.position.x_offset = 0;
        self.player.position.y_offset = 0;
        self.player.position.local.x = entry.destination.x;
        self.player.position.local.y = entry.destination.y;
        self.play_music();
    }

    pub fn current_map(&self) -> &super::WorldMap {
        if self.chunk_map.is_alive() {
            &self.chunk_map.current_chunk().map
        } else {
            self.map_sets.map_set().map()
        }
    }

    pub fn current_map_mut(&mut self) -> &mut super::WorldMap {
        if self.chunk_map.is_alive() {
            &mut self.chunk_map.current_chunk_mut().map
        } else {
            self.map_sets.map_set_mut().map_mut()
        }
    }

    pub fn load_player(&mut self, player_data: &PlayerData) {
        self.player = Player::new(player_data);
        self.player.load_textures();
        if player_data.location.map_id.as_str().eq("world") {
            self.chunk_map.spawn();
        } else {
            self.map_sets.spawn();
            self.load_map_set(&player_data.location.map_id, player_data.location.map_index);
        }
    }

    fn debug_input(&mut self) {
        if is_key_pressed(KeyCode::F1) {
            crate::util::battle_data::random_wild_battle();
        }

        if is_key_pressed(KeyCode::F2) {
            self.player.noclip = !self.player.noclip;
            if self.player.noclip {
                self.player.speed *= 4;
            } else {
                self.player.speed /= 4;
            }
            info!("No clip toggled!");
        }

        if is_key_pressed(KeyCode::F3) {
            info!("Local Coordinates: ({}, {})", self.player.position.local.x, self.player.position.local.y);
            info!("Global Coordinates: ({}, {})", self.player.position.get_x(), self.player.position.get_y());
            let tile = if self.chunk_map.is_alive() {
                self.chunk_map.tile(self.player.position.local.x, self.player.position.local.y)
            } else {
                self.map_sets.tile(self.player.position.local.x, self.player.position.local.y)
            };
            info!("Current Tile ID: {}", tile);
        }

        if is_key_pressed(KeyCode::F5) {
            let name = &self.current_map_mut().name;
            info!("Resetting battled trainers in this map! ({})", name);
            if let Some(mut data) = macroquad::prelude::collections::storage::get_mut::<PlayerData>() {
                std::ops::DerefMut::deref_mut(&mut data).world_status.get_or_create_map_data(name).battled.clear();
            }
        }

    }
    
}

pub fn test_move_code(move_code: u8, jump: bool) -> bool {
    move_code == 0x0C || move_code == 0x00 || move_code == 0x04 || jump
}