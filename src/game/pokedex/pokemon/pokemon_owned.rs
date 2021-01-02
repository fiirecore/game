use crate::game::pokedex::{pokedex::Pokedex, pokemon::pokemon_instance::PokemonInstance};

use super::stat_set::StatSet;

#[allow(dead_code)]
pub struct OwnedPokemon {
	
	pub instance: PokemonInstance,
	pub exp: usize,
	pub friendship: u8,
	
}

impl OwnedPokemon {

	pub fn new(instance: PokemonInstance) -> OwnedPokemon {
		
		OwnedPokemon {
			
			instance: instance,
			exp: 0,
			friendship: 70, // instance.pokemon.default friendship
			
		}
		
	}

	pub fn get_default(pokedex: &Pokedex) -> OwnedPokemon {
		return OwnedPokemon::new(PokemonInstance::new(pokedex, pokedex.pokemon_from_id(7), StatSet::uniform(15), 5));
	}
	
}