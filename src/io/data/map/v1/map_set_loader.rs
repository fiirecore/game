use std::path::PathBuf;

use macroquad::prelude::warn;
use ahash::AHashMap as HashMap;
use frc_audio::music::Music;
use crate::world::map::WorldMap;
use crate::world::map::set::WorldMapSet;
use super::gba_map::fix_tiles;
use super::gba_map::get_gba_map;

pub fn new_map_set(root_path: &PathBuf, palette_sizes: &HashMap<u8, u16>, config: super::map_serializable::MapConfig) -> Option<(String, WorldMapSet)> {

    let name = config.identifier.name;
    
    macroquad::prelude::debug!("Loading map set {}", &name);

    let mut maps: Vec<WorldMap> = Vec::new();

    for index in 0..config.identifier.map_files.len() {

        match crate::io::get_file(root_path.join(&config.identifier.map_files[index])) {
            Some(file) => {
                let mut gba_map = get_gba_map(file);
                fix_tiles(&mut gba_map, palette_sizes);

                maps.insert(
                    index,
                    WorldMap {
                        name: name.clone(),
                        music: Music::from(gba_map.music),
                        width: gba_map.width,
                        height: gba_map.height,
                        tile_map: gba_map.tile_map,
                        border_blocks: gba_map.border_blocks,
                        movement_map: gba_map.movement_map,
                        fly_position: config.settings.fly_position,
                        wild: super::load_wild_entry(&root_path, config.wild.clone(), Some(index)),
                        warps: super::load_warp_entries(&root_path, Some(index)),
                        npc_manager: super::load_npc_entries(&root_path, Some(index)),
                        script_manager: super::load_script_entries(root_path, Some(index)),

                    },
                );
            }
            None => {
                warn!("Could not get map at path {:?}/{}", &root_path, &config.identifier.map_files[index]);
                return None;
            }
        }
    }

    return Some((
        config.warp_map.unwrap().map_set_id,
        WorldMapSet::new(maps),
    ));
}
