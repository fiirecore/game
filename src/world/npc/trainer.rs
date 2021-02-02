use serde::{Deserialize, Serialize};

use crate::battle::battle_info::BattleType;
use crate::pokemon::party::PokemonParty;

#[derive(Clone, Debug, Deserialize)]
pub struct Trainer {

    pub trainer_type: TrainerType,
    pub party: PokemonParty,
    pub battled: bool,

    pub tracker: Option<Tracker>,

    pub victory_message: Vec<String>,
    pub worth: u16,

}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub enum TrainerType {

    None,
    Camper,
    GymLeader,
    
}

impl TrainerType {

    pub fn battle_type(&self) -> BattleType {
        match *self {
            TrainerType::GymLeader => BattleType::GymLeader,
            _ => BattleType::Trainer,
        }
    }

}

impl std::fmt::Display for TrainerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            TrainerType::None => "Trainer",
            TrainerType::Camper => "Camper",
            TrainerType::GymLeader => "Gym Leader",
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tracker {

    pub length: u8,

}