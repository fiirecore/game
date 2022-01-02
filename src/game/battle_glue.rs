use std::fmt::Debug;

use crate::pokedex::{
    item::bag::SavedBag,
    pokemon::{owned::SavedPokemon, party::Party},
};

use firecore_battle_gui::pokedex::engine::text::MessagePage;
use worldlib::{
    character::{
        npc::{group::NpcGroupId, trainer::BadgeId, NpcId},
        trainer::Worth,
    },
    map::TransitionId,
};

/***********************/

#[derive(Debug, Clone)]
pub struct BattleEntry {
    pub id: BattleId,
    pub party: Party<SavedPokemon>,
    pub trainer: Option<BattleTrainerEntry>,
    pub active: usize,
}

#[derive(Debug, Clone)]
pub struct BattleTrainerEntry {
    pub name: String,
    pub bag: SavedBag,
    pub badge: Option<BadgeId>,
    pub sprite: NpcGroupId,
    pub transition: TransitionId,
    pub defeat: Vec<MessagePage>,
    pub worth: Worth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BattleId {
    Default,
    Player,
    Wild,
    Trainer(NpcId),
}

impl Default for BattleId {
    fn default() -> Self {
        Self::Default
    }
}

impl core::fmt::Display for BattleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}
