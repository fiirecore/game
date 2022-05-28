use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::{
    character::Character,
    positions::{Coordinate, Direction},
};

mod interact;
pub use interact::*;

pub mod group;
pub mod trainer;

pub type NpcId = tinystr::TinyStr8;
pub type Npcs = HashMap<NpcId, Npc>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Npc {
    pub id: NpcId,

    pub character: Character,
    /// The NPC's type.
    /// This determines the texture of the NPC,
    /// what color their message text is,
    /// and what song is played on an encounter
    #[serde(rename = "type")]
    pub group: group::NpcGroupId,
    #[serde(default)]
    pub movement: Vec<NpcMovement>,
    #[serde(default)]
    pub origin: Option<Coordinate>,

    #[serde(default)]
    pub interact: NpcInteract,

    pub trainer: Option<trainer::NpcTrainer>,
}

/// to - do: implement non-random npc movement (like spinners)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum NpcMovement {
    Look(HashSet<Direction>),
    Move(Coordinate),
}
