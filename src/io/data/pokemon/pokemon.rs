use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;

use log::warn;

use super::StatSet;
use super::pokemon_toml::PokedexData;
use super::pokemon_toml::PokemonConfig;


#[derive(Clone, Default)]
pub struct Pokemon {

	pub data: PokedexData,	
	pub base: StatSet,
	pub learnable_moves: HashMap<u8, Vec<String>>,
	
}

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
	
				let toml_result: Result<PokemonConfig, toml::de::Error> = toml::from_str(&string);
				match toml_result {
					Ok(toml) => {                    
	
						let mut moves: HashMap<u8, Vec<String>> = HashMap::new();
				
						for learnable_move in toml.moves {
							if let Some(vec) = moves.get_mut(&learnable_move.level) {
								vec.push(learnable_move.move_id.clone());
							}
							moves.insert(learnable_move.level, vec![learnable_move.move_id.clone()]);						
						}
				
						Some(Pokemon {
						
							data: toml.pokedex_data,							
							base: toml.base_stats,
							
							learnable_moves: moves,
						
						})
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

// impl PokemonType {
	
// 	#[allow(dead_code)]
//     pub fn value(&self) -> &str {
// 		match *self {
// 			PokemonType::Normal => "Normal",
// 			PokemonType::Fire => "Fire",
// 			PokemonType::Water => "Water",
// 			PokemonType::Electric => "Electric",
// 			PokemonType::Grass => "Grass",
// 			PokemonType::Ice => "Ice",
// 			PokemonType::Fighting => "Fighting",
// 			PokemonType::Poison => "Poison",
// 			PokemonType::Ground => "Ground",
// 			PokemonType::Flying => "Flying",
// 			PokemonType::Psychic => "Psychic",
// 			PokemonType::Bug => "Bug",
// 			PokemonType::Rock => "Rock",
// 			PokemonType::Ghost => "Ghost",
// 			PokemonType::Dragon => "Dragon",
// 			PokemonType::Dark => "Dark",
// 			PokemonType::Steel => "Steel",
// 			PokemonType::Fairy => "Fairy",
// 		}
//     }

// 	pub fn from_string(string: &str) -> Option<PokemonType> {
// 		match string {
// 			"Normal" => Some(PokemonType::Normal),
// 			"Fire" => Some(PokemonType::Fire),
// 			"Water" => Some(PokemonType::Water),
// 			"Electric" => Some(PokemonType::Electric),
// 			"Grass" => Some(PokemonType::Grass),
// 			"Ice" => Some(PokemonType::Ice),
// 			"Fighting" => Some(PokemonType::Fighting),
// 			"Poison" => Some(PokemonType::Poison),
// 			"Ground" => Some(PokemonType::Ground),
// 			"Flying" => Some(PokemonType::Flying),
// 			"Psychic" => Some(PokemonType::Psychic),
// 			"Bug" => Some(PokemonType::Bug),
// 			"Rock" => Some(PokemonType::Rock),
// 			"Ghost" => Some(PokemonType::Ghost),
// 			"Dragon" => Some(PokemonType::Dragon),
// 			"Dark" => Some(PokemonType::Dark),
// 			"Steel" => Some(PokemonType::Steel),
// 			"Fairy" => Some(PokemonType::Fairy),
// 			&_ => None,
// 		}
// 	}

// }