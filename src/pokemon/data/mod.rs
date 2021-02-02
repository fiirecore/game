use serde::{Deserialize, Serialize};

use super::types::PokemonType;

//pub mod pokedex;
pub mod training;



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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LearnableMove {
	pub level: u8,
	pub move_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breeding {
	
//	pub groups: Vec<EggGroup>,
//	pub gender: Option< // something for percentages
	pub cycles: Option<u8>,
	
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Gender {
	
	None,
	Male,
	Female,
	
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StatSet {

	pub hp: u8,
	pub atk: u8,
	pub def: u8,
	pub sp_atk: u8,
	pub sp_def: u8,
	pub speed: u8,

}

impl StatSet {

	pub fn iv_random() -> Self {

		use macroquad::prelude::rand::gen_range;

		Self {
			hp: gen_range(0, 32),
			atk: gen_range(0, 32),
			def: gen_range(0, 32),
			sp_atk: gen_range(0, 32),
			sp_def: gen_range(0, 32),
			speed: gen_range(0, 32),
		}

	}

	pub fn uniform(stat: u8) -> Self {

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