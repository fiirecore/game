use serde::{Deserialize, Serialize};
use quad_rand::gen_range;

use crate::Level;
use crate::MoveId;
use crate::Stat;

use super::PokemonId;
use super::types::PokemonType;

//pub mod pokedex;
pub mod training;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokedexData {
	
	pub number: PokemonId,
	pub name: String,
	pub primary_type: PokemonType,
	pub secondary_type: Option<PokemonType>,
	pub species: String,
	pub height: f32,
	pub weight: f32,
	
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LearnableMove {
	pub move_id: MoveId,
	pub level: Level,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breeding {
	
//	pub groups: Vec<EggGroup>,
//	pub gender: Option< // something for percentages
	pub cycles: Option<u8>,
	
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StatSet {

	pub hp: Stat,
	pub atk: Stat,
	pub def: Stat,
	pub sp_atk: Stat,
	pub sp_def: Stat,
	pub speed: Stat,

}

impl StatSet {

	pub fn iv_random() -> Self {
		Self {
			hp: gen_range(0, 32),
			atk: gen_range(0, 32),
			def: gen_range(0, 32),
			sp_atk: gen_range(0, 32),
			sp_def: gen_range(0, 32),
			speed: gen_range(0, 32),
		}
	}

	pub fn uniform(stat: Stat) -> Self {
		Self {
			hp: stat,
			atk: stat,
			def: stat,
			sp_atk: stat,
			sp_def: stat,
			speed: stat,
		}
	}

}