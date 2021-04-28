use firecore_pokedex::item::StackSize;
use firecore_pokedex::{
	item::{ItemId, ItemStack},
	pokemon::party::PokemonParty,
};
use serde::{Deserialize, Serialize};
use firecore_util::{
	GlobalPosition, Location, Position, Coordinate, Direction,
	tinystr::TinyStr16,
	hash::HashMap,
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
	pub party: PokemonParty,

	#[serde(default)]
	pub items: HashMap<ItemId, StackSize>,

	#[serde(default)]
	pub worth: usize,

	#[serde(default)]
	pub world_status: WorldStatus,

}

impl PlayerSave {

	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
			..Default::default()
		}
	}

	pub fn has_battled(&self, map: &String, npc: &u8) -> bool {
		self.world_status.map_data.get(map).map(|map| map.battled.contains(npc)).unwrap_or(false)
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
			party: PokemonParty::default(),
			location: default_location(),
			items: HashMap::new(),
		    worth: 0,
		    world_status: WorldStatus::default(),
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
		position: GlobalPosition {
			local: Position {
				coords: Coordinate {
					x: 6,
					y: 6,
				},
				direction: Direction::Down,
				..Default::default()
			},
			..Default::default()
		}		
	}
}

pub fn default_map() -> TinyStr16 {
	"pallet_houses".parse().expect("Could not get map")
}

pub fn default_index() -> TinyStr16 {
	"player_room".parse().expect("Could not get map index")
}