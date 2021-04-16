use serde::{Deserialize, Serialize};
use firecore_util::hash::HashSet;
use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_util::battle::BattleScreenTransitions;

use crate::default_true;
use super::NPCId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trainer {

    #[serde(default = "default_true")]
    pub battle_on_interact: bool,
    pub tracking_length: Option<u8>,
    pub encounter_message: Vec<Vec<String>>,

    #[serde(default)]
    pub battle_transition: BattleScreenTransitions,

    pub party: PokemonParty,

    #[serde(default)]
    pub victory_message: Vec<Vec<String>>,
    #[serde(default)]
    pub disable_others: HashSet<NPCId>,
    pub worth: u16,

}