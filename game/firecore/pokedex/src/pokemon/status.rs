use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum PokemonStatus {

    Paralysis,
    Poison,
    Sleep,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct StatusEffect {

    pub status: PokemonStatus,
    pub remaining: Option<u8>,

}