use crate::game::pokedex::{pokedex::Pokedex, pokemon::pokemon_instance::PokemonInstance};
use crate::io::data::pokemon::StatSet;

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

	pub fn get_default0(pokedex: &Pokedex) -> OwnedPokemon {
		return OwnedPokemon::new(PokemonInstance::new(pokedex, pokedex.pokemon_from_id(1), StatSet::uniform(15), 5));
	}

	pub fn get_default1(pokedex: &Pokedex) -> OwnedPokemon {
		return OwnedPokemon::new(PokemonInstance::new(pokedex, pokedex.pokemon_from_id(4), StatSet::uniform(15), 5));
	}

	pub fn get_default2(pokedex: &Pokedex) -> OwnedPokemon {
		return OwnedPokemon::new(PokemonInstance::new(pokedex, pokedex.pokemon_from_id(7), StatSet::uniform(15), 5));
	}
	
}