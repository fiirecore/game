use std::collections::BTreeMap;

use crate::{
    moves::{
        MoveRef,
        target::MoveTargetLocation,
    },
    pokemon::{
        Health,
        stat::StatStage,
    },
    status::StatusEffectInstance,
};

use super::DamageResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Damage(DamageResult<Health>), // bool = crit
    Status(StatusEffectInstance),
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

pub type MoveResults = BTreeMap<MoveTargetLocation, Vec<MoveResult>>;

pub struct TurnResult {
    pub pokemon_move: MoveRef,
    pub results: MoveResults,
}

