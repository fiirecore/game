use std::fmt::Display;
use std::path::Path;

use serde::Deserialize;

use macroquad::prelude::warn;

use crate::game::pokedex::move_instance::MoveInstance;
use crate::io::data::pokemon::PokemonType;

use super::MoveCategory;

#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize)]
pub struct PokemonMove {

	pub name: String,
	pub category: MoveCategory,
	pub pokemon_type: Option<PokemonType>,
	pub power: Option<usize>,
	pub accuracy: Option<u8>,
	pub pp: u8,
	
}

impl PokemonMove {

	pub fn empty() -> Self {
		Self {
			name: String::new(),
			category: MoveCategory::Status,
			pokemon_type: None,
			power: None,
			accuracy: None,
			pp: 0,
		}
	}

	pub fn to_instance(&self) -> MoveInstance {
		MoveInstance {
			move_instance: self.clone(),
			remaining_pp: self.pp
		}
    }
    
    #[deprecated(since = "0.2.0", note = "Include as bytes instead of external file")]
    pub async fn load_move<P>(path: P) -> Option<PokemonMove> where P: AsRef<Path> {
        let path = path.as_ref();
        
        match crate::util::file::read_to_string(path).await {
            Ok(ref data) => {
                match PokemonMove::from_string(data) {
                    Ok(pkmn_move) => Some(pkmn_move),
                    Err(err) => {
                        warn!("Could not parse pokemon move toml at {:?} with error: {}", path, err);
                        return None;
                    }
                }
            },
            Err(err) => {
                warn!("Could not read move toml at {:?} with error {}", path, err);
                return None;
            }
        }
    
    }

    pub fn from_string(data: &String) -> Result<PokemonMove, toml::de::Error> {
        return toml::from_str(data);
    }


}

impl Display for PokemonMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}