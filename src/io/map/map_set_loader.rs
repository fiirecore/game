use macroquad::prelude::warn;

use crate::audio::music::Music;
use crate::world::map::WorldMap;
use crate::world::map::set::world_map_set::WorldMapSet;

use super::gba_map::fix_tiles;
use super::gba_map::get_gba_map;
use super::map_serializable::MapConfig;
use super::npc_loader::load_npc_entries;
use super::warp_loader::load_warp_entries;
use super::wild_entry_loader::load_wild_entry;

pub fn new_map_set(root_path: &include_dir::Dir, palette_sizes: &Vec<u16>, config: MapConfig) -> Option<(String, WorldMapSet)> {

    let mut maps: Vec<WorldMap> = Vec::new();

    for index in 0..config.identifier.map_files.len() {

        match root_path.get_file(root_path.path().join(&config.identifier.map_files[index])) {
            Some(file) => {
                let mut gba_map = get_gba_map(file);
                fix_tiles(&mut gba_map, palette_sizes);

                maps.insert(
                    index,
                    WorldMap {
                        name: config.identifier.name(),
                        music: Music::from(gba_map.music),
                        width: gba_map.width,
                        height: gba_map.height,
                        tile_map: gba_map.tile_map,
                        border_blocks: gba_map.border_blocks,
                        movement_map: gba_map.movement_map,
                        warps: load_warp_entries(&root_path, Some(index)),
                        npcs: load_npc_entries(&root_path, Some(index)),
                        wild: load_wild_entry(&root_path, config.wild.clone(), Some(index)),
                    },
                );
            }
            None => {
                warn!("Could not get map at path {}/{}", root_path.path, &config.identifier.map_files[index]);
                return None;
            }
        }
    }

    return Some((
        config.warp_map.unwrap().map_set_id,
        WorldMapSet::new(maps),
    ));
}
