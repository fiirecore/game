use firecore_pokedex::pokemon::data::StatSet;
use firecore_pokedex::pokemon::party::PokemonParty;
use crate::util::Coordinate;
use frc_data::data::PersistantData;
use std::path::{Path, PathBuf};
use macroquad::prelude::info;
use macroquad::prelude::warn;
use serde::{Deserialize, Serialize};
use crate::util::GlobalPosition;
use crate::util::Location;
use crate::util::Position;
use super::world::WorldStatus;
use firecore_pokedex::pokemon::instance::PokemonInstance;

static SAVE_DIRECTORY: &str = "saves";
static SAVE_FILE_TYPE: &str = ".ron";

pub static mut PLAYER_SAVE: Option<String> = None;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {

	pub name: String,

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

	pub fn select_data(data: String) {
		unsafe { PLAYER_SAVE = Some(data) };
	}

	pub async fn load_selected_data() -> Self {
		match unsafe {PLAYER_SAVE.as_ref()} {
		    Some(save_name) => Self::load(Path::new(SAVE_DIRECTORY).join(save_name.clone() + SAVE_FILE_TYPE)).await,
		    None => {
				warn!("Could not get player save data because no data has been selected");
				return new_save();
			}
		}
		
	}

	pub fn add_pokemon_to_party(&mut self, pokemon: PokemonInstance) {
		self.party.pokemon.push(pokemon);
	}

	pub fn mark_dirty(&mut self) {
		self.dirty = true;
	}
	
	fn from_string(data: &str) -> Self {
		match ron::from_str(data) {
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
			name: "Red".to_string(),
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

fn player_party() -> PokemonParty {
	PokemonParty {
		pokemon: firecore_pokedex::smallvec![
			PokemonInstance::generate_with_level(1, 11, Some(StatSet::uniform(15))),
			PokemonInstance::generate_with_level(4, 11, Some(StatSet::uniform(15))),
			PokemonInstance::generate_with_level(7, 11, Some(StatSet::uniform(15))),
		],
	}
}

#[async_trait::async_trait(?Send)]
impl PersistantData for PlayerData {
	
	async fn load(path: PathBuf) -> Self {
		info!("Loading player data...");
		match frc_data::data::read_string(&path).await {
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
			
			frc_data::data::save_struct(PathBuf::from(SAVE_DIRECTORY).join(unsafe{PLAYER_SAVE.clone()}.unwrap_or(String::from("player")) + SAVE_FILE_TYPE), &self);

			crate::gui::set_message(super::text::MessageSet::new(
				1, 
				super::text::color::TextColor::Black, 
				vec![vec![String::from("Saved player data!")]]
			));
			info!("Saved player data!");
	}

	async fn reload(&mut self) {
		*self = PlayerData::load_selected_data().await;
	}

}

fn new_save() -> PlayerData {
	info!("Creating a new player save file.");
	let default = PlayerData::default();
	default.save();
	return default;
}