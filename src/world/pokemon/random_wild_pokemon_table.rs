use oorandom::Rand32;

use crate::game::pokedex::pokedex::LENGTH;
use crate::game::pokedex::pokemon::stat_set::StatSet;
use crate::io::data::pokemon::saved_pokemon::SavedPokemon;

use super::wild_pokemon_table::WildPokemonTable;

pub struct RandomWildPokemonTable {

    encounter_rate: u8,

}

impl RandomWildPokemonTable {

    pub fn new(encounter_rate: u8) -> Self {

        Self {

            encounter_rate: encounter_rate,

        }

    }

}

impl WildPokemonTable for RandomWildPokemonTable {

    fn encounter_rate(&self) -> u8 {
        return self.encounter_rate;
    }

    // fn generate(&mut self, pokedex: &Pokedex, context: &mut GameContext) -> PokemonInstance {
    //     let id = context.random.rand_range(0..pokedex.pokemon_list.len() as u32) as usize;
    //     return PokemonInstance::generate(pokedex, context, pokedex.pokemon_from_id(id), 1, 100);
    // }

    fn generate(&self, random: &mut Rand32) -> SavedPokemon {
        let id = random.rand_range(0..LENGTH as u32) as usize + 1;
        let ivs = StatSet::iv_random(random);        
        return SavedPokemon::generate(random, id, 1, 100, Some(ivs), None);
    }
}