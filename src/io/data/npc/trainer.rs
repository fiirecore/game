use serde::{Deserialize, Serialize};

use crate::io::data::pokemon::pokemon_party::PokemonParty;

#[derive(Clone, Debug, Deserialize)]
pub struct Trainer {

    pub trainer_type: TrainerType,
    pub party: PokemonParty,

}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum TrainerType {

    None,
    GymLeader,
    
}

impl TrainerType {

    pub fn to_string(&self) -> &str {
        match *self {
            TrainerType::None => {
                "Trainer"
            }
            TrainerType::GymLeader => {
                "Gym Leader"
            }
        }
    }

}