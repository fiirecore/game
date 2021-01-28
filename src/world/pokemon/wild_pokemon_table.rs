use std::path::Path;

use macroquad::prelude::warn;

use crate::io::data::pokemon::StatSet;
use crate::io::data::pokemon::saved_pokemon::SavedPokemon;

use super::wild_pokemon_encounter::WildPokemonEncounter;

pub static DEFAULT_ENCOUNTER_CHANCE: u8 = 21;
pub struct WildPokemonTable {

    pub encounter_ratio: u8,
    pub table: Option<[WildPokemonEncounter; 12]>,

}

impl WildPokemonTable {

    pub fn encounter_rate(&self) -> u8 {
        self.encounter_ratio
    }

    pub fn generate(&self) -> SavedPokemon {
        match self.table {
            Some(table) => table[get_counter()].generate_saved(),
            None => return SavedPokemon::generate(
                macroquad::rand::gen_range(0, crate::game::pokedex::pokedex::LENGTH) + 1, 
                1, 
                100, 
                Some(StatSet::iv_random()), 
                None
            ),
        }
    }

}

impl Default for WildPokemonTable {
    fn default() -> Self {
        Self {
            encounter_ratio: DEFAULT_ENCOUNTER_CHANCE,
            table: None,
        }
    }
}

pub async fn get<P: AsRef<Path>>(encounter_type: &str, path: P) -> WildPokemonTable {
    let path = path.as_ref();
    match encounter_type {
        "original" => {
            return from_toml(path).await;
        }
        _ => {
            return WildPokemonTable::default();
        }
    }
}

async fn from_toml<P: AsRef<std::path::Path>>(path: P) -> WildPokemonTable {
    let path = path.as_ref();

    match crate::util::file::read_to_string(path).await {
        Ok(content) => {
            let toml_result: Result<WildPokemonTableInToml, toml::de::Error> = toml::from_str(content.as_str());
            match toml_result {
                Ok(toml_table) => {
                    WildPokemonTable {
                        encounter_ratio: toml_table.encounter_ratio,
                        table: Some(fill_table(&toml_table)),
                    }
                },
                Err(err) => {
                    warn!("Could not parse wild pokemon table at {:?} with error {}, using random table instead!", &path, err);
                    return WildPokemonTable::default();
                }
            }
        },
        Err(err) => {
            warn!("Could not read wild pokemon table at {:?} to string with error {}, using random table instead!", &path, err);
            return WildPokemonTable::default();
        }
    }

}

pub static CHANCES: [usize; 12] = [20, 20, 10, 10, 10, 10, 5, 5, 4, 4, 1, 1];


fn get_counter() -> usize {
    let chance = macroquad::rand::gen_range(1, 100);
    let mut chance_counter = 0;
    let mut counter = 0;
    while chance > chance_counter {
        chance_counter += CHANCES[counter];
        counter+=1;            
    }
    return counter - 1;
}

#[derive(Debug, serde::Deserialize)]
pub struct WildPokemonTableInToml {

    pub encounter_ratio: u8,
    pub encounter: Vec<TomlWildPokemon>,

}

#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct TomlWildPokemon {

    min_level: u8,
    max_level: u8,
    pokemon_id: usize,

}

fn encounter_from_toml(toml_table: TomlWildPokemon) -> WildPokemonEncounter {
    WildPokemonEncounter {
        min_level: toml_table.min_level,
        max_level: toml_table.max_level,
        pokemon_id: toml_table.pokemon_id,
    }
}

fn fill_table(toml_table: &WildPokemonTableInToml) -> [WildPokemonEncounter; 12] {
    let mut arr: [WildPokemonEncounter; 12] = [WildPokemonEncounter::default(); 12];
    for i in 0..12 {
        arr[i] = encounter_from_toml(toml_table.encounter[i]);
    }
    return arr;
}