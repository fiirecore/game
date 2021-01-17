use std::path::Path;

use oorandom::Rand32;

use crate::io::data::pokemon::saved_pokemon::SavedPokemon;

use super::{random_wild_pokemon_table::RandomWildPokemonTable, original_wild_pokemon_table::OriginalWildPokemonTable};

pub static DEFAULT_ENCOUNTER_CHANCE: u8 = 21;
pub trait WildPokemonTable {

    fn encounter_rate(&self) -> u8;

    fn generate(&self, random: &mut Rand32) -> SavedPokemon;

}

pub fn get<P: AsRef<Path>>(encounter_type: &str, path: P) -> Box<dyn WildPokemonTable> {
    let path = path.as_ref();
    match encounter_type {
        "original" => {
            return OriginalWildPokemonTable::from_toml(path);
        }
        _ => {
            return random_wild_table();
        }
    }
}

pub(crate) fn random_wild_table() -> Box<dyn WildPokemonTable> {
    return Box::new(RandomWildPokemonTable {encounter_rate: DEFAULT_ENCOUNTER_CHANCE});
}