use macroquad::prelude::warn;
use ahash::AHashMap as HashMap;
use crate::world::map::chunk::world_chunk::WorldChunk;
use crate::world::map::chunk::world_chunk_map::WorldChunkMap;
use crate::world::map::set::world_map_set::WorldMapSet;
use crate::world::map::set::world_map_set_manager::WorldMapSetManager;
use super::chunk_map_loader::new_chunk_map;
use super::map_serializable::MapConfig;
use super::map_set_loader::new_map_set;

pub fn load_maps(palette_sizes: &HashMap<u8, u16>, chunk_map: &mut WorldChunkMap, map_sets: &mut WorldMapSetManager) {

    match crate::io::ASSET_DIR.get_dir("world/maps") {
        Some(map_dir) => {
            for dir_entry in map_dir.dirs() {
                for file in dir_entry.files {
                    if let Some(ext) = file.path().extension() {
                        if ext == std::ffi::OsString::from("toml") {
                            let maps = map_from_toml(palette_sizes, dir_entry, file);
                            if let Some(world_chunk) = maps.0 {
                                chunk_map.insert(world_chunk.0, world_chunk.1);
                            } else if let Some(map_set) = maps.1 {
                                map_sets.insert(map_set.0, map_set.1);
                            }
                        }
                    }
                }
            }
        }
        None => {
            warn!("Could not get map directory!");
        }
    }
}

pub fn map_from_toml(palette_sizes: &HashMap<u8, u16>, root_path: &include_dir::Dir, file: &include_dir::File) -> (Option<(u16, WorldChunk)>, Option<(String, WorldMapSet)>) {

    match file.contents_utf8() {
        Some(data) => {
            let map_config: Result<MapConfig, toml::de::Error> = toml::from_str(data);
            match map_config {
                Ok(map_config) => {
                    if map_config.jigsaw_map.is_some() {
                        match new_chunk_map(root_path, palette_sizes, map_config) {
                            Some(map) => {
                                return (Some(map), None);
                            }
                            None => {
                                warn!("Error reading jigsaw map at path: {}", root_path.path);
                                return (None, None);
                            }
                        }
                        

                    } else if map_config.warp_map.is_some() {
                        match new_map_set(root_path, palette_sizes, map_config) {
                            Some(map) => {
                                return (None, Some(map));
                            }
                            None => {
                                warn!("Error reading warp map at path: {}", root_path.path);
                                return (None, None);
                            }
                        }

                    } else {

                        warn!("Map config at {} does not contain either a jigsaw map or a warp map.", root_path.path);
                        return (None, None);

                    }
                    
                }
                Err(err) => {
                    warn!(
                        "Toml file at {} is {}",
                        root_path.path,
                        err
                    );

                    return (None, None);
                }
            }
        }
        None => {
            warn!("Error reading file at {} to string with error", root_path.path);
            return (None, None);
        }
    }
}

