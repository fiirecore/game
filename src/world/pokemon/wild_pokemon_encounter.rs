use crate::io::data::pokemon::StatSet;
use crate::io::data::pokemon::saved_pokemon::SavedPokemon;

#[derive(Copy, Clone, Default)]
pub struct WildPokemonEncounter {

    pub pokemon_id: usize,
    pub min_level: u8,
    pub max_level: u8,

}

impl WildPokemonEncounter {

    pub fn generate_saved(&self) -> SavedPokemon {
        return SavedPokemon::generate(self.pokemon_id, self.min_level, self.max_level, Some(StatSet::iv_random()), None);
    }

}