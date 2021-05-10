use deps::hash::HashSet;
use firecore_pokedex::pokemon::saved::SavedPokemonParty;
use serde::{Deserialize, Serialize};
use firecore_util::battle::BattleScreenTransitions;

use crate::default_true;
use super::NPCId;

type MessageSet = Vec<Vec<String>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Trainer {

    #[serde(default = "default_true")]
    pub battle_on_interact: bool,
    pub tracking: Option<u8>,
    pub encounter_message: MessageSet,

    #[serde(default)]
    pub battle_transition: BattleScreenTransitions,

    pub party: SavedPokemonParty,

    #[serde(default)]
    pub victory_message: MessageSet,
    #[serde(default)]
    pub disable: HashSet<NPCId>,
    pub worth: u16,

}