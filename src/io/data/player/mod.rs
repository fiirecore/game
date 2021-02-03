use crate::pokemon::data::StatSet;
use crate::pokemon::party::PokemonParty;
use crate::util::file::PersistantData;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use serde::{Deserialize, Serialize};
use super::Location;
use super::Position;
use crate::pokemon::instance::PokemonInstance;

static SAVE_FILENAME: &str = "player.json";
#[derive(Serialize, Deserialize)]
pub struct PlayerData {

	// pub world_id: String,
	pub location: Location,
	pub party: PokemonParty,

	#[serde(skip)]
	pub dirty: bool,

}

impl PlayerData {

	pub fn exists() -> bool {
		get_path().exists()
	}

	pub fn add_pokemon_to_party(&mut self, pokemon: PokemonInstance) {
		self.party.pokemon.push(pokemon);
	}

	pub fn mark_dirty(&mut self) {
		self.dirty = true;
	}

	pub async fn load_async_default() -> Self {
		return PlayerData::load_async(get_path().join(SAVE_FILENAME)).await;
	}

	pub async fn load_async(path: PathBuf) -> Self {
		match macroquad::file::load_string(path.to_str().expect("Could not get player data path as string")).await {
			Ok(data) => PlayerData::from_string(&data),
		    Err(err) => {
				warn!("Could not open player data file at {:?} with error {}", path, err);
				return new_save();
			}
		}
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
			
			// world_id: String::from("firered"),

			party: PokemonParty {

				pokemon: vec![
					PokemonInstance::generate_with_level(1, 11, Some(StatSet::uniform(15))),
					PokemonInstance::generate_with_level(4, 11, Some(StatSet::uniform(15))),
					PokemonInstance::generate_with_level(7, 11, Some(StatSet::uniform(15))),
				],

			},

			location: Location {

				map_id: String::from("pallet_town_player_house"),
				map_index: 1,

				position: Position {
					x: 6,
					y: 6,
					..Default::default()
				}
				
			},

			dirty: false,

		}
	}

}

// impl PersistantDataLocation for PlayerData {

// 	fn load_from_file() -> Self {
// 		return PlayerData::load(get_path().join(SAVE_FILENAME));
// 	}

// }

impl PersistantData for PlayerData {

// 	fn load(path: PathBuf) -> Self {
// 		//let path= path.as_ref();
// 		match read_to_string_noasync(path) {
//             Some(content) => {
				
// 			}
//             None => {
// 				warn!("Error opening save file at {:?} with error", get_path());
//                 return new_save();
//             }
//         }
// 	}

	fn save(&self) {
		if cfg!(not(target_arch = "wasm32")) {
			info!("Saving player data...");
			let path = get_path();
			if !&path.exists() {
				match std::fs::create_dir_all(&path) {
				    Ok(()) => (),
				    Err(err) => {
						warn!("Could not create saves directory at {:?} with error {}", &path, err);
					}
				}
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
						Err(e) => {
							warn!("Failed to save settings: {}", e);
						}
					}
				}
			    Err(err) => {
					warn!("Could not create player save file at {:?} with error {}", &path, err);
				},
			}
			
			
			
		}		
	}

	// async fn reload(&mut self) {
	// 	*self = PlayerData::load_from_file().await;
	// }

}

fn get_path() -> PathBuf {
	//match current_exe() {
	//	Ok(mut exe_path) => {
	//		exe_path.pop();
	//		exe_path.push("saves");
	//		return exe_path;
	//	}
	//	Err(e) => {
	//		warn!("Failed to find exe path with error {}", e);
	//		let mut pb = PathBuf::from("./");
	//		pb.push("saves");
	//		return pb;
	//	}
	//}
    return PathBuf::from("saves");
}

fn new_save() -> PlayerData {
	info!("Creating a new player save file.");
	let default = PlayerData::default();
	default.save();
	return default;
}