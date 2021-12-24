use serde::{Deserialize, Serialize};
use hashbrown::HashSet;

use pokedex::pokemon::{owned::SavedPokemon, party::Party};

use crate::character::npc::NpcId;

pub type BadgeId = tinystr::TinyStr16;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Trainer {
    #[serde(default = "crate::default_true")]
    #[deprecated(note = "find a better solution")]
    pub battle_on_interact: bool,
    /// The trainer tracks a certain amount of tiles in front of them
    pub tracking: Option<u8>,

    pub encounter: Vec<Vec<String>>,

    pub defeat: Vec<Vec<String>>,

    #[serde(default = "default_battle_transition")]
    #[deprecated(note = "Will be moved to WorldMap")]
    pub transition: crate::map::TransitionId,

    pub party: Party<SavedPokemon>,

    #[serde(default)]
    pub badge: Option<BadgeId>,

    #[serde(default)]
    pub disable: TrainerDisable,
    pub worth: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TrainerDisable {
    #[serde(rename = "Self")]
    DisableSelf,
    Many(HashSet<NpcId>),
    None,
}

impl Default for TrainerDisable {
    fn default() -> Self {
        Self::DisableSelf
    }
}

#[deprecated]
fn default_battle_transition() -> crate::map::TransitionId {
    "default".parse().unwrap()
}
