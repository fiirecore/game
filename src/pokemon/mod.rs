use serde::{Deserialize, Serialize};

use self::data::LearnableMove;
use self::data::PokedexData;
use self::data::StatSet;
use self::data::training::Training;
use self::moves::MoveInstance;
use self::pokedex::Pokedex;

pub mod pokedex;
pub mod data;
pub mod types;
pub mod moves;
pub mod instance;
pub mod party;

pub type PokemonId = u16;

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
	
	pub data: PokedexData,
	pub base: StatSet,
	pub moves: Vec<LearnableMove>,
	pub training: Training,
	// pub breeding: Option<Breeding>,
	
}

impl Pokemon {

	pub fn moves_from_level(&self, pokedex: &Pokedex, level: u8) -> Vec<MoveInstance> {
		let mut moves = Vec::new();
		for learnable_move in &self.moves {
			if learnable_move.level <= level {
				match pokedex.move_list.get(&learnable_move.move_id) {
					Some(pokemon_move) => {
						moves.push(MoveInstance {
                            move_instance: pokemon_move.clone(),
						    remaining_pp: pokemon_move.pp,
                        });
					}
					None => {
						macroquad::prelude::warn!("Could not add pokemon move {} to {}", &learnable_move.move_id, &self.data.name)
					}
				}
			}
		}
		while moves.len() > 4 {
			moves.remove(0);
		}
		return moves;
	}

	pub fn from_string(data: &str) -> Result<Pokemon, toml::de::Error> {
		return toml::from_str(data);
	}
	
}