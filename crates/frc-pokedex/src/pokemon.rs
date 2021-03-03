use serde::{Deserialize, Serialize};
use crate::data::LearnableMove;
use crate::data::PokedexData;
use crate::data::StatSet;
use crate::data::training::Training;
use crate::moves::instance::{MoveInstance, MoveInstances};

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
	
	pub data: PokedexData,
	pub base: StatSet,
	pub moves: Vec<LearnableMove>,
	pub training: Training,
	// pub breeding: Option<Breeding>,
	
}

impl Pokemon {

	pub fn moves_from_level(&self, level: u8) -> MoveInstances {
		let mut moves: Vec<MoveInstance> = Vec::new();
		for learnable_move in &self.moves {
			if learnable_move.level <= level {
				if let Some(pokemon_move) =  crate::MOVEDEX.get(&learnable_move.move_id) {
					let mut has = false;
					for pmove in &moves {
						if pmove.pokemon_move.number == pokemon_move.number {
							has = true;
						}
					}
					if !has {
						moves.push(MoveInstance {
							remaining_pp: pokemon_move.pp,
							pokemon_move: pokemon_move,
						});
					}
					
				}
			}
		}
		moves.reverse();
		moves.truncate(4);

		return moves;		
	}
	
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Gender {
	
	None,
	Male,
	Female,
	
}