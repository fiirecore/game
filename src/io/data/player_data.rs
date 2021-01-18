use crate::game::pokedex::pokedex::Pokedex;
use crate::game::pokedex::pokemon::pokemon_owned::OwnedPokemon;
use crate::util::file::PersistantData;
use crate::util::file::PersistantDataLocation;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use log::info;
use log::warn;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use super::Location;
use super::Position;
use super::pokemon::pokemon_party::PokemonParty;
use super::pokemon::saved_pokemon::SavedPokemon;

static SAVE_FILENAME: &str = "player.json";
#[derive(Serialize, Deserialize)]
pub struct PlayerData {

	// pub world_id: String,
	pub location: Location,
	pub party: PokemonParty,

}

impl PlayerData {

	pub fn exists() -> bool {
		get_path().exists()
	}

	pub fn default_add(&mut self, pokedex: &Pokedex) {
		self.add_pokemon_to_party(OwnedPokemon::get_default0(&pokedex));
		self.add_pokemon_to_party(OwnedPokemon::get_default1(&pokedex));
		self.add_pokemon_to_party(OwnedPokemon::get_default2(&pokedex));
	}

	pub fn add_pokemon_to_party(&mut self, pokemon: OwnedPokemon) {
		self.party.pokemon.push(SavedPokemon::from_owned_pokemon(pokemon));
	}

    pub fn load_from_file() -> PlayerData {
        match read_to_string(get_path().join(SAVE_FILENAME)) {
            Ok(content) => {
//				println!("{}", content);
				match serde_json::from_str(content.as_str()) {
					Ok(data) => {
						return data;
					}
					Err(e) => {
						warn!("Error parsing save: {}", e);
						return new_save();
					}
				}
			}
            Err(err) => {
				warn!("Error opening save file at {:?} with error {}", get_path(), err);
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

				pokemon: Vec::new(),

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
		}
    }
}

impl PersistantDataLocation for PlayerData {

	fn load_from_file() -> Self {
		return PlayerData::load(get_path().with_file_name(SAVE_FILENAME));
	}

}

impl PersistantData for PlayerData {

	fn load<P>(path: P) -> Self where P: AsRef<Path> {
		let path = path.as_ref();
		match read_to_string(path) {
            Ok(content) => {
//				println!("{}", content);
				match serde_json::from_str(content.as_str()) {
					Ok(data) => {
						return data;
					}
					Err(e) => {
						warn!("Error parsing save: {}", e);
						return new_save();
					}
				}
			}
            Err(err) => {
				warn!("Error opening save file at {:?} with error {}", get_path(), err);
                return new_save();
            }
        }
	}

	fn save(&self) {
		let path = get_path();
		if !&path.exists() {
			std::fs::create_dir(&path).expect("Could not create saves directory!");
		}
		let file = File::create(path.join(SAVE_FILENAME)).unwrap();
        let mut writer = BufWriter::new(file);
		info!("Saving player data...");
        match serde_json::to_string_pretty(&self) {
            Ok(encoded) => {
                if let Err(e) = writer.write(encoded.as_bytes()) {
                    warn!("Failed to encode with error: {}", e);
                }
            }
            Err(e) => {
                warn!("Failed to save settings: {}", e);
            }
        }
	}

	fn reload(&mut self) {
		*self = PlayerData::load_from_file();
	}

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