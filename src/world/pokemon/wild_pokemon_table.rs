use macroquad::prelude::warn;
use crate::io::data::pokemon::StatSet;
use super::wild_pokemon_encounter::WildPokemonEncounter;
use crate::pokemon::instance::PokemonInstance;

pub static DEFAULT_ENCOUNTER_CHANCE: u8 = 21;
pub struct WildPokemonTable {
    pub encounter_ratio: u8,
    pub table: Option<[WildPokemonEncounter; 12]>,
}

impl WildPokemonTable {

    pub fn encounter_rate(&self) -> u8 {
        self.encounter_ratio
    }

    pub fn generate(&self) -> PokemonInstance {
        match self.table {
            Some(table) => table[get_counter()].generate_saved(),
            None => return PokemonInstance::generate(
                macroquad::rand::gen_range(0, crate::pokemon::pokedex::LENGTH) + 1, 
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
            table: None,
        }
    }
}

pub fn get(encounter_type: &str, file: Option<include_dir::File>) -> WildPokemonTable {

    match encounter_type {
        "original" => {
            return from_toml(file);
        }
        _ => {
            return WildPokemonTable::default();
        }
    }
}

fn from_toml(file: Option<include_dir::File>) -> WildPokemonTable {

    match file {
        Some(file) => {
            match file.contents_utf8() {
                Some(content) => {
                    let toml_result: Result<WildPokemonTableInToml, toml::de::Error> = toml::from_str(content);
                    match toml_result {
                        Ok(toml_table) => {
                            return WildPokemonTable {
                                encounter_ratio: toml_table.encounter_ratio,
                                table: Some(fill_table(&toml_table)),
                            };
                        },
                        Err(err) => {
                            warn!("Could not parse wild pokemon table at {:?} with error {}, using random table instead!", &file.path, err);
                            return WildPokemonTable::default();
                        }
                    }
                }
                None => {
                    warn!("Could not read wild toml file at {} to string!", file.path);
                    return WildPokemonTable::default();
                }
            }
        }
        None => {
            warn!("Could not find wild toml file!");
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