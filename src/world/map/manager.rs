use std::collections::HashMap;

use image::RgbaImage;
use log::info;
use log::warn;
use opengl_graphics::Texture;
use enum_iterator::IntoEnumIterator;

use crate::audio::music::Music;
use crate::util::Render;
use crate::util::context::GameContext;
use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::gui::gui::Activatable;
use crate::io::data::Direction;
use crate::io::data::player_data::PlayerData;
use crate::io::map::gba_map::fill_palette_map;
use crate::io::map::gba_map::get_texture;
use crate::io::map::map_loader::load_maps;
use crate::io::map::npc_loader::load_npc_textures;
use crate::gui::gui::GuiComponent;
use crate::world::gui::map_window_manager::MapWindowManager;
use crate::world::gui::player_world_gui::PlayerWorldGui;
use crate::world::player::BASE_SPEED;
use crate::world::player::Player;
use crate::world::player::RUN_SPEED;
use crate::world::warp::WarpEntry;

use super::RenderCoords;
use super::World;
use super::chunk::world_chunk_map::WorldChunkMap;
use super::set::world_map_set_manager::WorldMapSetManager;
#[derive(Default)]
pub struct WorldManager {

    pub(crate) chunk_map: WorldChunkMap,
    pub(crate) map_sets: WorldMapSetManager,

    pub(crate) player: Player,

    pub(crate) current_music: Music,

    // other

    // old world manager values below

    pub player_gui: PlayerWorldGui,

    pub window_manager: MapWindowManager,

    pub(crate) bottom_textures: HashMap<u16, Texture>,
    pub(crate) top_textures: HashMap<u16, Texture>,
    pub(crate) npc_textures: HashMap<u8, ThreeWayTexture>,

}

impl WorldManager {

    pub fn load(&mut self, player_data: &PlayerData) {
        self.load_maps_and_textures();
        self.load_player(player_data);
    }

    pub fn play_music(&mut self, context: &mut GameContext) {
        let music = if self.chunk_map.is_alive() {
            self.chunk_map.current_chunk().map.music
        } else {
            self.map_sets.map_set().map().music
        };
        if music != self.current_music {
            self.current_music = music;
            context.play_music(self.current_music);
        } else if !context.is_music_playing() {
            context.play_music(self.current_music);
        }
    }

    pub fn on_start(&mut self, context: &mut GameContext) {
        if self.chunk_map.is_alive() {
            self.load_chunk_map_at(
                context,
                self.player.position.x,
                self.player.position.y,
            );
        } else if self.map_sets.is_alive() {
            self.current_music = Music::from(self.map_sets.map_set().map().music);
        }
        context.play_music(self.current_music);
    }

    pub fn update(&mut self, context: &mut GameContext) {
        self.player_movement(context);
        //self.player.update(context);
        //self.world_map_manager.update(context, &self.player);
        //self.warp_map_manager.update(context, &self.player);
    }

    pub fn render(&mut self, ctx: &mut piston_window::Context, g: &mut opengl_graphics::GlGraphics, tr: &mut TextRenderer) {
        let coords =  RenderCoords::new(&self.player);
        if self.chunk_map.is_alive() {
            self.chunk_map.render(ctx, g, &self.bottom_textures, &self.npc_textures, coords, true);
        } else if self.map_sets.is_alive() {
            self.map_sets.render(ctx, g, &self.bottom_textures, &self.npc_textures, coords, true);
        }
        self.player.render(ctx, g, tr);
        self.player_gui.render(ctx, g, tr);      
    }


    pub fn load_chunk_map_at(&mut self, context: &mut GameContext, x: isize, y: isize) {
        if let Some(chunk) = self.chunk_map.chunk_id_at(x, y) {
            self.chunk_map.change_chunk(context, chunk);
        } else {
            warn!("Could not load chunk at {}, {}", x, y);
            self.chunk_map.change_chunk(context, 2);
        }
    }

