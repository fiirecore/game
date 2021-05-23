use serde::{Deserialize, Serialize};
use deps::{
	hash::HashMap,
	str::TinyStr16,
	StaticRef,
	Identifiable,
};

use super::pokemon::types::PokemonType;

pub type Movedex = HashMap<MoveId, Move>;

pub static mut MOVEDEX: Option<Movedex> = None;

pub mod instance;

pub mod result;
pub mod target;

pub mod script;
pub mod persistent;

pub type MoveId = TinyStr16;
pub type Power = u8;
pub type Accuracy = u8;
pub type PP = u8;

pub type MoveRef = StaticRef<Move>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Move {

	pub id: MoveId,

	pub name: String,
	pub category: MoveCategory,
	#[serde(rename = "type")]
	pub pokemon_type: PokemonType,

	pub accuracy: Option<Accuracy>,
	pub pp: PP,

	pub use_type: result::MoveUseType,
	#[serde(default = "target::move_target_opponent")]
	pub target: target::MoveTarget,
	
}

impl Identifiable for Move {
    type Id = MoveId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

	fn try_get(id: &Self::Id) -> Option<&'static Self> where Self: Sized {
		unsafe { MOVEDEX.as_ref().map(|map| map.get(id)).flatten() }
	}

}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum MoveCategory {
	Physical,
	Special,
	Status,	
}

impl core::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}