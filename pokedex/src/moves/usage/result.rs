use std::collections::BTreeMap;

use crate::{
    moves::{
        MoveRef,
        target::MoveTargetInstance,
    },
    pokemon::{
        Health,
        stat::StatStage,
        status::StatusEffect,
    }
};

use super::DamageResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Damage(DamageResult<Health>), // bool = crit
    Status(StatusEffect),
    Drain(DamageResult<Health>, Health), // damage, healing, effective, crit
    StatStage(StatStage),
    Flinch,
    // NextHit(), next hit protect, next hit endure
    NoHit(NoHitResult),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoHitResult {
    Ineffective,
    Miss,
    Todo,
}

pub type MoveResults = BTreeMap<MoveTargetInstance, Vec<MoveResult>>;

pub struct TurnResult {
    pub pokemon_move: MoveRef,
    pub results: MoveResults,
}

