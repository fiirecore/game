use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::moves::Move;
use crate::moves::target::MoveTargetInstance;
use crate::pokemon::Health;
use crate::pokemon::status::StatusEffect;

use super::PokemonInstance;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveResult {
    Damage(Health),
    Status(StatusEffect),
    Drain(Health, Health),
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