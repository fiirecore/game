extern crate firecore_dependencies as deps;

use std::sync::atomic::AtomicBool;
use deps::str::TinyStr16;
use firecore_pokedex::{
	item::bag::Bag,
	pokemon::party::PokemonParty,
};
use firecore_world_lib::character::Character;
use serde::{Deserialize, Serialize};
use firecore_util::{
	Location, Position, Coordinate, Direction,
};

use world::WorldStatus;


mod list;
pub mod world;

pub use list::PlayerSaves;

pub static SHOULD_SAVE: AtomicBool = AtomicBool::new(false); // if true, save player data

pub type Worth = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSave {

	#[serde(default = "default_name")]
	pub name: String,

	#[serde(default = "default_location")]
	pub location: Location,

	#[serde(default = "default_character")]
	pub character: Character,

	#[serde(default)]
	pub party: PokemonParty,

    // #[deprecated(note = "To - do: Item bag module")]
	#[serde(default)]
	pub bag: Bag, // ItemId is redundant

	#[serde(default)]
	pub worth: Worth,

	#[serde(default)]
	pub world: WorldStatus,

}

impl PlayerSave {

	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
			..Default::default()
		}
	}
	
}

impl Default for PlayerSave {
    fn default() -> Self {
		Self {
			name: default_name(),
			party: Default::default(),
			character: default_character(),
			location: default_location(),
			bag: Bag::default(),
		    worth: 0,
		    world: WorldStatus::default(),
		}
	}

}

pub fn default_name() -> String {
	"Red".to_owned()
}

pub const fn default_location() -> Location {
	Location {
		map: Some(default_map()),
		index: default_index(),
	}
}

pub fn default_character() -> Character {
	Character {
		position: default_position(),
		..Default::default()
	}
}

pub const fn default_position() -> Position {
	Position {
		coords: Coordinate {
			x: 6,
			y: 6,
		},
		direction: Direction::Down,
		offset: firecore_util::PixelOffset::ZERO,
	}
}

const DEFAULT_MAP: TinyStr16 = unsafe { TinyStr16::new_unchecked(9142636256173598303365790196080) };
const DEFAULT_INDEX: TinyStr16 = unsafe { TinyStr16::new_unchecked(132299152847616915686911088) };

#[inline]
pub const fn default_map() -> TinyStr16 { // To - do: get this from serialized world binary file
	DEFAULT_MAP
}

#[inline]
pub const fn default_index() -> TinyStr16 {
	DEFAULT_INDEX
}