use std::fmt::Display;
use std::path::Path;

use serde::Deserialize;

use log::warn;

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
    
    pub fn load_move<P>(path: P) -> Option<PokemonMove> where P: AsRef<Path> {
        let path = path.as_ref();
        
        match std::fs::read_to_string(path) {
            Ok(move_string) => {
                let read_toml: Result<PokemonMove, toml::de::Error> = toml::from_str(move_string.as_str());
                match read_toml {
                    Ok(pkmn_move) => {
                        Some(pkmn_move)
                    // Some(PokemonMove {
                    //     name: pkmn_move.name.clone(),
                    //     category: MoveCategory::from_string(pkmn_move.category.as_str()).unwrap_or_else(|| {
                    //         warn!("Could not get move category for {}, setting to physical", pkmn_move.name);
                    //         MoveCategory::Physical
                    //     }),
                    //     pokemon_type: PokemonType::from_string(pkmn_move.pokemon_type.as_str()),
                    //     power: pkmn_move.power,
                    //     accuracy: pkmn_move.accuracy,
                    //     pp: pkmn_move.pp,
                    // })
                    },
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

}

impl Display for PokemonMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}