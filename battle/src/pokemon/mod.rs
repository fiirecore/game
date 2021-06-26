use serde::{Deserialize, Serialize};
use crate::pokedex::{
    pokemon::{Experience, stat::{StatType, Stage}},
    item::ItemRef,
    moves::{target::MoveTargetInstance, MoveRef},
    battle::{view::UnknownPokemon, ActionInstance},
    types::Effective,
    battle::PokemonIndex,
};

mod party;

pub use party::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BattleClientMove<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> {
    Miss,
    TargetHP(f32),
    UserHP(f32), // dont heal the target
    Effective(Effective),
    Critical,
    StatStage(StatType, Stage),
    Faint(PokemonIndex<ID>), // target that is fainting
    /* #[deprecated(note = "only needs to be sent to one client")] */ GainExp(Experience),
    Fail,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BattleClientAction<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> {
    Move(MoveRef, Vec<(MoveTargetInstance, Vec<BattleClientMove<ID>>)>),
    Switch(usize, Option<UnknownPokemon>),
    UseItem(ItemRef, MoveTargetInstance),
}

pub type BattleClientActionInstance<ID> = ActionInstance<ID, BattleClientAction<ID>>;