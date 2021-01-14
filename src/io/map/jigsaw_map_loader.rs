use std::path::Path;

use log::warn;

use crate::world::world_chunk::WorldChunk;
use crate::world::world_map::WorldMap;

use super::gba_map::fix_tiles;
use super::gba_map::get_gba_map;
use super::map_serializable::MapConfig;
use super::npc_loader::get_npcs;
use super::warp_loader::get_warps;
use super::wild_entry_loader;

pub fn new_jigsaw_map<P: AsRef<Path>>(
    path: P,
    palette_sizes: &Vec<u16>,
    config: &MapConfig,
) -> Option<(u16, WorldChunk)> {
    let path = path.as_ref();

    let map_path = path.join(&config.identifier.map_files[0]);

    match map_path.extension() {
        Some(ext_os_str) => {
            let ext = ext_os_str.to_str().unwrap();
            if ext.eq("map") {
                let mut gba_map = get_gba_map(map_path);
                fix_tiles(&mut gba_map, palette_sizes);

                if let Some(jigsaw_map) = &config.jigsaw_map {
                    return Some((
                        jigsaw_map.piece_index,
                        WorldChunk {
                            x: jigsaw_map.x,
                            y: jigsaw_map.y,
                            map: WorldMap {
                                name: config.identifier.name(),
                                music: gba_map.music,
    
                                width: gba_map.width as u16,
                                height: gba_map.height as u16,
    
                                tile_map: gba_map.tile_map,
                                border_blocks: gba_map.border_blocks,
                                movement_map: gba_map.movement_map,
                                
                                warps: get_warps(path, None),
                                npcs: get_npcs(path, None),
                                wild: wild_entry_loader::load_wild_entry(path, &config, None),
                            },
                            connections: jigsaw_map.connections.clone(),
                        }
                        
                    ));
                } else {
                    return None;
                }
            } else if ext.eq("json") {
                warn!("JSON map found under {:?}, not implemented yet!", path);
                return None;
            } else {
                warn!(
                    "Map file with unknown extension provided under {:?}",
                    path
                );
                return None;
            }
        }
        None => {
            warn!("Map file without an extension found under {:?}", path);
            return None;
        }
    }
}
