use pokedex::pokemon::{party::Party, owned::SavedPokemon};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use tinystr::TinyStr8;

use super::NpcId;
use crate::default_true;

type MessageSet = Vec<Vec<String>>;
pub type TransitionId = TinyStr8;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Trainer {
    #[serde(default = "default_true")]
    pub battle_on_interact: bool,
    pub tracking: Option<u8>,
    pub encounter_message: MessageSet,

    #[serde(default = "default_battle_transition")]
    pub battle_transition: TransitionId,

    pub party: Party<SavedPokemon>,

    #[serde(default)]
    pub victory_message: MessageSet,
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

fn default_battle_transition() -> TransitionId {
    "default".parse().unwrap()
}
