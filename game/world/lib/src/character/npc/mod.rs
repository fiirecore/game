use firecore_util::Coordinate;
use serde::{Deserialize, Serialize};
use deps::str::{TinyStr8, TinyStr16};

use super::Character;
use super::movement::MovementType;
use self::trainer::Trainer;

mod interact;
pub use interact::*;

pub mod npc_type;

pub mod trainer;

pub type NpcId = TinyStr8;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Npc {

    pub name: String,

    #[serde(rename = "type")]
    pub npc_type: TinyStr16,
    
    pub character: Character,

    #[serde(default)]
    pub movement: MovementType,
    #[serde(skip, default)]
    pub origin: Option<Coordinate>,

    #[serde(default)]
    pub interact: NpcInteract,

    pub trainer: Option<Trainer>,

}

impl Npc {

    pub fn default_npc() -> Self {
        Self {
            name: "Default".to_string(),
            npc_type: "default".parse().unwrap(),
            character: Default::default(),
            movement: Default::default(),
            origin: None,
            interact: Default::default(),
            trainer: None,
        }
    }

}