use ahash::AHashMap as HashMap;
use crate::audio::play_music;
use crate::gui::Activatable;
use crate::gui::GuiComponent;
use crate::util::Completable;
use crate::world::gui::map_window_manager::MapWindowManager;
use macroquad::prelude::Image;
use macroquad::prelude::KeyCode;
use macroquad::prelude::info;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::warn;
use crate::util::input;
use crate::util::graphics::Texture;
use crate::world::gui::player_world_gui::PlayerWorldGui;
use crate::audio::music::Music;
use crate::util::Render;
use crate::util::input::Control;
use crate::entity::Entity;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::io::data::Direction;
use crate::io::data::player::PlayerData;
use crate::io::map::gba_map::fill_palette_map;
use crate::io::map::gba_map::get_texture;
use crate::io::map::map_loader::load_maps;
use crate::io::map::npc_loader::load_npc_textures;
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

    player: Player,

    current_music: Music,

    // other

    // old world manager values below

    player_gui: PlayerWorldGui,

    window_manager: MapWindowManager,

    bottom_textures: HashMap<u16, Texture>,
    //pub(crate) top_textures: HashMap<u16, Texture>,
    npc_textures: HashMap<u8, ThreeWayTexture>,

}

impl WorldManager {

    pub fn new(player_data: &PlayerData) -> Self {
        let mut this = Self {
            chunk_map: WorldChunkMap::default(),
            map_sets: WorldMapSetManager::default(),
            player: Player::default(),
            current_music: Music::default(),
            player_gui: PlayerWorldGui::new(),
            window_manager: MapWindowManager::default(),
            bottom_textures: HashMap::new(),
            npc_textures: HashMap::new(),
        };
        this.load_maps_and_textures();
        this.load_player(player_data);
        this
    }

    pub fn on_start(&mut self) {
        if self.chunk_map.is_alive() {
            self.load_chunk_map_at(
                self.player.position.x,
                self.player.position.y,
            );
        } else if self.map_sets.is_alive() {
            self.current_music = Music::from(self.map_sets.map_set().map().music);
        }
        play_music(self.current_music);
    }

    pub fn play_music(&mut self) {
        if let Some(music) = crate::audio::get_music_playing() {
            if music != self.current_music {
                self.current_music = if self.chunk_map.is_alive() {
                    self.chunk_map.current_chunk().map.music
                } else {
                    self.map_sets.map_set().map().music
                };
                play_music(self.current_music);
            }
        } else {
            play_music(self.current_music);
        }
    }

