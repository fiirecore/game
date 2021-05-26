use serde::{Deserialize, Serialize};
use deps::{
	str::{TinyStr4, TinyStr16},
	StaticRef,
	Identifiable,
};

use crate::types::PokemonType;

pub mod dex;

mod category;
pub use category::*;

pub mod instance;

pub mod usage;
pub mod target;

pub mod persistent;

pub type MoveId = TinyStr16;
pub type Power = u8;
pub type Accuracy = u8;
pub type PP = u8;

pub type FieldMoveId = TinyStr4;

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

	pub usage: usage::MoveUseType,

	#[serde(default = "target::move_target_opponent")]
	pub target: target::MoveTarget,

	pub field_id: Option<FieldMoveId>,
	
}

pub type MoveRef = StaticRef<Move>;

impl Identifiable for Move {
    type Id = MoveId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

	fn try_get(id: &Self::Id) -> Option<&'static Self> where Self: Sized {
		unsafe { dex::MOVEDEX.as_ref().map(|map| map.get(id)).flatten() }
	}

}

impl core::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}