use std::path::PathBuf;

use crate::world::pokemon::WildEntry;
use crate::world::pokemon::wild_pokemon_table;

use super::map_serializable::SerializedWildEntry;

pub fn load_wild_entry(root_path: &PathBuf, wild: Option<SerializedWildEntry>, map_set_index: Option<usize>) -> Option<WildEntry> {
    if let Some(toml_wild_entry) = wild {
        let mut wild_path = root_path.join("wild");

        if let Some(map_set_index) = map_set_index {
            wild_path = wild_path.join(String::from("map_") + &map_set_index.to_string());
        }

        Some(WildEntry {
            tiles: toml_wild_entry.wild_tiles,
            table: wild_pokemon_table::get(toml_wild_entry.encounter_type.as_str(), wild_path.join("grass.toml")),
        })

    } else {
        return None;
    }   
}
