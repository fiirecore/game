use crate::{engine::game_context::GameContext, game::pokedex::{pokedex::Pokedex, pokemon::pokemon_instance::PokemonInstance}};

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

    fn generate(&mut self, pokedex: &Pokedex, context: &mut GameContext) -> PokemonInstance {
        let id = context.random.rand_range(0..pokedex.pokemon_list.len() as u32) as usize;
        return PokemonInstance::generate(pokedex, context, pokedex.pokemon_from_id(id), 1, 100);
    }

}