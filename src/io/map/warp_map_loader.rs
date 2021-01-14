use std::path::Path;

use crate::world::world_map::WorldMap;
use crate::world::world_map_set::WorldMapSet;

use super::gba_map::fix_tiles;
use super::gba_map::get_gba_map;
use super::map_serializable::MapConfig;
use super::npc_loader::get_npcs;
use super::warp_loader::get_warps;
use super::wild_entry_loader;

pub fn new_warp_map<P: AsRef<Path>>(
    root_path: P,
    palette_sizes: &Vec<u16>,
    config: &MapConfig,
) -> Option<(String, WorldMapSet)> {
    
    let root_path = root_path.as_ref();

    let mut maps: Vec<WorldMap> = Vec::new();

    for index in 0..config.identifier.map_files.len() {

        let mut gba_map = get_gba_map(root_path.join(&config.identifier.map_files[index]));

        fix_tiles(&mut gba_map, palette_sizes);

        maps.insert(
            index,
            WorldMap {
                name: config.identifier.name(),
                music: gba_map.music,
                width: gba_map.width,
                height: gba_map.height,
                tile_map: gba_map.tile_map,
                border_blocks: gba_map.border_blocks,
                movement_map: gba_map.movement_map,
                warps: get_warps(&root_path, Some(index)),
                npcs: get_npcs(&root_path, Some(index)),
                wild: wild_entry_loader::load_wild_entry(&root_path, &config, Some(index)),
            },
        );
    }

    return Some((
        config.warp_map.as_ref().unwrap().map_set_id.clone(),
        WorldMapSet::new(maps),
    ));
}
