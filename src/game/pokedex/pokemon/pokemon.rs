use std::{ffi::OsString, path::Path};
use std::collections::HashMap;

use log::warn;

use crate::{game::pokedex::pokemon::pokemon_toml::TomlPokemonConfig, util::file_util::UNKNOWN_FILENAME_ERR};

use std::fs::read_to_string;


#[derive(Clone)]
pub struct Pokemon {

	pub number: usize,
	pub name: String,
	pub primary_type: PokemonType,
	pub secondary_type: Option<PokemonType>,
	pub species: String,
	pub height: f32,
	pub weight: f32,
	
	pub base_hp: u8,
	pub base_atk: u8,
	pub base_def: u8,
	pub base_sp_atk: u8,
	pub base_sp_def: u8,
	pub base_speed: u8,
	
	pub learnable_moves: HashMap<u8, Vec<String>>, // level, move
	
	pub path_normal_front: String,
	pub path_normal_back: String,
	
}

impl Pokemon {
	
	pub fn empty() -> Pokemon {

		Pokemon {
			
			number: 0,
			name: "None".to_string(),
			primary_type: PokemonType::Normal,
			secondary_type: None,
			species: "None".to_string(),
			
			height: 0f32,
			weight: 0f32,
	
			base_hp: 0,
			base_atk: 0,
			base_def: 0,
			base_sp_atk: 0,
			base_sp_def: 0,
			base_speed: 0,
			
			learnable_moves: HashMap::new(),
			
			path_normal_front: "debug/missing_texture.png".to_string(),
			path_normal_back: "debug/missing_texture.png".to_string(),
			
		}
		
	}
	
	pub fn new<P>(path: P) -> Option<Pokemon> where P: AsRef<Path> {
		let path = path.as_ref();
	
		let string_result = read_to_string(path);

		match string_result {

			Ok(string) => {

				let toml_result: Result<TomlPokemonConfig, toml::de::Error> = toml::from_str(&string);

				match toml_result {
					Ok(toml) => {
						let pokedex_data = toml.pokedex_data;
						let base_stats = toml.base_stats.unwrap();
						
						//println!("Loaded data for: {}", &pokedex_data.name.clone().unwrap());
						
						let primary_type = PokemonType::from_string(&pokedex_data.primary_type).unwrap();
						
						let secondary_type: Option<PokemonType>;
						
						if pokedex_data.secondary_type.is_some() {
							secondary_type = Some(PokemonType::from_string(&pokedex_data.secondary_type.unwrap()).unwrap());
						} else {
							secondary_type = None;
						}
					
						let number = pokedex_data.number;

						let name = pokedex_data.name;
						
						let mut path1 = String::from("pokedex/textures/normal/front/");
						path1.push_str(&name.to_lowercase());
						path1.push_str(".png");
					
						let mut path2 = String::from("pokedex/textures/normal/back/");
						path2.push_str(&name.to_lowercase());
						path2.push_str(".png");
				
						let mut moves: HashMap<u8, Vec<String>> = HashMap::new();
				
						match toml.moves {
							Some(learnable_moves) => {
								//println!("{:?}", learnable_moves);
								for learnable_move in learnable_moves {
									if moves.contains_key(&learnable_move.level) {
										let vec = moves.get_mut(&learnable_move.level).unwrap();
										vec.push(learnable_move.move_id.clone());
									} else {
										moves.insert(learnable_move.level, vec![learnable_move.move_id.clone()]);
									}							
								}
							}
							None => {
		
							}
						}
				
						Some(Pokemon {
						
							number: number,
							name: name,
							primary_type: primary_type,
							secondary_type: secondary_type,
							species: pokedex_data.species.unwrap(),
							height: pokedex_data.height.unwrap(),
							weight: pokedex_data.weight.unwrap(),
							
							base_hp: base_stats.hp.unwrap() as u8,
							base_atk: base_stats.atk.unwrap() as u8,
							base_def: base_stats.def.unwrap() as u8,
							base_sp_atk: base_stats.sp_atk.unwrap() as u8,
							base_sp_def: base_stats.sp_def.unwrap() as u8,
							base_speed: base_stats.speed.unwrap() as u8,
							
							learnable_moves: moves,
							
							path_normal_front: path1,
							path_normal_back: path2,
							
				//			front_image: image_from_path(&path1).unwrap(),//Texture::from_path(Path::new(&path1), &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest)).unwrap(),
				//			back_image: image_from_path(&path2).unwrap(),//Texture::from_path(Path::new(&path2), &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest)).unwrap(),
						
						})
					}
					Err(e) => {
						warn!("Could not parse pokemon toml at {:?} with error {}", path.file_name().unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)), e);
						return None;
					}
				}

			}

