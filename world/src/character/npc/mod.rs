use crate::positions::Coordinate;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use tinystr::TinyStr8;

use self::trainer::Trainer;
use super::Character;

mod interact;
pub use interact::*;

mod npc_type;
pub use npc_type::*;

pub mod trainer;

pub type NpcId = TinyStr8;
pub type Npcs = HashMap<NpcId, Npc>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Npc {
    pub name: String,

    #[serde(rename = "type")]
    pub type_id: NpcTypeId,

    pub character: Character,

    #[serde(default)]
    pub movement: NpcMovement,
    #[serde(skip, default)]
    pub origin: Option<Coordinate>,

    #[serde(default)]
    pub interact: NpcInteract,

    pub trainer: Option<Trainer>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum NpcMovement {
    Still,
    LookAround,
    WalkUpAndDown(u8),
}

impl Default for NpcMovement {
    fn default() -> Self {
        Self::Still
    }
}
