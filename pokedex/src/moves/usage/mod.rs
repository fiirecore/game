use serde::{Deserialize, Serialize};

use crate::{
    moves::target::MoveTargetInstance,
    pokemon::{
        instance::PokemonInstance,
        stat::{Stage, StatType},
        status::StatusEffect,
    },
};

mod damage;
pub use damage::*;

mod result;
pub use result::*;

pub mod script;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MoveUseType {
    Damage(DamageKind),
    Status(u8, StatusEffect), // u8 is 1 - 10, 1 = 10%, 10 = 100%
    Drain(DamageKind, f32),
    StatStage(StatType, Stage),
    Flinch,
    Script(String),
    Chance(Vec<Self>, f32),
    User(Vec<Self>),
    Todo,
    // Linger(u8, DamageKind),
    // TriggerLate(u8, Box<MoveUseType>),
}

#[derive(Clone, Copy)]
pub struct PokemonTarget<'a> {
    pub pokemon: &'a PokemonInstance,
    pub active: MoveTargetInstance,
}
