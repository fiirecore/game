use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PokemonParty {

	pub pokemon: Vec<super::instance::PokemonInstance>,

}

// impl PokemonParty {

// 	pub fn to_instance(&self, pokedex: &super::pokedex::Pokedex) -> Vec<PokemonInstance> {
// 		self.pokemon.iter().map(|pkmn| pkmn.to_pokemon(pokedex)).collect()
// 	}

// }