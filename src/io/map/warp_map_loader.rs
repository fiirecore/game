use std::collections::HashMap;
use std::path::Path;

use crate::game::world::warp_map::warp_map::WarpMap;
use crate::game::world::warp_map::warp_map_set::WarpMapSet;

use super::gba_map::fix_tiles;
use super::gba_map::fix_tiles2;
use super::gba_map::get_gba_map;
use super::gba_map::get_offset;
use super::map_loader::NULL_TILE_ID;
use super::map_serializable::MapConfig;
use super::npc_loader::get_npcs;
use super::warp_loader::get_warps;

pub fn new_warp_map<P: AsRef<Path>>(
    root_path: P,
    palette_sizes: &Vec<u16>,
    config: &MapConfig,
) -> Option<(String, WarpMapSet)> {
    
    let root_path = root_path.as_ref();

    let mut maps: HashMap<usize, WarpMap> = HashMap::new();

    for index in 0..config.identifier.map_files.len() {

        let map_path = root_path.join(&config.identifier.map_files[index]);

        let mut gba_map = get_gba_map(map_path);

        fix_tiles(&mut gba_map, palette_sizes);

        let offset = get_offset(&gba_map, &palette_sizes);

        let mut border_blocks = gba_map.border_blocks;
        let mut tile_map = gba_map.tile_map;
        let map12 = gba_map.palettes[0] == 12;

        if map12 {
            let mut offset12: u16 = 0;

            for x in 0..12 {
                offset12 += palette_sizes[x];
            }

            for index in 0..tile_map.len() {
                if tile_map[index] < palette_sizes[0] && tile_map[index] != NULL_TILE_ID {
                    tile_map[index] += offset12;
                }
            }

            for index in 0..border_blocks.len() {
                if border_blocks[index] < palette_sizes[0] {
                    border_blocks[index] += offset12;
                }
            }
        }

        // fix below

        let mut map_ = String::from("map_");
        map_.push_str(index.to_string().as_str());

        let wildtomlpath = root_path.clone().join("wild").join(map_).join("grass.toml");
        let mut wild_tiles;
        let table;

        if let Some(settings) = &config.settings {
            wild_tiles = settings.wild_tiles.clone();
            fix_tiles2(wild_tiles.as_mut().unwrap(), offset, palette_sizes[0]);
            table = crate::game::world::pokemon::wild_pokemon_table::get(
                settings.encounter_type.as_ref().unwrap_or(&String::new()).clone(),
                &wildtomlpath,
            );
        } else {
            wild_tiles = None;
            table = None;
        }

        //

        maps.insert(
            index,
            WarpMap {
                music: gba_map.music,
                width: gba_map.width,
                height: gba_map.height,
                tile_map: tile_map,
                border_blocks: border_blocks,
                movement_map: gba_map.movement_map,
                warps: get_warps(&root_path, Some(index)),
                npcs: get_npcs(&root_path, Some(index)),
                wild_tiles: wild_tiles,
                wild_pokemon_table: table,
            },
        );
    }

    return Some((
        config.warp_map.as_ref().unwrap().map_set_id.clone(),
        WarpMapSet {
            maps: maps,
            current_map_index: 0,
        },
    ));
}
