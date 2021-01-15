use oorandom::Rand32;

use crate::io::data::pokemon::StatSet;
use crate::io::data::pokemon::saved_pokemon::SavedPokemon;

#[derive(Copy, Clone)]
pub struct WildPokemonEncounter {

    pub pokemon_id: usize,
    pub min_level: u8,
    pub max_level: u8,

}

impl WildPokemonEncounter {

    pub fn generate_saved(&self, random: &mut Rand32) -> SavedPokemon {
        let ivs = StatSet::iv_random(random);
        return SavedPokemon::generate(random, self.pokemon_id, self.min_level, self.max_level, Some(ivs), None);
    }

}