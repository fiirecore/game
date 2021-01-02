use crate::{engine::game_context::GameContext, game::pokedex::{pokedex::Pokedex, pokemon::pokemon_instance::PokemonInstance}};

#[derive(Copy, Clone)]
pub struct WildPokemonEncounter {

    pub min_level: u8,
    pub max_level: u8,
    pub pokemon_id: usize,

}

impl WildPokemonEncounter {

    pub fn generate_instance(&mut self, pokedex: &Pokedex, context: &mut GameContext) -> PokemonInstance {
        if self.min_level == self.max_level {
            self.max_level+=1;
        }
        return PokemonInstance::generate(pokedex, context, pokedex.pokemon_from_id(self.pokemon_id), self.min_level, self.max_level);
    }

}