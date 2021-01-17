use std::path::Path;
use std::path::PathBuf;

use log::warn;

use super::PokedexData;
use super::Pokemon;
use super::PokemonType;

impl Pokemon {

	pub fn texture_path(side: &str, pokemon: &Pokemon) -> PathBuf {
		let mut name = pokemon.data.name.clone();
		name.push_str(".png");
		PathBuf::from("pokedex/textures/normal/").join(side).join(name)
	}

	pub fn new<P>(path: P) -> Option<Pokemon> where P: AsRef<Path> {
		let path = path.as_ref();
	
		match std::fs::read_to_string(path) {
	
			Ok(string) => {
	
				let toml_result: Result<Pokemon, toml::de::Error> = toml::from_str(&string);
				match toml_result {
					Ok(pokemon) => {				
						Some(pokemon)
					}
					Err(err) => {
						warn!("Could not parse pokemon toml at {:?} with error {}", path, err);
						return None;
					}
				}
	
			}
	
			Err(err) => {
	
				warn!("Error reading pokemon entry at {:?} to string with error: {}", path, err);
				return None;
	
			}
	
		}
		
	}
	
}

impl Default for PokedexData {
	fn default() -> Self {
		Self {
			number: 0,
			name: "None".to_string(),
			primary_type: PokemonType::Normal,
			secondary_type: None,
			species: "None".to_string(),
			
			height: 0f32,
			weight: 0f32,
		}
	}
}