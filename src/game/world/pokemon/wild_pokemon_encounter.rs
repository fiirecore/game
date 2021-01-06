use oorandom::Rand32;

use crate::game::pokedex::pokemon::stat_set::StatSet;
use crate::io::data::saved_pokemon::SavedPokemon;

#[derive(Copy, Clone)]
pub struct WildPokemonEncounter {

    pub pokemon_id: usize,
    pub min_level: u8,
    pub max_level: u8,

}

impl WildPokemonEncounter {

    //pub fn generate_instance(&self, pokedex: &Pokedex, context: &mut GameContext) -> PokemonInstance {
    //    return PokemonInstance::generate(pokedex, context, pokedex.pokemon_from_id(self.pokemon_id), self.min_level, self.max_level);
    //}

    pub fn generate_saved(&self, random: &mut Rand32) -> SavedPokemon {
        let ivs = StatSet::iv_random(random);
        return SavedPokemon::generate(random, self.pokemon_id, self.min_level, self.max_level, Some(ivs), None);
    }

}