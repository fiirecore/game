use serde::{Deserialize, Serialize};

use crate::pokemon::pokedex::Pokedex;

use super::MoveInstance;
#[derive(Debug, Clone, Serialize, Deserialize)]
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
		let mut instance = Vec::new();
		for pkmn_move in &self.moves {
			if let Some(pokemon_move) = pokedex.move_list.get(&pkmn_move.move_id) {
				instance.push(MoveInstance {
					move_instance: pokemon_move.clone(),
					remaining_pp: pkmn_move.remaining_pp,
				});
			} else {
				macroquad::prelude::warn!("Could not find move #{} from saved moveset!", &pkmn_move.move_id);
			}
		}
		return instance;
	}

	pub fn from_instance(moves: &Vec<MoveInstance>) -> Self {
		Self {
			moves: moves.iter().map(|pkmn_move| SavedPokemonMove {
			    move_id: pkmn_move.move_instance.number,
			    remaining_pp: pkmn_move.remaining_pp,
			}).collect(),
		}
	}

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPokemonMove {

	pub move_id: u16,
	pub remaining_pp: u8,
	
}