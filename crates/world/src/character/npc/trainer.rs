use std::ops::{Deref, DerefMut};

use hashbrown::HashSet;
use serde::{Deserialize, Serialize};

use crate::{
    character::{npc::NpcId, trainer::Trainer, Character},
    positions::Destination,
};

use super::group::TrainerGroupId;

pub type BadgeId = tinystr::TinyStr16;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NpcTrainer {
    pub group: TrainerGroupId,

    pub character: Trainer,

    /// The trainer tracks a certain amount of tiles in front of them
    pub sight: Option<u8>,

    pub encounter: Vec<Vec<String>>,
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
}

impl NpcTrainer {
    pub fn find_character(&self, character: &mut Character, find: &mut Character) -> bool {
        if self
            .sight
            .map(|sight| character.sees(sight, &find.position))
            .unwrap_or_default()
        {
            character.actions.extend(
                &character.position,
                Destination::next_to(&character.position, find.position.coords),
            );
            character.queue_interact(false);
            true
        } else {
            false
        }
    }
}

impl Default for TrainerDisable {
    fn default() -> Self {
        Self::DisableSelf
    }
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