			Err(err) => {

				warn!("Error reading pokemon entry at {:?} to string with error: {}", path.file_name().unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)), err);
				return None;
	
			}

		}

		
		
		
		
	}
	
	pub fn load(&mut self) {
		
	}
	
	//pub fn render_front(&mut self, c: &mut Context, g: &mut GlGraphics, x: isize, y: isize) {
	//	draw_image(c, gl,&self.front_image, x, y);
	//}
	
	//pub fn render_back(&mut self, c: &mut Context, g: &mut GlGraphics, x: isize, y: isize) {
	//	draw_image(c, gl,&self.back_image, x, y);
	//}
	
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum Gender {
	
	None,
	Male,
	Female,
	
}

impl Gender {
	
	#[allow(dead_code)]
    pub fn value(&self) -> &str {
		match *self {
			Gender::None => "None",
			Gender::Male => "Male",
			Gender::Female => "Female",
		}
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum Stat {
	
	HP,
	Attack,
	Defense,
	SpecialAttack,
	SpecialDefense,
	Speed,
	
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum PokemonType {
	
	Normal,
	Fire,
	Water,
	Electric,
	Grass,
	Ice,
	Fighting,
	Poison,
	Ground,
	Flying,
	Psychic,
	Bug,
	Rock,
	Ghost,
	Dragon,
	Dark,
	Steel,
	Fairy,
	
}

impl PokemonType {
	
	#[allow(dead_code)]
    pub fn value(&self) -> &str {
		match *self {
			PokemonType::Normal => "Normal",
			PokemonType::Fire => "Fire",
			PokemonType::Water => "Water",
			PokemonType::Electric => "Electric",
			PokemonType::Grass => "Grass",
			PokemonType::Ice => "Ice",
			PokemonType::Fighting => "Fighting",
			PokemonType::Poison => "Poison",
			PokemonType::Ground => "Ground",
			PokemonType::Flying => "Flying",
			PokemonType::Psychic => "Psychic",
			PokemonType::Bug => "Bug",
			PokemonType::Rock => "Rock",
			PokemonType::Ghost => "Ghost",
			PokemonType::Dragon => "Dragon",
			PokemonType::Dark => "Dark",
			PokemonType::Steel => "Steel",
			PokemonType::Fairy => "Fairy",
		}
    }

	pub fn from_string(string: &str) -> Option<PokemonType> {
		match string {
			"Normal" => Some(PokemonType::Normal),
			"Fire" => Some(PokemonType::Fire),
			"Water" => Some(PokemonType::Water),
			"Electric" => Some(PokemonType::Electric),
			"Grass" => Some(PokemonType::Grass),
			"Ice" => Some(PokemonType::Ice),
			"Fighting" => Some(PokemonType::Fighting),
			"Poison" => Some(PokemonType::Poison),
			"Ground" => Some(PokemonType::Ground),
			"Flying" => Some(PokemonType::Flying),
			"Psychic" => Some(PokemonType::Psychic),
			"Bug" => Some(PokemonType::Bug),
			"Rock" => Some(PokemonType::Rock),
			"Ghost" => Some(PokemonType::Ghost),
			"Dragon" => Some(PokemonType::Dragon),
			"Dark" => Some(PokemonType::Dark),
			"Steel" => Some(PokemonType::Steel),
			"Fairy" => Some(PokemonType::Fairy),
			&_ => None,
		}
	}

}