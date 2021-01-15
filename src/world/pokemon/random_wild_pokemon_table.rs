use oorandom::Rand32;

use crate::game::pokedex::pokedex::LENGTH;
use crate::io::data::pokemon::StatSet;
use crate::io::data::pokemon::saved_pokemon::SavedPokemon;

use super::wild_pokemon_table::WildPokemonTable;

pub struct RandomWildPokemonTable {
    pub encounter_rate: u8,
}

impl WildPokemonTable for RandomWildPokemonTable {

    fn encounter_rate(&self) -> u8 {
        return self.encounter_rate;
    }

    fn generate(&self, random: &mut Rand32) -> SavedPokemon {
        let id = random.rand_range(0..LENGTH as u32) as usize + 1;
        let ivs = StatSet::iv_random(random);        
        return SavedPokemon::generate(random, id, 1, 100, Some(ivs), None);
    }
}