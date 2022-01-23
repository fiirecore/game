use serde::{Deserialize, Serialize};
use worldcli::worldlib::{
    character::player::PlayerCharacter, positions::{Position, Location},
};

// mod list;

// pub use list::PlayerSaves;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(default = "Player::default_id")]
    pub id: u64,

    #[serde(default = "Player::default_version")]
    pub version: String,

    #[serde(default)]
    pub player: PlayerData,
}

impl Player {

    pub fn new(player: PlayerData) -> Self {
        Self {
            player,
            ..Default::default()
        }
    }

    pub fn default_version() -> String {
        crate::VERSION.to_owned()
    }
    
    pub fn default_id() -> u64 {
        crate::engine::utils::seed()
    }
    

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerData {
    Character(PlayerCharacter),
    Named(String, String),
    None,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::None
    }
}

impl PlayerData {
    pub fn create(&mut self, spawn: (Location, Position)) {
        *self = Self::Character(match std::mem::take(self) {
            PlayerData::Named(name, rival) => PlayerCharacter::new(name, rival, spawn),
            PlayerData::None => PlayerCharacter::new("Red".to_owned(), "Blue".to_owned(), spawn),
            PlayerData::Character(c) => c,
        })
    }

    pub fn as_ref(&self) -> Option<&PlayerCharacter> {
        match self {
            PlayerData::Character(c) => Some(c),
            _ => None
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut PlayerCharacter> {
        match self {
            PlayerData::Character(c) => Some(c),
            _ => None
        }
    }

    pub fn unwrap(&mut self) -> &mut PlayerCharacter {
        match self {
            PlayerData::Character(c) => c,
            _ => panic!("Cannot get player character as it is unintialized!")
        }
    }

}

impl storage::PersistantData for Player {
    fn path() -> &'static str {
        "save"
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            version: Self::default_version(),
            player: Default::default(),
        }
    }
}