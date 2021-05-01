use serde::{Deserialize, Serialize};

use crate::pokemon::{PokemonId, Level, Stat, types::PokemonType};

use crate::moves::MoveId;

use super::POKEMON_RANDOM;


pub mod training;
pub mod breeding;


#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Gender {
	None,
	Male,
	Female,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokedexData {
	pub id: PokemonId,
	pub name: String,
	pub primary_type: PokemonType,
	pub secondary_type: Option<PokemonType>,
	pub species: String,
	pub height: u8,
	pub weight: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LearnableMove {
	pub move_id: MoveId,
	pub level: Level,
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

	pub const MAX_EV: Stat = 32;
	pub const MAX_IV: Stat = 252;
	pub const MAX_IVS_TOTAL: u16 = 512;

	pub const fn uniform(stat: Stat) -> Self {
		Self {
			hp: stat,
			atk: stat,
			def: stat,
			sp_atk: stat,
			sp_def: stat,
			speed: stat,
		}
	}

	pub fn random() -> Self {
		Self {
			hp: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			atk: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			def: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			sp_atk: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			sp_def: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			speed: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
		}
	}

}