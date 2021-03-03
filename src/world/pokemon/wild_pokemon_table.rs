use std::path::PathBuf;

use macroquad::prelude::warn;
use frc_pokedex::PokemonId;
use frc_pokedex::data::StatSet;
use super::wild_pokemon_encounter::WildPokemonEncounter;
use frc_pokedex::instance::PokemonInstance;

pub static DEFAULT_ENCOUNTER_CHANCE: u8 = 21;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WildPokemonTable {
    pub encounter_ratio: u8,
    pub encounter: Option<[WildPokemonEncounter; 12]>,
}

impl WildPokemonTable {

    pub fn encounter_rate(&self) -> u8 {
        self.encounter_ratio
    }

    pub fn generate(&self) -> PokemonInstance {
        match self.encounter {
            Some(encounter) => encounter[get_counter()].generate_saved(),
            None => return PokemonInstance::generate(
                macroquad::rand::gen_range(0, frc_pokedex::POKEDEX.len()) as PokemonId + 1, 
                1, 
                100, 
                Some(StatSet::iv_random()), 
            ),
        }
    }

}

impl Default for WildPokemonTable {
    fn default() -> Self {
        Self {
            encounter_ratio: DEFAULT_ENCOUNTER_CHANCE,
            encounter: None,
        }
    }
}

pub fn get_table(encounter_type: &str, file: PathBuf) -> WildPokemonTable {

    match encounter_type {
        "original" => {
            return from_toml(file);
        }
        _ => {
            return WildPokemonTable::default();
        }
    }

}

fn from_toml(file: PathBuf) -> WildPokemonTable {

    match crate::io::get_file_as_string(&file) {
        Ok(content) => {
            match toml::from_str(&content) {
                Ok(table) => return table,
                Err(err) => {
                    warn!("Could not parse wild pokemon table at {:?} with error {}, using random table instead!", &file, err);
                    return WildPokemonTable::default();
                }
            }
        }
        Err(err) => {
            warn!("Could not find wild toml file at {:?} with error {}!", file, err);
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

// #[derive(Debug, serde::Deserialize)]
// pub struct SerializedWildPokemonTable {

//     pub encounter_ratio: u8,
//     pub encounter: Vec<TomlWildPokemon>,

// }

// #[derive(Debug, serde::Deserialize, Clone, Copy)]
// pub struct TomlWildPokemon {

//     min_level: u8,
//     max_level: u8,
//     pokemon_id: PokemonId,

// }

// fn encounter_from_toml(toml_table: TomlWildPokemon) -> WildPokemonEncounter {
//     WildPokemonEncounter {
//         min_level: toml_table.min_level,
//         max_level: toml_table.max_level,
//         pokemon_id: toml_table.pokemon_id,
//     }
// }

// fn fill_table(toml_table: &WildPokemonTableInToml) -> [WildPokemonEncounter; 12] {
//     let mut arr: [WildPokemonEncounter; 12] = [WildPokemonEncounter::default(); 12];
//     for i in 0..12 {
//         arr[i] = encounter_from_toml(toml_table.encounter[i]);
//     }
//     return arr;
// }