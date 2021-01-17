use std::path::Path;


use crate::world::pokemon::WildEntry;
use crate::world::pokemon::wild_pokemon_table;

use super::map_serializable::SerializedWildEntry;

pub fn load_wild_entry<P: AsRef<Path>>(path: P, wild: Option<SerializedWildEntry>, map_set_num: Option<usize>) -> Option<WildEntry> {
    let path = path.as_ref();

    // fix below

    let mut wildtomlpath = path.clone().join("wild");
    if let Some(map_set_num) = map_set_num {
        let mut map_id = String::from("map_");
        map_id.push_str(map_set_num.to_string().as_str());
        wildtomlpath = wildtomlpath.join(map_id);
    }

    if let Some(toml_wild_entry) = wild {
        Some(WildEntry {
            tiles: toml_wild_entry.wild_tiles,
            table: wild_pokemon_table::get(toml_wild_entry.encounter_type.as_str(), wildtomlpath.join("grass.toml")),
        })
    } else {
        None
    }
}
