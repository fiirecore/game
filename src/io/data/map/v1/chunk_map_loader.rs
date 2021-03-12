use std::path::PathBuf;

use macroquad::prelude::warn;
use ahash::AHashMap as HashMap;
use super::gba_map::fix_tiles;
use super::gba_map::get_gba_map;
use super::map_serializable::MapConfig;
use firecore_world::map::chunk::WorldChunk;

pub fn new_chunk_map(root_path: &PathBuf, palette_sizes: &HashMap<u8, u16>, config: MapConfig) -> Option<(u16, WorldChunk)> {
    macroquad::prelude::debug!("Loading chunk map {}", &config.identifier.name);
    if let Some(map_file) = config.identifier.map_files.get(0) {
        let map_path = root_path.join(map_file);
        match map_path.extension() {
            Some(ext) => {
                if ext.to_string_lossy().eq("map") {
                    match crate::io::get_file(&map_path) {
                        Some(map_file) => {
                            let mut gba_map = get_gba_map(map_file);
                            fix_tiles(&mut gba_map, palette_sizes);
    
                            if config.jigsaw_map.is_some() {
                                let map = super::new_world_from_v1(
                                    gba_map, 
                                    &config, 
                                    root_path, 
                                    None
                                );
                                let jigsaw_map = config.jigsaw_map.unwrap();
                                return Some((
                                    jigsaw_map.piece_index,
                                    WorldChunk {
                                        index: jigsaw_map.piece_index,
                                        map,
                                        x: jigsaw_map.x,
                                        y: jigsaw_map.y,
                                        connections: jigsaw_map.connections,
                                    }
                                    
                                ));
                            } else {
                                return None;
                            }
                        }
                        None => return None,
                    }
                } else {
                    warn!("Could not find map {} at path {:?}", &map_file, &root_path);
                    return None;
                }
            }
            None => {
                warn!("Map file at {:?} has unsupported extension!", &map_path);
                return None;
            }
        }
    } else {
        warn!("Map configuration did not specify any map files!");
        return None;
    }
    
}
