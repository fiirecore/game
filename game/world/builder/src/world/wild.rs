use std::path::PathBuf;

use firecore_world_lib::map::wild::{WildEntry, table::WildPokemonTable};

use crate::world::SerializedWildEntry;

pub fn load_wild_entry(wild: Option<SerializedWildEntry>, wild_path: PathBuf) -> Option<WildEntry> {
    wild.map(|serialized_wild_entry| {

        let file = wild_path.join("grass.toml");

        let table = match serialized_wild_entry.encounter_type.as_str() {
            "original" => {
                match std::fs::read_to_string(&file) {
                    Ok(content) => {
                        match toml::from_str(&content) {
                            Ok(table) => table,
                            Err(err) => {
                                panic!("Could not parse wild pokemon table at {:?} with error {}", &file, err);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Could not find wild toml file at {:?} with error {}", file, err);
                        WildPokemonTable::default()
                    }
                }
            }
            _ => {
                WildPokemonTable::default()
            }
        };

        WildEntry {
            tiles: serialized_wild_entry.tiles,
            table: table,
        }

    })
}