    pub fn update(&mut self, delta: f32) {
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
                self.window_manager.spawn();
                if let Some(trainer) = &self.current_map_mut().npcs[index].trainer {
                    if let Some(music) = trainer.encounter_music {
                        play_music(music);
                    }
                    let mut messages = Vec::new();
                    for index in 0..trainer.encounter_message.len() {
                        messages.push(crate::io::data::text::Message {
                            font_id: 1,
                            message: trainer.encounter_message[index].clone(),
                            color: crate::io::data::text::TextColor::Blue,
                            no_pause: false,
                        });
                    }
                    self.window_manager.set_text(messages);
                }
            }
            self.player_movement(delta);
        }
    }

    pub fn render(&self) {
        let coords =  RenderCoords::new(&self.player);
        if self.chunk_map.is_alive() {
            self.chunk_map.render(&self.bottom_textures, &self.npc_textures, coords, true);
        } else if self.map_sets.is_alive() {
            self.map_sets.render(&self.bottom_textures, &self.npc_textures, coords, true);
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

    pub fn load_chunk_map_at(&mut self, x: isize, y: isize) {
        if let Some(chunk) = self.chunk_map.chunk_id_at(x, y) {
            self.chunk_map.change_chunk(chunk);
        } else {
            warn!("Could not load chunk at {}, {}", x, y);
            self.chunk_map.change_chunk(2);
        }
    }

    pub fn load_map_set(&mut self, id: &String, index: u16) {
        self.map_sets.set(id);
        self.map_sets.set_index(index as usize);
    }

    pub fn input(&mut self, delta: f32) {

        if !crate::not_debug() {
            self.debug_input()
        }

        if input::pressed(Control::Start) {
            self.player_gui.toggle();
        }

        if self.player_gui.in_focus() {
            self.player_gui.input(delta);
        } else if self.window_manager.is_alive() {
            self.window_manager.input(delta);
        } else {

            if self.chunk_map.is_alive() {
                self.chunk_map.input(delta, &self.player);
            } else {
                self.map_sets.input(delta, &self.player);
            }
    
            if self.player.position.x_offset == 0.0 && self.player.position.y_offset == 0.0 && !self.player.frozen {
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

                if !input::down(self.player.position.direction.keybind()) {
                    for direction in Direction::into_enum_iter() {
                        if input::down(direction.keybind()) {
                            self.move_direction(direction);
                            break;
                        }
                    }
                } else if input::down(self.player.position.direction.keybind()) {
                    self.move_direction(self.player.position.direction);
                } else {
                    self.player.moving = false;
                }

            }

        }
        
    }

    fn move_direction(&mut self, direction: Direction) {
        self.player.on_try_move(direction);
        let offset = direction.offset();
        let x = self.player.position.x + offset.0 as isize;
        let y = self.player.position.y + offset.1 as isize;
        let move_code = if self.chunk_map.is_alive() {
            if self.chunk_map.in_bounds(x, y) {
                if let Some(entry) = self.chunk_map.check_warp(x, y) {
                    self.warp(entry);
                    return;
                }            
                self.chunk_map.walkable(x, y)
            } else {
                self.chunk_map.walk_connections(x, y)
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
            self.player.position.x_offset = offset.0;
            self.player.position.y_offset = offset.1;
        }
    }

    fn player_movement(&mut self, delta: f32) {
        if self.player.position.x_offset != 0.0 || self.player.position.y_offset != 0.0 {
            match self.player.position.direction {
                Direction::Up => {
                    self.player.position.y_offset -= (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.y_offset <= -16.0 {
                        self.player.position.y -= 1;
                        self.player.position.y_offset = 0.0;
                        self.stop_player();
                    }
                }
                Direction::Down => {
                    self.player.position.y_offset += (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.y_offset >= 16.0 {
                        self.player.position.y += 1;
                        self.player.position.y_offset = 0.0;
                        self.stop_player();
                    }
                }
                Direction::Left => {
                    self.player.position.x_offset -= (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.x_offset <= -16.0 {
                        self.player.position.x -= 1;
                        self.player.position.x_offset = 0.0;
                        self.stop_player();
                    }
                }
                Direction::Right => {
                    self.player.position.x_offset += (self.player.speed as f32) * 60.0 * delta;
                    if self.player.position.x_offset >= 16.0 {
                        self.player.position.x += 1;
                        self.player.position.x_offset = 0.0;
                        self.stop_player();
                    }
                }
            }
            self.player.move_update(delta);
        }
    }

    fn stop_player(&mut self) {
        //self.player.moving = false;
        self.player.on_stopped_moving();
        let x = self.player.position.x;
        let y = self.player.position.y;
        if self.chunk_map.is_alive() {
            if self.chunk_map.in_bounds(x, y) {
                self.chunk_map.on_tile(x, y);
            }
        } else {
            if self.map_sets.in_bounds(x, y) {
                self.map_sets.on_tile(x, y);
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
        self.player.position.x = self.chunk_map.current_chunk().x + entry.destination.x;
        self.player.position.y = self.chunk_map.current_chunk().y + entry.destination.y;
        self.play_music();
    }

    pub fn warp_to_map_set(&mut self, entry: WarpEntry) {
        if !self.map_sets.is_alive() {
            self.map_sets.spawn();
            self.chunk_map.despawn();
        }
        self.load_map_set(&entry.destination.map_id, entry.destination.map_index);
        self.player.position.x = entry.destination.x;
        self.player.position.y = entry.destination.y;
        self.play_music();
    }

    pub fn current_map_mut(&mut self) -> &mut super::WorldMap {
        if self.chunk_map.is_alive() {
            &mut self.chunk_map.current_chunk_mut().map
        } else {
            self.map_sets.map_set_mut().map_mut()
        }
    }

    pub fn load_maps_and_textures(&mut self) {

        let mut bottom_sheets: HashMap<u8, Image> = HashMap::new();
        //let mut top_sheets: HashMap<u8, RgbaImage> = HashMap::new();

        let palette_sizes = fill_palette_map(&mut bottom_sheets/*, &mut top_sheets*/);

        load_maps(&palette_sizes, &mut self.chunk_map, &mut self.map_sets);

        load_npc_textures(&mut self.npc_textures);

        for wmap in self.chunk_map.chunks.values() {
            for tile_id in &wmap.map.tile_map {
                if !(self.bottom_textures.contains_key(tile_id) ){// && self.top_textures.contains_key(tile_id)) {
                    //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                    self.bottom_textures.insert(*tile_id, get_texture(&bottom_sheets, &palette_sizes, *tile_id));
                }
            }
            for tile_id in &wmap.map.border_blocks {
                if !(self.bottom_textures.contains_key(tile_id) ){// && self.top_textures.contains_key(tile_id)) {
                    self.bottom_textures.insert(*tile_id, get_texture(&bottom_sheets, &palette_sizes, *tile_id));
                    //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                }
            }
        }
        for wmapset in self.map_sets.values() {
            for tile_id in &wmapset.tiles() {
                if !(self.bottom_textures.contains_key(tile_id) ){// && self.top_textures.contains_key(tile_id)) {
                    //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                    self.bottom_textures.insert(*tile_id, get_texture(&bottom_sheets, &palette_sizes, *tile_id));
                }
            }
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
            if self.chunk_map.is_alive() {
                info!("Local X: {}, Local Y: {}", self.player.position.x - self.chunk_map.current_chunk().x, self.player.position.y - self.chunk_map.current_chunk().y);
            }
            let tile = if self.chunk_map.is_alive() {
                self.chunk_map.tile(self.player.position.x, self.player.position.y)
            } else {
                self.map_sets.tile(self.player.position.x, self.player.position.y)
            };
            info!("X: {}, Y: {}, Tile ID: {}", self.player.position.x, self.player.position.y, tile);
        }

    }
    
}

pub fn test_move_code(move_code: u8, jump: bool) -> bool {
    move_code == 0x0C || move_code == 0x00 || move_code == 0x04 || jump
}