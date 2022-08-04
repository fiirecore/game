use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::{
    character::CharacterGroupId,
    map::object::ObjectId,
    positions::{Coordinate, Direction, Position},
};

pub mod group;
pub mod trainer;

pub type NpcId = ObjectId; //tinystr::TinyStr8;
pub type Npcs = HashMap<NpcId, Npc>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Npc {
    pub id: NpcId,

    pub name: String,

    pub origin: Position,
    /// The NPC's type.
    /// This determines the texture of the NPC,
    /// what color their message text is,
    /// and what song is played on an encounter
    #[serde(rename = "type")]
    pub group: CharacterGroupId,
    #[serde(default)]
    pub movement: Vec<NpcMovement>,

    pub trainer: Option<trainer::NpcTrainer>,
}

/// to - do: implement non-random npc movement (like spinners)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum NpcMovement {
    Look(HashSet<Direction>),
    Move(Coordinate),
}
