use serde::{Deserialize, Serialize};

use crate::audio::music::Music;
use crate::battle::battle_info::BattleType;
use crate::battle::transitions::managers::battle_screen_transition_manager::BattleScreenTransitions;
use crate::pokemon::party::PokemonParty;

#[derive(Debug, Deserialize)]
pub struct Trainer {

    pub trainer_type: TrainerType,

    pub encounter_music: Option<Music>,
    //#[serde(default)]
    pub encounter_message: Vec<Vec<String>>,
    pub victory_message: Vec<String>,
    pub worth: u16,

    pub tracking_length: Option<usize>,
    pub transition: Option<BattleScreenTransitions>,

    pub party: PokemonParty,
    //pub battled: bool,

}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
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