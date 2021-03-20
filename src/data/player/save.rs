
use firecore_pokedex::pokemon::party::PokemonParty;
use serde::{Deserialize, Serialize};
use firecore_util::{GlobalPosition, Location, Position, Coordinate};
use firecore_pokedex::pokemon::instance::PokemonInstance;

use super::world::WorldStatus;

// #[deprecated(note = "move to firecore-data secondary library crate")]
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSave {

	pub name: String,

	#[serde(default = "player_location")]
	pub location: Location,

	#[serde(default)]
	pub party: PokemonParty,

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

	pub fn add_pokemon_to_party(&mut self, pokemon: PokemonInstance) {
		self.party.pokemon.push(pokemon);
	}

	pub fn has_battled(&self, map: &String, npc: &String) -> bool {
		self.world_status.map_data.get(map).map(|map| map.battled.contains(npc)).unwrap_or(false)
	}
	
}

impl Default for PlayerSave {
    fn default() -> Self {
		Self {
			name: default_name(),
			party: PokemonParty::default(),
			location: player_location(),
		    worth: 0,
		    world_status: WorldStatus::default(),
		}
	}

}

pub fn default_name() -> String {
	"Red".to_owned()
}

fn player_location() -> Location {
	Location {
		map_id: "pallet_houses".to_owned(),
		map_index: 1,
		position: GlobalPosition {
			local: Position {
				coords: Coordinate {
					x: 6,
					y: 6,
				},
				..Default::default()
			},
			..Default::default()
		}		
	}
}