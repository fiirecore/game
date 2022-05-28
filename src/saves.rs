use serde::{Deserialize, Serialize};
use worldcli::worldlib::{
    character::player::SavedPlayerCharacter,
};

// mod list;

// pub use list::PlayerSaves;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(default = "Player::default_version")]
    pub version: String,

    #[serde(default)]
    pub player: SavedPlayerCharacter,
}

impl Player {
    pub fn new(name: impl Into<String>, rival: impl Into<String>) -> Self {
        Self {
            player: SavedPlayerCharacter::new(name, rival),
            version: Self::default_version(),
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
            player: Default::default(),
        }
    }
}
