use crate::pokedex::{
    item::bag::SavedBag,
    pokemon::{owned::SavedPokemon, party::Party},
};

use worldlib::{
    character::{
        npc::{group::NpcGroupId, trainer::BadgeId, NpcId},
        trainer::Worth,
    },
    map::TransitionId,
};

use crate::engine::text::MessagePage;

// pub const DEFAULT_RANDOM_BATTLE_SIZE: usize = 2;

// pub fn random_wild_battle(random: &mut impl Rng, pokedex: u16, size: usize) -> BattleEntry {
//     let mut party = Party::new();
//     for _ in 0..size {
//         let id = random.gen_range(0..pokedex) + 1;
//         let ivs = StatSet::random_iv(random);
//         let level = random.gen_range(1..=100);
//         party.push(SavedPokemon::generate(random, id, level, None, Some(ivs)));
//     }
//     BattleEntry {
//         id: BattleId::Wild,
//         party,
//         trainer: None,
//         active: 1,
//     }
// }

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
