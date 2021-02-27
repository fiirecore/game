use serde::{Deserialize, Serialize};

use crate::pokemon::data::StatSet;

use super::PokemonId;
use super::moves::serializable::SerializableMoveSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonInstance {

	pub id: PokemonId,
    pub nickname: Option<String>,
    pub level: u8,
    
    #[serde(default = "iv_default")]
	pub ivs: StatSet,
    #[serde(default)]
    pub evs: StatSet,
    
    pub moves: Option<SerializableMoveSet>,
    
    #[serde(default)]
	pub exp: usize,
    #[serde(default)]
    pub friendship: u8,
    
    pub current_hp: Option<u16>,

}

impl PokemonInstance {

    pub fn generate(pokemon_id: PokemonId, min_level: u8, max_level: u8, ivs: Option<StatSet>) -> Self {

        Self {

            id: pokemon_id,
            nickname: None,
            level: macroquad::rand::gen_range(min_level, max_level),
            ivs: ivs.unwrap_or_default(),
            evs: StatSet::default(),
            current_hp: None,
            moves: None,
            exp: 0,
            friendship: 70,

        }

    }
    
    pub fn generate_with_level(pokemon_id: PokemonId, level: u8, ivs: Option<StatSet>) -> Self {
        PokemonInstance::generate(pokemon_id, level, level, ivs)
    }

}

fn iv_default() -> StatSet {
    StatSet::uniform(15)
}