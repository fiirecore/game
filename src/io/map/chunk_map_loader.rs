use macroquad::prelude::warn;
use ahash::AHashMap as HashMap;
use crate::audio::music::Music;
use crate::world::map::WorldMap;
use crate::world::map::chunk::world_chunk::WorldChunk;
use super::gba_map::fix_tiles;
use super::gba_map::get_gba_map;
use super::map_serializable::MapConfig;
use super::npc_loader::load_npc_entries;
use super::warp_loader::load_warp_entries;
use super::wild_entry_loader::load_wild_entry;

pub fn new_chunk_map(root_path: &include_dir::Dir, palette_sizes: &HashMap<u8, u16>, config: MapConfig) -> Option<(u16, WorldChunk)> {
    match root_path.get_file(root_path.path().join(&config.identifier.map_files[0])) {
        Some(map_file) => {
            match map_file.path().extension() {
                Some(ext) => {
                    if ext.to_string_lossy().eq("map") {
                        let mut gba_map = get_gba_map(map_file);
                        fix_tiles(&mut gba_map, palette_sizes);

                        if let Some(jigsaw_map) = &config.jigsaw_map {
                            return Some((
                                jigsaw_map.piece_index,
                                WorldChunk {
                                    x: jigsaw_map.x,
                                    y: jigsaw_map.y,
                                    map: WorldMap {
        
                                        name: config.identifier.name(),
                                        music: Music::from(gba_map.music),
            
                                        width: gba_map.width as u16,
                                        height: gba_map.height as u16,
            
                                        tile_map: gba_map.tile_map,
                                        border_blocks: gba_map.border_blocks,
                                        movement_map: gba_map.movement_map,
                                        
                                        warps: load_warp_entries(root_path, None),
                                        npcs: load_npc_entries(root_path, None),
                                        wild: load_wild_entry(root_path, config.wild, None),

                                        ..Default::default()        
                                    },
                                    connections: jigsaw_map.connections.clone(),
                                }
                                
                            ));
                        } else {
                            return None;
                        }
                    } else {
                        warn!("Map file at {} has unsupported extension!", map_file.path);
                        return None;
                    }
                }
                None => {
                    return None;
                }
            }
        }
        None => {
            warn!("Could not find map {} at path {}", &config.identifier.map_files[0], root_path.path);
            return None;
        }
    }
}
