use serde::{Deserialize, Serialize};

use crate::io::data::StatSet;

use self::moves::MoveInstance;
use self::pokedex::Pokedex;

pub mod pokedex;
pub mod moves;
pub mod instance;
pub mod party;

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
	
	pub data: PokedexData,
	pub base: StatSet,
	pub moves: Vec<LearnableMove>,
	// pub training: Option<Training>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokedexData {
	
	pub number: usize,
	pub name: String,
	pub primary_type: PokemonType,
	pub secondary_type: Option<PokemonType>,
	pub species: String,
	pub height: f32,
	pub weight: f32,
	
}

// impl Default for PokedexData {
// 	fn default() -> Self {
// 		Self {
// 			number: 0,
// 			name: "None".to_string(),
// 			primary_type: PokemonType::Normal,
// 			secondary_type: None,
// 			species: "None".to_string(),
			
// 			height: 0f32,
// 			weight: 0f32,
// 		}
// 	}
// }

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct LearnableMove {
	pub level: u8,
	pub move_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
	
	pub ev_yield: Option<(String, usize)>,
	pub catch_rate: Option<u8>,
	pub base_friendship: Option<u8>,
	pub base_exp: Option<usize>,
	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breeding {
	
//	pub groups: Vec<EggGroup>,
//	pub gender: Option< // something for percentages
	pub cycles: Option<u8>,
	
}

#[derive(PartialEq, Eq, Deserialize, Serialize)]
pub enum Gender {
	
	None,
	Male,
	Female,
	
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum PokemonType {
	
	Normal,
	Fire,
	Water,
	Electric,
	Grass,
	Ice,
	Fighting,
	Poison,
	Ground,
	Flying,
	Psychic,
	Bug,
	Rock,
	Ghost,
	Dragon,
	Dark,
	Steel,
	Fairy,
	
}

impl Default for PokemonType {
    fn default() -> Self {
        Self::Normal
    }
}