use crate::{game::pokedex::{pokedex::Pokedex, pokemon::{pokemon_instance::PokemonInstance, pokemon_owned::OwnedPokemon, stat_set::StatSet}, pokemon_move::pokemon_move::SavedPokemonMove}, util::traits::PersistantData};
use crate::util::traits::PersistantDataLocation;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use serde_derive::{Deserialize, Serialize};
use crate::entity::util::direction::Direction;
use std::env::current_exe;
use std::fs::read_to_string;

static SAVE_FILENAME: &str = "player.json";
#[derive(Serialize, Deserialize)]
pub struct PlayerData {

	pub location: Location,
	pub party: Party,

}

impl Default for PlayerData {
    fn default() -> Self {
		Self {
			
			party: Party {

				pokemon: Vec::new(),

			},

			location: Location {

				world_id: String::from("firered"),
				map_set_id: String::from("pallet_town_player_house"),
				map_set_num: 1,

				x: 6,
				y: 6,
				direction: String::from(Direction::Down.value()),
			},
		}
    }
}

impl PlayerData {

	pub fn add_pokemon_to_party(&mut self, pokemon: OwnedPokemon) {
		self.party.pokemon.push(SavedPokemon::from_owned(pokemon));
	}

    pub fn load_from_file() -> PlayerData {
        match read_to_string(get_path().with_file_name(SAVE_FILENAME)) {
            Ok(content) => {
//				println!("{}", content);
				match serde_json::from_str(content.as_str()) {
					Ok(data) => {
						return data;
					}
					Err(e) => {
						println!("Error parsing save: {}", e);
						return new_save();
					}
				}
			}
            Err(err) => {
				println!("Error opening save file at {:?} with error {}", get_path(), err);
                return new_save();
            }
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
						println!("Error parsing save: {}", e);
						return new_save();
					}
				}
			}
            Err(err) => {
				println!("Error opening save file at {:?} with error {}", get_path(), err);
                return new_save();
            }
        }
	}

	fn save(&self) {
		let file = File::create(get_path().with_file_name(SAVE_FILENAME)).unwrap();
        let mut writer = BufWriter::new(file);
		println!("Saving game...");
        match serde_json::to_string_pretty(&self) {
            Ok(encoded) => {
                if let Err(e) = writer.write(encoded.as_bytes()) {
					println!("{}", encoded);
                    println!("WARNING: Failed to encode with error: {}", e);
                }
            }
            Err(e) => {
                println!("WARNING: Failed to save settings: {}", e);
            }
        }
	}

}

fn get_path() -> PathBuf {
	match current_exe() {
		Ok(mut exe_path) => {
			exe_path.pop();
			exe_path.push("saves");
			return exe_path;
		}
		Err(e) => {
			println!("WARNING: Failed to find exe path with error {}", e);
			let mut pb = PathBuf::from("./");
			pb.push("saves");
			return pb;
		}
	}
    
}

fn new_save() -> PlayerData {
	println!("Creating a new save file.");
	let default = PlayerData::default();
	default.save();
	return default;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {

	pub world_id: String,
	pub map_set_id: String,
	pub map_set_num: usize,
	pub x: isize,
	pub y: isize,
	pub direction: String,

}

#[derive(Serialize, Deserialize)]
pub struct Party {

	pub pokemon: Vec<SavedPokemon>,

}

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedPokemon {

	pub pokemon_id: usize,
	pub level: u8,
	pub ivs: StatSet,
	pub evs: StatSet,
	pub moves: [Option<SavedPokemonMove>; 4],
	pub exp: usize,
	pub friendship: u8,

}

impl SavedPokemon {

	pub fn from_owned(pokemon: OwnedPokemon) -> Self {

		let mut vec_moves: Vec<Option<SavedPokemonMove>> = pokemon.instance.moves.iter().map(|moves| Some(SavedPokemonMove {
			name: moves.move_instance.name.clone(),
			pp: moves.remaining_pp,
		})).collect();

		while vec_moves.len() < 4 {
			vec_moves.push(None);
		}

		let arr = [vec_moves[0].clone(), vec_moves[1].clone(), vec_moves[2].clone(), vec_moves[3].clone()];

		Self {

			pokemon_id: pokemon.instance.pokemon.number,
			level: pokemon.instance.level,
			ivs: pokemon.instance.ivs,
			evs: pokemon.instance.evs,
			moves: arr,
			exp: pokemon.exp,
			friendship: pokemon.friendship,

		}

	}

	pub fn to_instance(&self, pokedex: &Pokedex) -> OwnedPokemon {

		OwnedPokemon {

			instance: PokemonInstance {

				pokemon: pokedex.pokemon_from_id(self.pokemon_id).clone(),

				moves: SavedPokemonMove::to_instance(&self.moves, pokedex),

				level: self.level,

				ivs: self.ivs,
				evs: self.evs,

			},

			exp: self.exp,
			friendship: self.friendship,

		}

	}

}