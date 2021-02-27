use std::path::PathBuf;

use macroquad::prelude::warn;
use ahash::AHashMap as HashMap;
use crate::audio::music::Music;
use crate::world::map::WorldMap;
use crate::world::map::chunk::WorldChunk;
use super::gba_map::fix_tiles;
use super::gba_map::get_gba_map;
use super::map_serializable::MapConfig;

pub fn new_chunk_map(root_path: &PathBuf, palette_sizes: &HashMap<u8, u16>, config: MapConfig) -> Option<(u16, WorldChunk)> {
    macroquad::prelude::debug!("Loading chunk map {}", &config.identifier.name);
    let map_path = root_path.join(&config.identifier.map_files[0]);
    match map_path.extension() {
        Some(ext) => {
            if ext.to_string_lossy().eq("map") {
                match crate::io::get_file(&map_path) {
                    Some(map_file) => {
                        let mut gba_map = get_gba_map(map_file);
                        fix_tiles(&mut gba_map, palette_sizes);

                        if let Some(jigsaw_map) = config.jigsaw_map {
                            return Some((
                                jigsaw_map.piece_index,
                                WorldChunk {
                                    x: jigsaw_map.x,
                                    y: jigsaw_map.y,
                                    map: WorldMap {
        
                                        name: config.identifier.name,
                                        music: Music::from(gba_map.music),
            
                                        width: gba_map.width as u16,
                                        height: gba_map.height as u16,
            
                                        tile_map: gba_map.tile_map,
                                        border_blocks: gba_map.border_blocks,
                                        movement_map: gba_map.movement_map,

                                        fly_position: config.fly_position,
                                        
                                        wild: super::load_wild_entry(root_path, config.wild, None),
                                        
                                        warps: super::load_warp_entries(root_path, None),
                                        npc_manager: super::load_npc_entries(root_path, None),
                                        script_manager: super::load_script_entries(root_path, None),

                                    },
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
                warn!("Could not find map {} at path {:?}", &config.identifier.map_files[0], &root_path);
                return None;
            }
        }
        None => {
            warn!("Map file at {:?} has unsupported extension!", &map_path);
            return None;
        }
    }
}
