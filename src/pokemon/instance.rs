use serde::{Deserialize, Serialize};

use crate::pokemon::data::StatSet;

use super::PokemonId;
use super::moves::instance::SavedPokemonMoveSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonInstance {

	pub id: PokemonId,
    pub level: u8,
    
	pub ivs: Option<StatSet>,
    pub evs: Option<StatSet>,
    
    pub move_set: Option<SavedPokemonMoveSet>,
    
	pub exp: Option<usize>,
    pub friendship: Option<u8>,
    
    pub current_hp: Option<u16>,

}

impl PokemonInstance {

    pub fn generate(pokemon_id: PokemonId, min_level: u8, max_level: u8, ivs: Option<StatSet>) -> Self {

        Self {

            id: pokemon_id,
            level: macroquad::rand::gen_range(min_level, max_level),
            ivs: ivs,
            evs: None,
            current_hp: None,
            move_set: None,
            exp: None,
            friendship: None,

        }

    }
    
    pub fn generate_with_level(pokemon_id: PokemonId, level: u8, ivs: Option<StatSet>) -> Self {
        PokemonInstance::generate(pokemon_id, level, level, ivs)
    }

}