use std::{ffi::OsString, path::Path};
use log::warn;
use serde::Deserialize;

use crate::{game::pokedex::pokemon::pokemon::PokemonType, util::file_util::UNKNOWN_FILENAME_ERR};

use super::pokemon_move::PokemonMove;
use super::move_category::MoveCategory;

pub fn load_move_from_toml<P>(path: P) -> Option<PokemonMove> where P: AsRef<Path> {
    let path = path.as_ref();
    
    let string_result = std::fs::read_to_string(path);
    
    match string_result {
        Ok(string) => {
            let read_toml: Result<TomlPokemonMove, toml::de::Error> = toml::from_str(string.as_str());
            match read_toml {
                Ok(pkmn_move) => {

                    Some(PokemonMove {
                        name: pkmn_move.name.clone(),
                        category: MoveCategory::from_string(pkmn_move.category.as_str()).unwrap_or_else(|| {
                            warn!("Could not get move category for {}, setting to physical", pkmn_move.name);
                            MoveCategory::Physical
                        }),
                        pokemon_type: PokemonType::from_string(pkmn_move.pokemon_type.as_str()),
                        power: pkmn_move.power,
                        accuracy: pkmn_move.accuracy,
                        pp: pkmn_move.pp,
                    })
                },
                Err(e) => {
                    warn!("Could not parse pokemon move toml at {:?} with error: {}", path.file_name().unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)), e);
                    return None;
                }
            }
        },
        Err(err) => {
            warn!("Could not read move toml at {:?} with error {}", path.file_name().unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)), err);
            return None;
        }
    }

    

}

#[derive(Debug, Deserialize)]
pub struct TomlPokemonMove {

    pub name: String,
    pub category: String,
    pub pokemon_type: String,
    pub power: Option<usize>,
    pub accuracy: Option<u8>,
    pub pp: u8,

}