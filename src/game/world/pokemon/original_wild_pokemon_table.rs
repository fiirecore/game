use std::{ffi::OsStr, path::Path};
use log::warn;
use serde_derive::Deserialize;

use crate::{engine::game_context::GameContext, game::pokedex::{pokedex::Pokedex, pokemon::pokemon_instance::PokemonInstance}, util::file_util::UNKNOWN_FILENAME_ERR};

use super::{wild_pokemon_encounter::WildPokemonEncounter, wild_pokemon_table::WildPokemonTable};

#[derive(Copy, Clone)]
pub struct OriginalWildPokemonTable {

    pub encounter_ratio: u8,
    pub table: [WildPokemonEncounter; 12],

}

impl OriginalWildPokemonTable {

    pub fn from_toml<P>(path: P) -> Option<OriginalWildPokemonTable> where P: AsRef<Path> {
        let path = path.as_ref();

        let filename = path.parent().unwrap().parent().unwrap().file_name().unwrap_or(&OsStr::new(UNKNOWN_FILENAME_ERR));

        let content_result = std::fs::read_to_string(path);

        match content_result {
            Ok(content) => {
                let toml_result: Result<WildPokemonTableInToml, toml::de::Error> = toml::from_str(content.as_str());
                match toml_result {
                    Ok(toml_table) => {
                        
                        Some(OriginalWildPokemonTable {
                            encounter_ratio: toml_table.encounter_ratio,
                            table: WildPokemonEncounter::fill_table(&toml_table),
                        })
                    },
                    Err(e) => {
                        warn!("Could not parse wild pokemon table in {:?} with error {}", filename, e);
                        return None;
                    }
                }
            },
            Err(err) => {
                warn!("Could not read wild pokemon table at {:?} to string with error: {}", filename, err);
                return None;
            }
        }

    }

}

impl WildPokemonTable for OriginalWildPokemonTable {

    fn encounter_rate(&self) -> u8 {
        return self.encounter_ratio;
    }

    fn generate(&mut self, pokedex: &Pokedex, context: &mut GameContext) -> PokemonInstance {
        let chance = context.random.rand_range(1..100) as usize;
        let mut chance_counter = 0;
        let mut counter = 0;
        while chance > chance_counter {
            chance_counter += CHANCES[counter];
            counter+=1;            
        }
        return self.table[counter - 1].generate_instance(pokedex, context);
    }

}

impl WildPokemonEncounter {

    pub fn empty() -> WildPokemonEncounter {
        Self {
            min_level: 0,
            max_level: 0,
            pokemon_id: 0,
        }
    }

    pub fn from_toml(toml_table: TomlWildPokemon) -> WildPokemonEncounter {
        WildPokemonEncounter {
            min_level: toml_table.min_level,
            max_level: toml_table.max_level,
            pokemon_id: toml_table.pokemon_id,
        }
    }

    fn fill_table(toml_table: &WildPokemonTableInToml) -> [WildPokemonEncounter; 12] {
        let mut arr: [WildPokemonEncounter; 12] = [WildPokemonEncounter::empty(); 12];
        for i in 0..12 {
            arr[i] = WildPokemonEncounter::from_toml(toml_table.encounter[i]);
        }
        return arr;
    }

}

pub static CHANCES: [usize; 12] = [20, 20, 10, 10, 10, 10, 5, 5, 4, 4, 1, 1];

#[derive(Debug, Deserialize)]
pub struct WildPokemonTableInToml {

    pub encounter_ratio: u8,
    pub encounter: Vec<TomlWildPokemon>,

}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct TomlWildPokemon {

    min_level: u8,
    max_level: u8,
    pokemon_id: usize,

}

