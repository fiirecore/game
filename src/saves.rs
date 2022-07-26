use rand::prelude::SmallRng;
use serde::{Deserialize, Serialize};
use worldcli::{worldlib::{state::WorldState, script::default::DefaultWorldScriptEngine}, pokedex::trainer::SavedTrainer};

use crate::random::GamePseudoRandom;

// mod list;

// pub use list::PlayerSaves;

pub type GameWorldState = WorldState<DefaultWorldScriptEngine>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(default = "Player::default_version")]
    pub version: String,

    #[serde(default)]
    pub world: GameWorldState,

    #[serde(default)]
    pub trainer: SavedTrainer,
}

impl Player {
    pub fn new(name: impl Into<String>, rival: impl Into<String>) -> Self {
        Self {
            version: Self::default_version(),
            world: GameWorldState::new(name, rival),
            trainer: Default::default(),
        }
    }

    pub fn default_version() -> String {
        crate::VERSION.to_owned()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            version: Self::default_version(),
            world: Default::default(),
            trainer: Default::default(),
        }
    }
}
