use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;

use image::RgbaImage;

use crate::audio::music::Music;
use crate::entity::entities::player::Player;
use crate::entity::texture::still_texture_manager::StillTextureManager;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::io::data::player_data::PlayerData;
use crate::util::file_util::asset_as_pathbuf;
use crate::io::map::gba_map::{fill_palette_map, get_texture};
use crate::util::texture_util::texture_from_path;
use crate::util::traits::Loadable;

use super::world_manager::PALETTE_COUNT;
use super::world_manager::WorldManager;

impl WorldManager {

    pub fn new_world_map(&mut self, world_id: &String) {
        self.world_map_manager.new_world_map();
        self.world_id = world_id.clone();
        //self.load_world(player_data);
        //self.world_map_manager.worlds.insert(world_id.clone(), WorldMap::new());
        //self.world_map_manager.current_world_id = world_id.clone();
    }

    pub fn load_world_map(&mut self) {
        self.world_map_manager.get_current_world_mut().current_piece_index = self.world_map_manager.get_current_world().map_index_at_coordinates(self.player.coords.x, self.player.coords.y).unwrap_or(0);
    }

    pub(crate) fn bind_music(&self) {
        music::bind_music_file(Music::MountMoon, asset_as_pathbuf("audio/music/mus_mt_moon.mid"));
        music::bind_music_file(Music::Route1, asset_as_pathbuf("audio/music/route1.mid"));
        music::bind_music_file(Music::Route2, asset_as_pathbuf("audio/music/route2.mid"));
        music::bind_music_file(Music::Route3, asset_as_pathbuf("audio/music/route3.mid"));
        music::bind_music_file(Music::Route4, asset_as_pathbuf("audio/music/route4.mid"));
        music::bind_music_file(Music::Fuchsia, asset_as_pathbuf("audio/music/mus_fuchsia.mid"));
        music::bind_music_file(Music::Celadon, asset_as_pathbuf("audio/music/mus_celadon.mid"));
        music::bind_music_file(Music::Pewter, asset_as_pathbuf("audio/music/mus_pewter.mid"));
        music::bind_music_file(Music::Lavender, asset_as_pathbuf("audio/music/mus_lavender.mid"));
        music::bind_music_file(Music::Cinnabar, asset_as_pathbuf("audio/music/mus_cinnabar.mid"));
        music::bind_music_file(Music::Pallet, asset_as_pathbuf("audio/music/mus_pallet.mid"));
        music::bind_music_file(Music::Vermilion, asset_as_pathbuf("audio/music/mus_vermilion.mid"));
        music::bind_music_file(Music::Gym, asset_as_pathbuf("audio/music/mus_gym.mid"));
    }

    pub(crate) fn load_maps(&mut self, world_id: &String) {

        let mut dir_pb = PathBuf::from("worlds/");
        dir_pb.push(world_id);
        dir_pb.push("maps");

        //println!("{:?}", dir_pb.clone());
    
        let entries = std::fs::read_dir(asset_as_pathbuf(dir_pb)).unwrap().map( |res| res.map(|e| e.path()));
    
        for dir_entry in entries {
            match dir_entry {
                Ok(dir_entry) => {
                    let dir = dir_entry.read_dir().unwrap().map( |res| res.map(|e| e.path()));
                    for subdir_entry in dir {
                        match subdir_entry {
                            Ok(p) => {
                                if let Some(ext) = p.extension() {
                                    if ext == OsString::from("toml") {
                                        let maps = crate::io::map::map_loader::map_from_toml(&self.palette_sizes, p);
                                        if let Some(map_data) = maps.1 {
                                            //if map_data.0.eq(&self.player_data.current_world_id) {
                                                self.world_map_manager.get_current_world_mut().pieces.insert(map_data.0, map_data.1);
                                            //}
                                        } else if let Some(warp_map_set) = maps.2 {
                                            //if warp_map_set.0.eq(&self.player_data.current_world_id) {
                                                self.warp_map_manager.map_sets.insert(warp_map_set.0, warp_map_set.1);
                                            //}
                                        }
                                        
                                    }
                                }
                            }
                            Err(e) => {
                                //println!("Error parsing directory: {:?}", subdir_entry);
                                println!("{}", e);
                            }
                        }
                    }
                                        
                }
                Err(e) => {
                    //println!("error getting map set files under {:?}", dir_entry);
                    println!("{}", e);
                }
            }
        }

    }

