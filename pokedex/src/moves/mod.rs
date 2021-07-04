use serde::{Deserialize, Serialize};
use deps::{
	str::{TinyStr4, TinyStr16},
	hash::HashMap,
	borrow::{Identifiable, StaticRef},
	UNKNOWN16,
};

use crate::Dex;

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
pub type Priority = i8;
pub type CriticalRate = u8;

pub type FieldMoveId = TinyStr4;

pub struct Movedex;

static mut MOVEDEX: Option<HashMap<MoveId, Move>> = None;

impl Dex<'static> for Movedex {
    type DexType = Move;

    fn dex() -> &'static mut Option<HashMap<<<Self as Dex<'static>>::DexType as Identifiable<'static>>::Id, Self::DexType>> {
        unsafe { &mut MOVEDEX }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Move {

	pub id: MoveId,

	pub name: String,
	pub category: MoveCategory,
	#[serde(rename = "type")]
	pub pokemon_type: crate::types::PokemonType,

	pub accuracy: Option<Accuracy>,
	pub pp: PP,
	#[serde(default)]
	pub priority: Priority,

	pub usage: Vec<usage::MoveUseType>,

	#[serde(default)]
	pub target: target::MoveTarget,

	#[serde(default)]
	pub contact: bool,

	#[serde(default)]
	pub crit_rate: CriticalRate,

	pub field_id: Option<FieldMoveId>,
	
}

pub type MoveRef = StaticRef<Move>;

impl<'a> Identifiable<'a> for Move {
    type Id = MoveId;

	const UNKNOWN: MoveId = UNKNOWN16;

    fn id(&self) -> &Self::Id {
        &self.id
    }

	fn try_get(id: &Self::Id) -> Option<&'a Self> where Self: Sized {
		Movedex::try_get(id)
	}

}

impl core::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}