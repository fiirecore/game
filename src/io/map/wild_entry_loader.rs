use std::path::Path;


use crate::world::pokemon::WildEntry;
use crate::world::pokemon::wild_pokemon_table;

use super::map_serializable::MapConfig;

pub fn load_wild_entry<P: AsRef<Path>>(path: P, config: &MapConfig, map_set_num: Option<usize>) -> Option<WildEntry> {
    let path = path.as_ref();

    // fix below

    let wildtomlpath = path.clone();
    if let Some(map_set_num) = map_set_num {
        let mut map_id = String::from("map_");
        map_id.push_str(map_set_num.to_string().as_str());
    }
    let wildtomlpath = wildtomlpath.join("wild").join("grass.toml");
    if let Some(toml_wild_entry) = &config.wild {
        match wild_pokemon_table::get(toml_wild_entry.encounter_type.clone(), &wildtomlpath) {
            Some(table) => {
                Some(WildEntry {
                    tiles: toml_wild_entry.wild_tiles.clone(),
                    table: table,
                })
            }
            None => {
                None
            }
        }
    } else {
        None
    }

}
