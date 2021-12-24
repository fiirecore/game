use crate::positions::Coordinate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::trainer::Trainer;
use super::Character;

mod interact;
pub use interact::*;

pub mod group;
pub mod trainer;

pub type NpcId = tinystr::TinyStr8;
pub type Npcs = HashMap<NpcId, Npc>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Npc {
    pub character: Character,
    /// The NPC's type.
    /// This determines the texture of the NPC,
    /// what color their message text is,
    /// and what song is played on an encounter
    #[serde(rename = "type")]
    pub group: group::NpcGroupId,
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
