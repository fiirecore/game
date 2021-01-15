use serde::{Deserialize, Serialize};

use crate::game::pokedex::pokedex::Pokedex;
use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;

use super::saved_pokemon::SavedPokemon;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokemonParty {

	pub pokemon: Vec<SavedPokemon>,

}

impl PokemonParty {

	pub fn to_instance(&self, pokedex: &Pokedex) -> Vec<PokemonInstance> {
		self.pokemon.iter().map(|pkmn| pkmn.to_pokemon(pokedex)).collect()
	}

}