    pub fn load_map_set(&mut self, id: &String, index: u16) {
        self.map_sets.set(id);
        self.map_sets.set_index(index as usize);
    }

    pub fn input(&mut self, context: &mut GameContext) {

        if cfg!(debug_assertions) {
            self.debug_input(context)
        }

        if context.keys[6] == 1 {
            self.player_gui.toggle();
        }

        if self.player_gui.in_focus() {
            self.player_gui.input(context);
        } else {
    
            if self.player.position.x_offset == 0 && self.player.position.y_offset == 0 && !self.player.frozen {
                self.player.moving = true;

                if context.key_active(1) {
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

                if !context.key_active(self.player.position.direction.keybind()) {
                    for direction in Direction::into_enum_iter() {
                        if context.key_active(direction.keybind()) {
                            self.move_direction(context, direction);
                            break;
                        }
                    }
                } else if context.key_active(self.player.position.direction.keybind()) {
                    self.move_direction(context, self.player.position.direction);
                } else {
                    self.player.moving = false;
                }

            }

        }
        
    }

    fn move_direction(&mut self, context: &mut GameContext, direction: Direction) {
        self.player.on_try_move(direction);
        let offset = direction.offset();
        let x = self.player.position.x + offset.0 as isize;
        let y = self.player.position.y + offset.1 as isize;
        let move_code = if self.chunk_map.is_alive() {
            if self.chunk_map.in_bounds(x, y) {
                if let Some(entry) = self.chunk_map.check_warp(x, y) {
                    self.warp(context, entry);
                    return;
                }            
                self.chunk_map.walkable(x, y)
            } else {
                self.chunk_map.walk_connections(context, x, y)
            }            
        } else {
            if self.map_sets.in_bounds(x, y) {
                if let Some(entry) = self.map_sets.check_warp(x, y) {
                    self.warp(context, entry);
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

    fn player_movement(&mut self, context: &mut GameContext) {
        if self.player.position.x_offset != 0 || self.player.position.y_offset != 0 {
            match self.player.position.direction {
                Direction::Up => {
                    self.player.position.y_offset -= self.player.speed as i8;
                    if self.player.position.y_offset <= -16 {
                        self.player.position.y -= 1;
                        self.player.position.y_offset = 0;
                        self.stop_player(context);
                    }
                }
                Direction::Down => {
                    self.player.position.y_offset += self.player.speed as i8;
                    if self.player.position.y_offset >= 16 {
                        self.player.position.y += 1;
                        self.player.position.y_offset = 0;
                        self.stop_player(context);
                    }
                }
                Direction::Left => {
                    self.player.position.x_offset -= self.player.speed as i8;
                    if self.player.position.x_offset <= -16 {
                        self.player.position.x -= 1;
                        self.player.position.x_offset = 0;
                        self.stop_player(context);
                    }
                }
                Direction::Right => {
                    self.player.position.x_offset += self.player.speed as i8;
                    if self.player.position.x_offset >= 16 {
                        self.player.position.x += 1;
                        self.player.position.x_offset = 0;
                        self.stop_player(context);
                    }
                }
            }
            self.player.move_update();
        }
    }

    fn stop_player(&mut self, context: &mut GameContext) {
        //self.player.moving = false;
        self.player.on_stopped_moving();
        let x = self.player.position.x;
        let y = self.player.position.y;
        if self.chunk_map.is_alive() {
            if self.chunk_map.in_bounds(x, y) {
                self.chunk_map.on_tile(context, x, y);
            }
        } else {
            if self.map_sets.in_bounds(x, y) {
                self.map_sets.on_tile(context, x, y);
            }
        }        
    }

    pub fn warp(&mut self, context: &mut GameContext, entry: WarpEntry) {
        // spawn warp transition here
        if entry.destination.map_id.as_str().eq("world") {
            self.warp_to_chunk_map(context, entry);
        } else {
            self.warp_to_map_set(context, entry);
        }
    }

    pub fn warp_to_chunk_map(&mut self, context: &mut GameContext, entry: WarpEntry) {
        if !self.chunk_map.is_alive() {
            self.chunk_map.spawn();
            self.map_sets.despawn();
        }
        self.chunk_map.current_chunk = entry.destination.map_index;
        self.player.position.x = self.chunk_map.current_chunk().x + entry.destination.x;
        self.player.position.y = self.chunk_map.current_chunk().y + entry.destination.y;
        self.play_music(context);
    }

    pub fn warp_to_map_set(&mut self, context: &mut GameContext, entry: WarpEntry) {
        if !self.map_sets.is_alive() {
            self.map_sets.spawn();
            self.chunk_map.despawn();
        }
        self.load_map_set(&entry.destination.map_id, entry.destination.map_index);
        self.player.position.x = entry.destination.x;
        self.player.position.y = entry.destination.y;
        self.play_music(context);
    }

    pub fn load_maps_and_textures(&mut self) {

        let mut bottom_sheets: HashMap<u8, RgbaImage> = HashMap::new();
        let mut top_sheets: HashMap<u8, RgbaImage> = HashMap::new();

        let palette_sizes = fill_palette_map(&mut bottom_sheets, &mut top_sheets);

        load_maps(&palette_sizes, &mut self.chunk_map, &mut self.map_sets);

        load_npc_textures(&mut self.npc_textures);

        for wmap in self.chunk_map.chunks.values() {
            for tile_id in &wmap.map.tile_map {
                if !(self.bottom_textures.contains_key(tile_id) && self.top_textures.contains_key(tile_id)) {
                    self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                    self.bottom_textures.insert(*tile_id, get_texture(&bottom_sheets, &palette_sizes, *tile_id));
                }
            }
            for tile_id in &wmap.map.border_blocks {
                if !(self.bottom_textures.contains_key(tile_id) && self.top_textures.contains_key(tile_id)) {
                    self.bottom_textures.insert(*tile_id, get_texture(&bottom_sheets, &palette_sizes, *tile_id));
                    self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                }
            }
        }
        for wmapset in self.map_sets.values() {
            for tile_id in &wmapset.tiles() {
                if !(self.bottom_textures.contains_key(tile_id) && self.top_textures.contains_key(tile_id)) {
                    self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
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

    fn debug_input(&mut self, context: &mut GameContext) {
        if context.fkeys[0] == 1 {
            context.random_wild_battle();
        }

        if context.fkeys[2] == 1 {
            context.stop_music();
        }

        if context.fkeys[1] == 1 {
            self.player.noclip = !self.player.noclip;
            if self.player.noclip {
                self.player.speed *= 4;
            } else {
                self.player.speed /= 4;
            }
            info!("No clip toggled!");
        }

        if context.fkeys[3] == 1 {
            if self.chunk_map.is_alive() {
                let mut pos_map = String::from("Local X: ");
                pos_map.push_str((self.player.position.x - self.chunk_map.current_chunk().x).to_string().as_str());
                pos_map.push_str(", Local Y: ");
                pos_map.push_str((self.player.position.y - self.chunk_map.current_chunk().y).to_string().as_str());
                info!("{}", pos_map);
            }
            let mut pos = String::from("X: ");
            pos.push_str(self.player.position.x.to_string().as_str());
            pos.push_str(", Y: ");
            pos.push_str(self.player.position.y.to_string().as_str());
            let tile = if self.chunk_map.is_alive() {
                self.chunk_map.tile(self.player.position.x, self.player.position.y)
            } else {
                self.map_sets.tile(self.player.position.x, self.player.position.y)
            };
            info!("{}, Tile: {}", pos, tile);
            
        }

    }
    
}

pub fn test_move_code(move_code: u8, jump: bool) -> bool {
    move_code == 0x0C || move_code == 0x00 || move_code == 0x04 || jump
}