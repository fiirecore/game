use serde::{Deserialize, Serialize};

use crate::battle::battle_info::BattleType;
use crate::io::data::pokemon::pokemon_party::PokemonParty;

#[derive(Clone, Debug, Deserialize)]
pub struct Trainer {

    pub trainer_type: TrainerType,
    pub party: PokemonParty,

    pub tracker: Option<Tracker>,

    pub victory_message: Vec<String>,
    pub worth: u16,

}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub enum TrainerType {

    None,
    GymLeader,
    
}

impl TrainerType {

    pub fn battle_type(&self) -> BattleType {
        match *self {
            TrainerType::None => BattleType::Trainer,
            TrainerType::GymLeader => BattleType::GymLeader,
        }
    }

}

impl ToString for TrainerType {

    fn to_string(&self) -> String {
        match *self {
            TrainerType::None => "Trainer",
            TrainerType::GymLeader => "Gym Leader",
        }.to_string()
    }

}

#[derive(Clone, Debug, Deserialize)]
pub struct Tracker {

    pub length: u8,

}