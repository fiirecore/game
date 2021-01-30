use macroquad::prelude::warn;

use crate::world::pokemon::WildEntry;
use crate::world::pokemon::wild_pokemon_table;

use super::map_serializable::SerializedWildEntry;

pub fn load_wild_entry(root_path: &include_dir::Dir, wild: Option<SerializedWildEntry>, map_set_num: Option<usize>) -> Option<WildEntry> {
    if let Some(toml_wild_entry) = wild {
        match root_path.get_dir(root_path.path().join("wild")) {
            Some(mut wild_dir) => {
                if let Some(map_set_num) = map_set_num {
                    let mut map_id = String::from("map_");
                    map_id.push_str(map_set_num.to_string().as_str());
                    wild_dir = wild_dir.get_dir(wild_dir.path().join(map_id)).unwrap(); // Fix this
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
