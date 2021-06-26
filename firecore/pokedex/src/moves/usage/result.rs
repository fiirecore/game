use std::collections::BTreeMap;

use crate::{
    types::Effective,
    moves::{
        MoveRef,
        target::MoveTargetInstance,
    },
    pokemon::{
        Health,
        stat::{StatType, Stage},
        status::StatusEffect,
    }
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Damage(Health, Effective, bool), // bool = crit
    Status(StatusEffect),
    Drain(Health, Health, Effective, bool), // damage, healing, effective, crit
    StatStage(StatType, Stage),
    // NextHit(), next hit protect, next hit endure
    Todo,
}

pub type MoveResults = BTreeMap<MoveTargetInstance, Option<MoveResult>>;

pub struct TurnResult {
    pub pokemon_move: MoveRef,
    pub results: MoveResults,
}

