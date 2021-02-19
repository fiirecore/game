use crate::pokemon::data::StatSet;
use crate::pokemon::party::PokemonParty;
use crate::util::file::PersistantData;
use crate::util::file::PersistantDataLocation;
use std::path::{Path, PathBuf};
use macroquad::prelude::info;
use macroquad::prelude::warn;
use serde::{Deserialize, Serialize};
use super::GlobalPosition;
use super::Location;
use super::Position;
use super::world::WorldStatus;
use crate::pokemon::instance::PokemonInstance;

static SAVE_DIRECTORY: &str = "saves";
static SAVE_FILENAME: &str = "player.json";
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {

	// pub world_id: String,
	#[serde(default = "player_location")]
	pub location: Location,
	#[serde(default = "player_party")]
	pub party: PokemonParty,

	#[serde(default)]
	pub worth: usize,

	#[serde(default)]
	pub world_status: WorldStatus,

	#[serde(skip)]
	pub dirty: bool,

}

impl PlayerData {

	pub fn add_pokemon_to_party(&mut self, pokemon: PokemonInstance) {
		self.party.pokemon.push(pokemon);
	}

	pub fn mark_dirty(&mut self) {
		self.dirty = true;
	}
	
	fn from_string(data: &str) -> Self {
		match serde_json::from_str(data) {
			Ok(data) => return data,
			Err(err) => {
				warn!("Error parsing player save: {}", err);
				return new_save();
			}
		}
	}
	
}

impl Default for PlayerData {
    fn default() -> Self {
		Self {
			party: player_party(),
			location: player_location(),
		    worth: 0,
		    world_status: WorldStatus::default(),
		    dirty: false,
		}
	}

}

fn player_location() -> Location {
	Location {
		map_id: String::from("pallet_town_player_house"),
		map_index: 1,
		position: GlobalPosition {
			local: Position {
				x: 6,
				y: 6,
				..Default::default()
			},
			..Default::default()
		}		
	}
}

fn player_party() -> PokemonParty {
	PokemonParty {
		pokemon: vec![
			PokemonInstance::generate_with_level(1, 11, Some(StatSet::uniform(15))),
			PokemonInstance::generate_with_level(4, 11, Some(StatSet::uniform(15))),
			PokemonInstance::generate_with_level(7, 11, Some(StatSet::uniform(15))),
		],
	}
}

#[async_trait::async_trait(?Send)]
impl PersistantDataLocation for PlayerData {

	async fn load_from_file() -> Self {
		return PlayerData::load(Path::new(SAVE_DIRECTORY).join(SAVE_FILENAME)).await;
	}

}

#[async_trait::async_trait(?Send)]
impl PersistantData for PlayerData {
	
	async fn load(path: PathBuf) -> Self {
		info!("Loading player data...");
		match crate::util::file::read_string(&path).await {
			Ok(data) => PlayerData::from_string(&data),
		    Err(err) => {
				warn!("Could not open player data file at {:?} with error {}", path, err);
				return new_save();
			}
		}	
	}

	fn save(&self) {
		// #[cfg(not(target_arch = "wasm32"))] {
			info!("Saving player data...");
			
			crate::util::file::save_struct(PathBuf::from(SAVE_DIRECTORY).join(SAVE_FILENAME), &self);

			crate::gui::set_message(super::text::MessageSet::new(
				1, 
				super::text::color::TextColor::Black, 
				vec![vec![String::from("Saved player data!")]]
			));
			info!("Saved player data!");
		// }
		// #[cfg(target_arch = "wasm32")]
		// {
		// 	crate::gui::set_message(super::text::MessageSet::new(
		// 		1, 
		// 		super::text::color::TextColor::Black, 
		// 		vec![vec![String::from("Cannot save player data"), String::from("on web browsers!")]]
		// 	));
		// }
	}

	// async fn reload(&mut self) {
	// 	*self = PlayerData::load_from_file().await;
	// }

}

fn new_save() -> PlayerData {
	info!("Creating a new player save file.");
	let default = PlayerData::default();
	default.save();
	return default;
}