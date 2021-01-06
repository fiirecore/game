use serde_derive::{Deserialize, Serialize};

use super::pokemon_party::PokemonParty;

#[derive(Clone, Debug, Deserialize)]
pub struct Trainer {

    #[deprecated = "fix"]
    pub sprite: u8,

    pub trainer_type: TrainerType,
    pub party: PokemonParty,

}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum TrainerType {

    GymLeader,
    
}