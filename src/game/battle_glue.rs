use std::fmt::Debug;

use crate::pokedex::pokemon::{owned::SavedPokemon, party::Party};
use worldlib::character::npc::{trainer::TransitionId, BadgeId};

pub use worldlib::TrainerId;

/***********************/

pub type BattleEntryRef<'a> = &'a mut Option<BattleEntry>;

pub struct BattleEntry {
    pub id: BattleId,
    pub name: Option<String>,
    pub party: Party<SavedPokemon>,
    pub trainer: Option<BattleTrainerEntry>,
    pub active: usize,
}

pub struct BattleTrainerEntry {
    pub transition: TransitionId,
    pub gym_badge: Option<BadgeId>,
    pub victory_message: Vec<Vec<String>>,
    pub worth: u16,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BattleId(pub Option<TrainerId>);

impl core::fmt::Display for BattleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}