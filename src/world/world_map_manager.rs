use std::collections::HashMap;

use image::RgbaImage;
use log::info;
use log::warn;
use opengl_graphics::Texture;

use crate::audio::music::Music;
use crate::engine::game_context::GameContext;
use crate::entity::entities::player::Player;
use crate::entity::entities::player::RUN_SPEED;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::gui::gui::Activatable;
use crate::io::data::Direction;
use crate::io::data::player_data::PlayerData;
use crate::io::map::gba_map::fill_palette_map;
use crate::io::map::gba_map::get_texture;
use crate::io::map::map_loader::load_maps;
use crate::io::map::npc_loader::load_npc_textures;
use crate::util::traits::Loadable;
use crate::gui::gui::GuiComponent;

use super::ScreenCoords;
use super::World;
use super::gui::map_window_manager::MapWindowManager;
use super::gui::player_world_gui::PlayerWorldGui;
use super::warp::WarpEntry;
use super::world_chunk_map::WorldChunkMap;
use super::world_map_set_manager::WorldMapSetManager;

pub struct WorldMapManager {

    pub(crate) chunk_map: WorldChunkMap,
    pub(crate) map_sets: WorldMapSetManager,

    pub(crate) player: Player,

    pub(crate) current_music: Music,

    // old world manager values below
    
    pub world_id: String,

    pub player_gui: PlayerWorldGui,

    pub window_manager: MapWindowManager,

    pub(crate) bottom_textures: HashMap<u16, Texture>,
    pub(crate) top_textures: HashMap<u16, Texture>,
    pub(crate) npc_textures: HashMap<u8, ThreeWayTexture>,

    //pub palette_sizes: Vec<u16>,

}

impl WorldMapManager {

