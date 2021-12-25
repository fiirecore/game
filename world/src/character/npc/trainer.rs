use std::ops::{Deref, DerefMut};

use hashbrown::HashSet;
use serde::{Deserialize, Serialize};

use crate::character::{npc::NpcId, trainer::Trainer};

pub type BadgeId = tinystr::TinyStr16;
pub type TransitionId = tinystr::TinyStr8;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NpcTrainer {

    pub character: Trainer,

    /// The trainer tracks a certain amount of tiles in front of them
    pub tracking: Option<u8>,

    pub encounter: Vec<Vec<String>>,
    #[serde(default = "default_battle_transition")]
    pub transition: TransitionId,

    pub defeat: Vec<Vec<String>>,

    #[serde(default)]
    pub badge: Option<BadgeId>,

    #[serde(default)]
    pub disable: TrainerDisable,
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

fn default_battle_transition() -> TransitionId {
    "default".parse().unwrap()
}

impl Deref for NpcTrainer {
    type Target = Trainer;

    fn deref(&self) -> &Self::Target {
        &self.character
    }
}

impl DerefMut for NpcTrainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.character
    }
}
