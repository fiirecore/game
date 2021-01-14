use std::path::Path;

use log::warn;
use oorandom::Rand32;

use crate::io::data::pokemon::saved_pokemon::SavedPokemon;

use super::{random_wild_pokemon_table::RandomWildPokemonTable, original_wild_pokemon_table::OriginalWildPokemonTable};

pub static DEFAULT_ENCOUNTER_CHANCE: u8 = 21;
pub trait WildPokemonTable {

    fn encounter_rate(&self) -> u8;

    //fn generate(&mut self, pokedex: &Pokedex, context: &mut GameContext) -> PokemonInstance;

    fn generate(&self, random: &mut Rand32) -> SavedPokemon;

}

pub fn get<P: AsRef<Path>>(encounter_type: String, path: P) -> Option<Box<dyn WildPokemonTable>> {
    let path = path.as_ref();
    let encounter_type = encounter_type.as_str();
    match encounter_type {
        "original" => {
            return Some(get_or_random(path));
        }
        "random" => {
            return Some(get_random());
        }
        _ => {
            return None;
        }
    }
}

fn get_or_random<P: AsRef<Path>>(path: P) -> Box<dyn WildPokemonTable> {
    let path = path.as_ref();
    match OriginalWildPokemonTable::from_toml(path) {
        Some(wpt) => {
            return Box::new(wpt);
        },
        None => {
            warn!("Tried to get original wild pokemon table but failed, using random wild pokemon table");
            return get_random();
        }
    }   
}

fn get_random() -> Box<dyn WildPokemonTable> {
    return Box::new(RandomWildPokemonTable::new(DEFAULT_ENCOUNTER_CHANCE));
}