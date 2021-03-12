use firecore_pokedex::pokemon::data::StatSet;
use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_util::text::MessageSet;
use firecore_util::text::TextColor;
use macroquad::prelude::collections::storage;
use firecore_data::data::PersistantData;
use std::path::{Path, PathBuf};
use macroquad::prelude::info;
use macroquad::prelude::warn;
use serde::{Deserialize, Serialize};
use firecore_util::{GlobalPosition, Location, Position, Coordinate};
use super::world::WorldStatus;
use firecore_pokedex::pokemon::instance::PokemonInstance;

static SAVE_DIRECTORY: &str = "saves";
static SAVE_FILE_TYPE: &str = ".ron";

pub static mut PLAYER_SAVE: Option<String> = None;
pub static mut DIRTY: bool = false;

lazy_static::lazy_static! {
	pub static ref PLAYER_DATA: parking_lot::RwLock<Option<PlayerData>> = parking_lot::RwLock::new(None);
}

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

	// #[serde(skip)]
	// pub dirty: bool,

}

impl PlayerData {

	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
			..Default::default()
		}
	}

	pub fn select_data(name: &str) {
		unsafe {
			PLAYER_SAVE = Some(name.to_owned());
		}
	}

	pub async fn load_selected_data() {
		if let Some(save) = unsafe{PLAYER_SAVE.take()} {
			let data = Self::load(Path::new(SAVE_DIRECTORY).join( save + SAVE_FILE_TYPE)).await;
			storage::store(data);
		}
	}

	// fn from_string(data: &str) -> Self {
	// 	match ron::from_str(data) {
	// 		Ok(data) => return data,
	// 		Err(err) => {
	// 			warn!("Error parsing player save: {}", err);
	// 			let name = path.file_name().unwrap().to_string_lossy();
	// 			let name = &name[0..name.len() - 4];
	// 			return new_save(name);
	// 		}
	// 	}
	// }


	pub fn add_pokemon_to_party(&mut self, pokemon: PokemonInstance) {
		self.party.pokemon.push(pokemon);
	}

	// pub fn mark_dirty(&mut self) {
	// 	self.dirty = true;
	// }
	
}

impl Default for PlayerData {
    fn default() -> Self {
		Self {
			name: "Red".to_string(),
			party: player_party(),
			location: player_location(),
		    worth: 0,
		    world_status: WorldStatus::default(),
		    // dirty: false,
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
		match firecore_data::data::read_string(&path).await {
			Ok(data) => {
				match ron::from_str(&data) {
				    Ok(data) => data,
				    Err(err) => {
						warn!("Could not read player data with error {}", err);
						let name = path.file_name().unwrap().to_string_lossy();
						let name = &name[0..name.len() - 4];
						new_save(name)
					}
				}
			},
		    Err(err) => {
				warn!("Could not open player data file at {:?} with error {}", path, err);
				let name = path.file_name().unwrap().to_string_lossy();
				let name = &name[0..name.len() - 4];
				new_save(name)
			}
		}
	}

	fn save(&self) {
		info!("Saving player data...");

		crate::scene::scenes::main_menu::save_list::SaveList::append(&self.name);
		
		firecore_data::data::save_struct(PathBuf::from(SAVE_DIRECTORY).join(self.name.clone() + SAVE_FILE_TYPE), &self);

		crate::gui::set_message(MessageSet::new(
			1, 
			TextColor::Black, 
			vec![vec![String::from("Saved player data!")]]
		));
		info!("Saved player data!");
	}

	async fn reload(&mut self) {
		*self = Self::load(Path::new(SAVE_DIRECTORY).join(self.name.clone() + SAVE_FILE_TYPE)).await;
	}

}

fn new_save(name: &str) -> PlayerData {
	info!("Creating a new player save file.");
	let default = PlayerData::new(name);
	default.save();
	return default;
}