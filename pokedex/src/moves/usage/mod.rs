use serde::{Deserialize, Serialize};

use crate::{
    moves::target::MoveTargetLocation,
    pokemon::{
        instance::PokemonInstance,
        stat::{BattleStatType, Stage},
    },
    status::{Status, StatusRange},
};

mod damage;
pub use damage::*;

mod result;
pub use result::*;

pub mod script;

pub type Critical = bool;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MoveUseType {
    Damage(DamageKind),
    Status(Status, StatusRange, f32),
    // Ailment(Ailment, f32),
    Drain(DamageKind, f32),
    StatStage(BattleStatType, Stage),
    Flinch,
    Script(String),
    Chance(Vec<Self>, f32),
    User(Vec<Self>),
    Todo,
}

#[derive(Clone, Copy)]
pub struct PokemonTarget<'a> {
    pub pokemon: &'a PokemonInstance,
    pub active: MoveTargetLocation,
}