    pub(crate) fn load_npcs(&mut self, world_id: &String) {
        let mut dir_pb = PathBuf::from("worlds/");
        dir_pb.push(world_id);
        dir_pb.push("textures");
        dir_pb.push("npcs");

        //println!("{:?}", dir_pb.clone());
    
        let entries_result = std::fs::read_dir(asset_as_pathbuf(dir_pb));
        match entries_result {
            Ok(readdir) => {
                let paths: Vec<Result<PathBuf, std::io::Error>> = readdir.map( |res| res.map(|e| e.path())).collect();
                let size = paths.len();
                for path in paths {
                    match path {
                        Ok(path) => {
                            if path.is_dir() {
                                let mut twt = ThreeWayTexture::new();
                                if size > 3 {
                                    println!("Moving NPC textures found, not supported yet.");
                                } else {
                                    twt.add_texture_manager(Box::new(StillTextureManager::new(texture_from_path(&path.join("idle_up.png")), false)));
                                    twt.add_texture_manager(Box::new(StillTextureManager::new(texture_from_path(&path.join("idle_down.png")), false)));
                                    twt.add_texture_manager(Box::new(StillTextureManager::new(texture_from_path(&path.join("idle_side.png")), false)));
                                }                                    
                                self.npc_textures.insert(path.file_name().unwrap().to_str().unwrap().parse::<u8>().expect("Found a folder with a non-number name"), twt); // fix
                            }
                        },
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                }
            },
            Err(err) => {
                println!("Error reading NPC textures directory for map {} with error: {}", world_id, err);
            },
        }
    }

    pub fn populate_textures(&mut self) {

        let mut bottom_sheets: HashMap<u8, RgbaImage> = HashMap::new();
        let mut top_sheets: HashMap<u8, RgbaImage> = HashMap::new();
        fill_palette_map(&mut bottom_sheets, &mut top_sheets, &self.world_id, PALETTE_COUNT);

        for wmap in self.world_map_manager.get_current_world().pieces.values() {
            for tile_id in &wmap.tile_map {
                if !(self.bottom_textures.contains_key(tile_id) && self.top_textures.contains_key(tile_id)) {
                    self.top_textures.insert(*tile_id, get_texture(&top_sheets, &self.palette_sizes, *tile_id));
                    self.bottom_textures.insert(*tile_id, get_texture(&bottom_sheets, &self.palette_sizes, *tile_id));
                }
            }
            for tile_id in &wmap.border_blocks {
                if !(self.bottom_textures.contains_key(tile_id) && self.top_textures.contains_key(tile_id)) {
                    self.bottom_textures.insert(*tile_id, get_texture(&bottom_sheets, &self.palette_sizes, *tile_id));
                    self.top_textures.insert(*tile_id, get_texture(&top_sheets, &self.palette_sizes, *tile_id));
                }
            }
        }
        for wmapset in self.warp_map_manager.map_sets.values() {
            for wmap in wmapset.maps.values() {
                for tile_id in &wmap.tile_map {
                    if !(self.bottom_textures.contains_key(tile_id) && self.top_textures.contains_key(tile_id)) {
                        self.top_textures.insert(*tile_id, get_texture(&top_sheets, &self.palette_sizes, *tile_id));
                        self.bottom_textures.insert(*tile_id, get_texture(&bottom_sheets, &self.palette_sizes, *tile_id));
                    }
                }
                for tile_id in &wmap.border_blocks {
                    if !(self.bottom_textures.contains_key(tile_id) && self.top_textures.contains_key(tile_id)) {
                        self.bottom_textures.insert(*tile_id, get_texture(&bottom_sheets, &self.palette_sizes, *tile_id));
                        self.top_textures.insert(*tile_id, get_texture(&top_sheets, &self.palette_sizes, *tile_id));
                    }
                }
            }
        }

    }

    pub fn load_player(&mut self, player_data: &PlayerData) {
        self.player = Player::new(player_data);
        self.player.load_textures(player_data.location.world_id.as_str());
        self.player.load();
    }

}