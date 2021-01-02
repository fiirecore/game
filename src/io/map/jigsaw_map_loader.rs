use std::path::Path;

use crate::game::world::world_map::world_map_piece::WorldMapPiece;

use super::gba_map::fix_tiles;
use super::gba_map::get_gba_map;
use super::map_serializable::MapConfig;
use super::npc_loader::get_npcs;
use super::warp_loader::get_warps;

pub fn new_jigsaw_map<P: AsRef<Path>>(
    path: P,
    palette_sizes: &Vec<u16>,
    config: &MapConfig,
) -> Option<(usize, WorldMapPiece)> {
    let path = path.as_ref();

    let map_path = path.join(&config.identifier.map_files[0]);

    match map_path.extension() {
        Some(ext_os_str) => {
            let ext = ext_os_str.to_str().unwrap();
            if ext.eq("map") {
                let mut gba_map = get_gba_map(map_path);
                fix_tiles(&mut gba_map, palette_sizes);

                // fix below

                let wildtomlpath = path.clone().join("wild").join("grass.toml");
                let wild_tiles;
                let table;

                if let Some(settings) = &config.settings {
                    wild_tiles = settings.wild_tiles.clone();
                    table = crate::game::world::pokemon::wild_pokemon_table::get(
                        settings.encounter_type.as_ref().unwrap_or(&String::new()).clone(),
                        &wildtomlpath,
                    );
                } else {
                    wild_tiles = None;
                    table = None;
                }

                //
                

                if let Some(jigsaw_map) = &config.jigsaw_map {
                    return Some((
                        jigsaw_map.piece_index,
                        WorldMapPiece {
                            name: gba_map.name,
                            music: gba_map.music,

                            x: jigsaw_map.x,
                            y: jigsaw_map.y,

                            width: gba_map.width as u16,
                            height: gba_map.height as u16,

                            tile_map: gba_map.tile_map,
                            border_blocks: gba_map.border_blocks,
                            movement_map: gba_map.movement_map,
                            connections: jigsaw_map.connections.clone(),
                            warps: get_warps(path, None),
                            npcs: get_npcs(path, None),

                            wild_tiles: wild_tiles,
                            wild_pokemon_table: table,
                        },
                    ));
                } else {
                    return None;
                }
            } else if ext.eq("json") {
                println!("Json map found under {:?}, not implemented yet!", path);
                return None;
            } else {
                println!(
                    "Error: Map file with unknown extension provided under {:?}",
                    path
                );
                return None;
            }
        }
        None => {
            println!("Map file without an extension found under {:?}", path);
            return None;
        }
    }
}
