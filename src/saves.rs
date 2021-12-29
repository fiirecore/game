extern crate firecore_world as worldlib;

use serde::{Deserialize, Serialize};
use worldlib::{
    character::{player::PlayerCharacter, Character},
    map::manager::state::{default_location, default_position},
};

// mod list;

// pub use list::PlayerSaves;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(default = "default_id")]
    pub id: u64,

    #[serde(default = "default_character", rename = "player")]
    pub character: PlayerCharacter,
}

impl Player {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            character: PlayerCharacter {
                rival: "Gary".to_owned(),
                character: Character {
                    name: name.into(),
                    position: default_position(),
                    ..Default::default()
                },
                ..default_character()
            },
            ..Default::default()
        }
    }

    // pub fn save(&self) -> Result<(), DataError> {
    //     info!("Saving player data!");
    //     if let Err(err) = crate::storage::save(
    //         self,
    //         crate::PUBLISHER,
    //         crate::APPLICATION,
    //         PathBuf::from("saves").join(&format!("{}-{}.ron", self.character.name, self.id)),
    //     ) {
    //         warn!("Could not save player data with error: {}", err);
    //     }
    //     Ok(())
    // }
}

impl storage::PersistantData for Player {
    fn path() -> &'static str {
        "save"
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: default_id(),
            // party: Default::default(),
            character: default_character(),
        }
    }
}

pub fn default_id() -> u64 {
    crate::engine::utils::seed()
}

pub fn default_name() -> &'static str {
    "Red"
}

pub fn default_rival() -> &'static str {
    "Gary"
}

pub fn default_character() -> PlayerCharacter {
    PlayerCharacter {
        location: default_location(),
        character: Character::new(default_name(), default_position()),
        trainer: Default::default(),
        pc: Default::default(),
        input_frozen: false,
        ignore: false,
        world: Default::default(),
        rival: default_rival().to_owned(),
    }
}