    pub fn new(player_data: &PlayerData) -> WorldMapManager {

        // let id = player_data.location.world_id.as_str();

        // let mut filename = String::from("worlds/");
        // filename.push_str(id);
        // filename.push_str("/");
        // filename.push_str(id);
        // filename.push_str(".toml");

        // match std::fs::read_to_string(asset_as_pathbuf(filename)) {
        //     Ok(content) => {

        //         let toml: WorldConfig = toml::from_str(content.as_str()).unwrap();

                WorldMapManager {

                    chunk_map: WorldChunkMap::new(),
                    map_sets: WorldMapSetManager::default(),

                    player: Player::default(),

                    current_music: Music::Pallet,

                    //

                    world_id: player_data.world_id.clone(),
                
                    window_manager: MapWindowManager::new(),
                    player_gui: PlayerWorldGui::new(),



                    bottom_textures: HashMap::new(),
                    top_textures: HashMap::new(),
                    npc_textures: HashMap::new(),

                    //palette_sizes: toml.palette_sizes.unwrap(),
                    //no_data_spawnpoint: toml.no_data_spawnpoint.unwrap(),
                }

            // }
            // Err(error) => {
            //     panic!("Error opening toml file {}", error);
            // }

        //}

    }

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
        self.player.update(context);
        //self.world_map_manager.update(context, &self.player);
        //self.warp_map_manager.update(context, &self.player);
    }

    pub fn render(&mut self, ctx: &mut piston_window::Context, g: &mut opengl_graphics::GlGraphics, tr: &mut crate::scene::scene::TextRenderer) {
        self.player.focus_update();
        let coords =  ScreenCoords::new(&self.player);
        if self.chunk_map.is_alive() {
            self.chunk_map.render(ctx, g, &self.bottom_textures, &self.npc_textures, coords, true);
        } else if self.map_sets.is_alive() {
            self.map_sets.map_set().render(ctx, g, &self.bottom_textures, &self.npc_textures, coords, true);
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
        self.map_sets.set(id.clone());
        self.map_sets.map_set_mut().set(index as usize);
    }

    pub fn input(&mut self, context: &mut GameContext) {

        if context.fkeys[0] == 1 {
			context.battle_context.random_wild_battle(&mut context.random);
        }

        if context.keys[6] == 1 {
            self.player_gui.toggle();
        }

        if self.player_gui.in_focus() {
            self.player_gui.input(context);
        } else {

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
                //context.app_console.add_line(String::from("Noclip toggled!"));
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
                    self.map_sets.map_set().tile(self.player.position.x, self.player.position.y)
                };
                info!("{}, Tile: {}", pos, tile);
                
            }

            self.player.reset_speed();
            if context.keys[1] == 1 || context.keys[1] == 2 {
                self.player.running = true;
                if self.player.noclip {
                    self.player.speed = RUN_SPEED * 2;
                } else {
                    self.player.speed = RUN_SPEED;
                }
            }
    
            if !self.player.moving {
    
                let mut x_offset: i8 = 0;
                let mut y_offset: i8 = 0;
                let mut jump = false;
    
                if context.keys[2] == 1 || context.keys[2] == 2 {
                    // Up
                    //self.player.dir_changed = true;
                    self.player.position.direction = Direction::Up;
                    y_offset -= 1;
                    self.player.on_try_move(self.player.position.direction);
                }
                if context.keys[3] == 1 || context.keys[3] == 2 {
                    //self.player.dir_changed = true;
                    self.player.position.direction = Direction::Down;
                    y_offset += 1;
                    // if self.chunk_map.is_alive() {
                    //     let tile_id = self.chunk_map.tile(self.player.position.x, self.player.position.y + y_offset as isize);
                    //     if tile_id == 135 || tile_id == 176 || tile_id == 177 || tile_id == 143 || tile_id == 184 || tile_id == 185 || tile_id == 217 || tile_id == 1234 {
                    //         jump = true;
                    //     }
                    // }                    
                    self.player.on_try_move(self.player.position.direction);
                }
                if context.keys[4] == 1 || context.keys[4] == 2 {
                    // Left
                    //self.player.dir_changed = true;
                    self.player.position.direction = Direction::Left;
                    x_offset -= 1;
                    if self.chunk_map.is_alive() {
                    let tile_id = self.chunk_map.tile(self.player.position.x + x_offset as isize, self.player.position.y);
                        if tile_id == 133 {
                            jump = true;
                        }
                    }
                    self.player.on_try_move(self.player.position.direction);
                }
                if context.keys[5] == 1 || context.keys[5] == 2 {
                    // Right
                    //self.player.dir_changed = true;
                    self.player.position.direction = Direction::Right;
                    x_offset += 1;
                    if self.chunk_map.is_alive() {
                    let tile_id = self.chunk_map.tile(self.player.position.x + x_offset as isize, self.player.position.y);
                        if tile_id == 134 {
                            jump = true;
                        }
                    }
                    self.player.on_try_move(self.player.position.direction);
                }
    
                if x_offset != 0 && y_offset != 0 {
                    y_offset = 0;
                }
    
                if self.chunk_map.is_alive() {
                    self.chunk_map.input(context, &self.player);
                } else {
                    self.map_sets.map_set_mut().input(context, &self.player);
                }
                
                if x_offset != 0 || y_offset != 0 {
    
                    let code;
    
                    if self.chunk_map.is_alive() {
    
                        code = self.chunk_map.walkable(context, self.player.position.x + x_offset as isize, self.player.position.y + y_offset as isize);
                        if let Some(entry) = self.chunk_map.check_warp(self.player.position.x + x_offset as isize, self.player.position.y + y_offset as isize) {
                            self.warp(context, entry);
                            return;
                        }
                    } else {
    
                        let cms = self.map_sets.map_set_mut();
                        
                        code = cms.walkable(context, self.player.position.x + x_offset as isize, self.player.position.y + y_offset as isize);
                        if let Some(entry) = cms.check_warp(self.player.position.x + x_offset as isize, self.player.position.y + y_offset as isize) {
                            self.warp(context, entry);
                            return;
                        }
                    }
                    if code == 0x0C || self.player.noclip || code == 0x00 || code == 0x04 || jump {
                        self.player.moving();
                    }
    
                }
            }

        }
        
    }

    #[deprecated]
    pub(crate) fn player_movement(&mut self, context: &mut GameContext) {
        if self.player.moving {
            if (self.player.position.direction) == Direction::Up {
                self.player.position.y_offset -= self.player.speed as i8;
                if self.player.position.y_offset <= -16 {
                    self.player.position.y -= 1;
                    self.player.position.y_offset = 0;
                    self.stop_player(context);
                }
            }

            if (self.player.position.direction) == Direction::Down {
                self.player.position.y_offset += self.player.speed as i8;
                if self.player.position.y_offset >= 16 {
                    self.player.position.y += 1;
                    self.player.position.y_offset = 0;
                    self.stop_player(context);
                }
            }
            if (self.player.position.direction) == Direction::Left {
                self.player.position.x_offset -= self.player.speed as i8;
                if self.player.position.x_offset <= -16 {
                    self.player.position.x -= 1;
                    self.player.position.x_offset = 0;
                    self.stop_player(context);
                }
            }

            if (self.player.position.direction) == Direction::Right {
                self.player.position.x_offset += self.player.speed as i8;
                if self.player.position.x_offset >= 16 {
                    self.player.position.x += 1;
                    self.player.position.x_offset = 0;
                    self.stop_player(context);
                }
            }
            self.player.move_update();
        }
    }

    #[deprecated]
    fn stop_player(&mut self, context: &mut GameContext) {
        self.player.moving = false;
        self.player.on_stopped_moving();
        let x = self.player.position.x;
        let y = self.player.position.y;
        if self.chunk_map.is_alive() {
            if self.chunk_map.in_bounds(x, y) {
                self.chunk_map.on_tile(context, x, y);
            }
        } else {
            let map_set = self.map_sets.map_set_mut();
            if map_set.in_bounds(x, y) {
                map_set.on_tile(context, x, y);
            }
        }
        
        //self.on_finished_moving(context);
        
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

        let palette_sizes = fill_palette_map(&mut bottom_sheets, &mut top_sheets, &self.world_id);

        load_maps(&self.world_id, &palette_sizes, &mut self.chunk_map, &mut self.map_sets);

        load_npc_textures(&self.world_id, &mut self.npc_textures);

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
        self.player.load_textures(player_data.world_id.as_str());
        self.player.load();
        if player_data.location.map_id.as_str().eq("world") {
            self.chunk_map.spawn();
        } else {
            self.map_sets.spawn();
            self.load_map_set(&player_data.location.map_id, player_data.location.map_index);
        }
    }

    // pub(crate) fn bind_music(&self, context: &mut GameContext) {
        
    //     // music::bind_music_file(Music::ViridianForest, asset_as_pathbuf("audio/music/mus_viridian_forest.mid"));
    //     // music::bind_music_file(Music::MountMoon, asset_as_pathbuf("audio/music/mus_mt_moon.mid"));
    //     // music::bind_music_file(Music::Route1, asset_as_pathbuf("audio/music/route1.mid"));
    //     // music::bind_music_file(Music::Route2, asset_as_pathbuf("audio/music/route2.mid"));
    //     // music::bind_music_file(Music::Route3, asset_as_pathbuf("audio/music/route3.mid"));
    //     // music::bind_music_file(Music::Route4, asset_as_pathbuf("audio/music/route4.mid"));
    //     // music::bind_music_file(Music::Fuchsia, asset_as_pathbuf("audio/music/mus_fuchsia.mid"));
    //     // music::bind_music_file(Music::Celadon, asset_as_pathbuf("audio/music/mus_celadon.mid"));
    //     // music::bind_music_file(Music::Pewter, asset_as_pathbuf("audio/music/mus_pewter.mid"));
    //     // music::bind_music_file(Music::Lavender, asset_as_pathbuf("audio/music/mus_lavender.mid"));
    //     // music::bind_music_file(Music::Cinnabar, asset_as_pathbuf("audio/music/mus_cinnabar.mid"));
    //     // music::bind_music_file(Music::Pallet, asset_as_pathbuf("audio/music/mus_pallet.mid"));
    //     // music::bind_music_file(Music::Vermilion, asset_as_pathbuf("audio/music/mus_vermilion.mid"));
    //     // music::bind_music_file(Music::Gym, asset_as_pathbuf("audio/music/mus_gym.mid"));
    // }
    
}