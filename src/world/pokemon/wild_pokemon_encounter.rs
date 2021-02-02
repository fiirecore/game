use crate::pokemon::instance::PokemonInstance;

#[derive(Copy, Clone, Default)]
pub struct WildPokemonEncounter {

    pub pokemon_id: usize,
    pub min_level: u8,
    pub max_level: u8,

}

impl WildPokemonEncounter {

    pub fn generate_saved(&self) -> PokemonInstance {
        return PokemonInstance::generate(self.pokemon_id, self.min_level, self.max_level, Some(crate::pokemon::data::StatSet::iv_random()));
    }

}