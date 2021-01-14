
use serde::{Deserialize, Serialize};

use crate::game::pokedex::pokedex::Pokedex;
use crate::game::pokedex::pokemon_move::move_instance::MoveInstance;

use super::saved_pokemon_move::SavedPokemonMove;
#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct SavedPokemonMoveSet {

	pub moves: Vec<SavedPokemonMove>,

}

impl SavedPokemonMoveSet {

	pub fn empty() -> Self {
		Self {
			moves: Vec::new(),
		}
	}

	pub fn to_instance(&self, pokedex: &Pokedex) -> Vec<MoveInstance> {
		return self.moves.iter().map(|pkmn_move| MoveInstance {
			move_instance: pokedex.move_list.get(&pkmn_move.name).unwrap().clone(),
			remaining_pp: pkmn_move.remaining_pp,
		}).collect();
	}

	pub fn from_instance(moves: Vec<MoveInstance>) -> Self {
		Self {
			moves: moves.iter().map(|pkmn_move| SavedPokemonMove {
			    name: pkmn_move.move_instance.name.clone(),
			    remaining_pp: pkmn_move.remaining_pp,
			}).collect(),
		}
	}

}