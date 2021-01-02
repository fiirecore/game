use crate::game::pokedex::{pokedex::Pokedex, pokemon::pokemon::PokemonType};
use serde_derive::{Deserialize, Serialize};

use super::{move_category::MoveCategory, move_instance::MoveInstance};

#[derive(Clone)]
pub struct PokemonMove {

	pub name: String,
	pub category: MoveCategory,
	pub pokemon_type: Option<PokemonType>,
	pub power: Option<usize>,
	pub accuracy: u8,
	pub pp: u8,
	
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedPokemonMove {

	pub name: String,
	pub pp: u8,
	
}

impl SavedPokemonMove {

	pub fn to_instance(moves: &[Option<SavedPokemonMove>; 4], pokedex: &Pokedex) -> Vec<MoveInstance> {
		let mut move_instances = Vec::new();
		for pkmn_move in moves {
			if let Some(pkmn_move) = pkmn_move {
				move_instances.push(MoveInstance {
					move_instance: pokedex.move_list.get(&pkmn_move.name).unwrap().clone(),
					remaining_pp: pkmn_move.pp,
				});
			}
		}
		return move_instances;
	}

}