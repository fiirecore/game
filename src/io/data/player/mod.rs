use crate::pokemon::data::StatSet;
use crate::pokemon::party::PokemonParty;
use crate::util::file::PersistantData;
use crate::util::file::PersistantDataLocation;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use macroquad::prelude::info;
use macroquad::prelude::warn;
use serde::{Deserialize, Serialize};
use super::GlobalPosition;
use super::Location;
use super::Position;
use super::world_status::WorldStatus;
use crate::pokemon::instance::PokemonInstance;

static SAVE_DIRECTORY: &str = "saves";
static SAVE_FILENAME: &str = "player.json";
#[derive(Serialize, Deserialize)]
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
		    world_status: WorldStatus {},
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
		match macroquad::file::load_string(path.to_str().expect("Could not get player data path as string")).await {
			Ok(data) => PlayerData::from_string(&data),
		    Err(err) => {
				warn!("Could not open player data file at {:?} with error {}", path, err);
				return new_save();
			}
		}	
	}

	fn save(&self) {
		if crate::SAVEABLE {
			info!("Saving player data...");
			let path = PathBuf::from(SAVE_DIRECTORY);
			if !path.exists() {
				if let Err(err) = std::fs::create_dir_all(&path) {
				    warn!("Could not create saves directory at {:?} with error {}", &path, err);
				}
			}

			if !path.exists() {
				return;
			}
			
			let path = path.join(SAVE_FILENAME);
			match File::create(&path) {
			    Ok(mut file) => {
					match serde_json::to_string_pretty(&self) {
						Ok(encoded) => {
							if let Err(err) = file.write(encoded.as_bytes()) {
								warn!("Failed to encode player data with error: {}", err);
							}
						}
						Err(e) => warn!("Failed to save player data with error: {}", e),
					}
				}
			    Err(err) => warn!("Could not create player save file at {:?} with error {}", &path, err),
			}
			
			
			
		}		
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