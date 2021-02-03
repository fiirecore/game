use macroquad::prelude::warn;

use crate::world::pokemon::WildEntry;
use crate::world::pokemon::wild_pokemon_table;

use super::map_serializable::SerializedWildEntry;

pub fn load_wild_entry(root_path: &include_dir::Dir, wild: Option<SerializedWildEntry>, map_set_index: Option<usize>) -> Option<WildEntry> {
    if let Some(toml_wild_entry) = wild {
        match root_path.get_dir(root_path.path().join("wild")) {
            Some(mut wild_dir) => {
                if let Some(map_set_index) = map_set_index {
                    match wild_dir.get_dir(wild_dir.path().join(String::from("map_") + &map_set_index.to_string())) {
                        Some(dir) => wild_dir = dir,
                        None => {
                            warn!("Could not get map set #{} for wild directory under {}", map_set_index, wild_dir.path);
                            return None;
                        }
                    }
                }
                Some(WildEntry {
                    tiles: toml_wild_entry.wild_tiles,
                    table: wild_pokemon_table::get(toml_wild_entry.encounter_type.as_str(), wild_dir.get_file(wild_dir.path().join("grass.toml"))),
                })
            }
            None => {
                warn!("Could not get wild directory for path {}", root_path.path);
                return None;
            }
        }
    } else {
        return None;
    }   
}
