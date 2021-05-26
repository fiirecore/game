use std::collections::BTreeMap;

use crate::{
    types::Effective,
    moves::{
        Move,
        target::MoveTargetInstance,
    },
    pokemon::{
        Health,
        instance::PokemonInstance,
        stat::{StatType, Stage},
        status::StatusEffect,
    }
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Damage(Health, Effective),
    Status(StatusEffect),
    Drain(Health, Health, Effective), // damage, healing, effective
    StatStage(StatType, Stage),
    // NextHit(), next hit protect, next hit endure
    Todo,
}

pub struct TurnResult {
    pub pokemon_move: &'static Move,
    pub results: BTreeMap<MoveTargetInstance, Option<MoveResult>>,
}

#[derive(Clone, Copy)]
pub struct PokemonTarget<'a> {
    pub instance: MoveTargetInstance,
    pub pokemon: &'a PokemonInstance,
}