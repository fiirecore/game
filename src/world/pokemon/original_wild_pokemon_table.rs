use std::path::Path;
use log::warn;
use oorandom::Rand32;
use serde::Deserialize;

use crate::io::data::pokemon::saved_pokemon::SavedPokemon;

use super::wild_pokemon_table::random_wild_table;
use super::{wild_pokemon_encounter::WildPokemonEncounter, wild_pokemon_table::WildPokemonTable};

#[derive(Copy, Clone)]
pub struct OriginalWildPokemonTable {

    pub encounter_ratio: u8,
    pub table: [WildPokemonEncounter; 12],

}

impl OriginalWildPokemonTable {

    pub fn from_toml<P>(path: P) -> Box<dyn WildPokemonTable> where P: AsRef<Path> {
        let path = path.as_ref();

        let content_result = std::fs::read_to_string(path);

        match content_result {
            Ok(content) => {
                let toml_result: Result<WildPokemonTableInToml, toml::de::Error> = toml::from_str(content.as_str());
                match toml_result {
                    Ok(toml_table) => {
                        
                        Box::new(OriginalWildPokemonTable {
                            encounter_ratio: toml_table.encounter_ratio,
                            table: WildPokemonEncounter::fill_table(&toml_table),
                        })
                    },
                    Err(err) => {
                        warn!("Could not parse wild pokemon table at {:?} with error {}, using random table instead!", &path, err);
                        return random_wild_table();
                    }
                }
            },
            Err(err) => {
                warn!("Could not read wild pokemon table at {:?} to string with error: {}, using random table instead!", &path, err);
                return random_wild_table();
            }
        }

    }

    fn get_counter(&self, random: &mut Rand32) -> usize {
        let chance = random.rand_range(1..100) as usize;
        let mut chance_counter = 0;
        let mut counter = 0;
        while chance > chance_counter {
            chance_counter += CHANCES[counter];
            counter+=1;            
        }
        return counter - 1;
    }

}

impl WildPokemonTable for OriginalWildPokemonTable {

    fn encounter_rate(&self) -> u8 {
        return self.encounter_ratio;
    }

    //fn generate(&mut self, pokedex: &Pokedex, context: &mut GameContext) -> PokemonInstance {
    //    return self.table[self.get_counter(context)].generate_instance(pokedex, context);
    //}

    fn generate(&self, random: &mut Rand32) -> SavedPokemon {
        return self.table[self.get_counter(random)].generate_saved(random);
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

