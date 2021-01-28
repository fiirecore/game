use super::PokedexData;
use super::Pokemon;
use super::PokemonType;

impl Pokemon {

	pub fn texture_path(side: &str, pokemon: &Pokemon) -> String {
		let mut side = side.to_string();
		side.push_str("/");
		let mut name = pokemon.data.name.to_lowercase();
		name.push_str(".png");
		side.push_str(&name);
		side
	}

	// #[deprecated(since = "0.2.0", note = "Include as bytes instead of external file")]
	// pub async fn new<P>(path: P) -> Option<Pokemon> where P: AsRef<Path> {
	// 	let path = path.as_ref();
	// 	match crate::util::file::read_to_string(path).await {
	// 		Ok(data) => {
	// 			match Pokemon::from_string(&data) {
	// 			    Ok(pokemon) => Some(pokemon),
	// 			    Err(err) => {
	// 					warn!("Could not parse pokemon toml with data {} with error {}", &data[0..20], err);
	// 					return None;
	// 				}
	// 			}
	// 		}
	// 		Err(err) => {
	// 			warn!("Error reading pokemon entry at {:?} to string with error {}", path, err);
	// 			return None;
	
	// 		}
	// 	}
	// }

	pub fn from_string(data: &String) -> Result<Pokemon, toml::de::Error> {
		return toml::from_str(data);
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