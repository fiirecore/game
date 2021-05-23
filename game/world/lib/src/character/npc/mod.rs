use firecore_font::message::MessagePages;
use firecore_util::Coordinate;
use serde::{Deserialize, Serialize};
use deps::str::{TinyStr8, TinyStr16};

use super::Character;
use super::movement::MovementType;
use self::trainer::Trainer;

pub mod npc_type;

pub mod trainer;

pub mod interact;

pub type NPCId = TinyStr8;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NPC {

    pub name: String,

    #[serde(rename = "type")]
    pub npc_type: TinyStr16,
    
    pub character: Character,

    #[serde(default)]
    pub movement: MovementType,
    #[serde(skip, default)]
    pub origin: Option<Coordinate>,

    pub message: Option<MessagePages>,

    pub trainer: Option<Trainer>,

}