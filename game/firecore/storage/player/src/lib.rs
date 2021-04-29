extern crate firecore_dependencies as deps;

use firecore_dependencies::{
	tinystr::TinyStr16,
	hash::HashMap,
};
use firecore_pokedex::{
	item::{ItemId, ItemStack, StackSize},
	pokemon::saved::SavedPokemonParty,
};
use serde::{Deserialize, Serialize};
use firecore_util::{
	Location, Position, Coordinate, Direction,
};

use world::WorldStatus;


mod list;
pub mod world;

pub use list::PlayerSaves;


#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSave {

	#[serde(default = "default_name")]
	pub name: String,

	#[serde(default = "default_location")]
	pub location: Location,

	#[serde(default)]
	pub party: SavedPokemonParty,

	#[serde(default)]
	pub items: HashMap<ItemId, StackSize>,

	#[serde(default)]
	pub worth: u32,

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

	pub fn add_item(&mut self, stack: ItemStack) -> bool {
		if let Some(item) = firecore_pokedex::itemdex().get(&stack.id) {
			if let Some(count) = self.items.get_mut(&stack.id) {
				if *count + stack.count > item.stack_size {
					false
				} else {
					*count += stack.count;
					true
				}
			} else {
				self.items.insert(stack.id, stack.count);
				true
			}
		} else {
			false
		}
	}

	pub fn use_item(&mut self, id: &ItemId) -> bool {
		if let Some(count) = self.items.get_mut(id) {
			if *count > 0 {
				*count -= 1;
				true
			} else {
				false
			}
		} else {
			false
		}
	}
	
}

impl Default for PlayerSave {
    fn default() -> Self {
		Self {
			name: default_name(),
			party: SavedPokemonParty::default(),
			location: default_location(),
			items: HashMap::new(),
		    worth: 0,
		    world: WorldStatus::default(),
		}
	}

}

pub fn default_name() -> String {
	"Red".to_owned()
}

pub fn default_location() -> Location {
	Location {
		map: Some(default_map()),
		index: default_index(),
		// position: GlobalPosition {
			position: Position {
				coords: Coordinate {
					x: 6,
					y: 6,
				},
				direction: Direction::Down,
				..Default::default()
			},
			// ..Default::default()
		// }		
	}
}

pub fn default_map() -> TinyStr16 {
	"pallet_houses".parse().expect("Could not get map")
}

pub fn default_index() -> TinyStr16 {
	"player_room".parse().expect("Could not get map index")
}