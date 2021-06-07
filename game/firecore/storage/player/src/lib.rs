extern crate firecore_dependencies as deps;
extern crate firecore_util as util;
extern crate firecore_pokedex as pokedex;
extern crate firecore_world_lib as worldlib;

use std::sync::atomic::AtomicBool;
use serde::{Deserialize, Serialize};
use util::{Location, LocationId, Position, Coordinate, Direction, PixelOffset};
use pokedex::{
	item::bag::Bag,
	pokemon::party::PokemonParty,
};
use worldlib::character::Character;

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

	#[serde(default)]
	pub bag: Bag, // To - do: ItemId is redundant

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
		offset: PixelOffset::ZERO,
	}
}

const DEFAULT_MAP: LocationId = unsafe { LocationId::new_unchecked(9142636256173598303365790196080u128) };
const DEFAULT_INDEX: LocationId = unsafe { LocationId::new_unchecked(132299152847616915686911088u128) };

#[inline]
pub const fn default_map() -> LocationId { // To - do: get this from serialized world binary file
	DEFAULT_MAP
}

#[inline]
pub const fn default_index() -> LocationId {
	DEFAULT_INDEX
}