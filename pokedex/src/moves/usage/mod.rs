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
pub type Percent = u8; // 0 to 100

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum MoveUseType {
    Damage(DamageKind),
    Status(Status, StatusRange, Percent),
    // Ailment(Ailment, f32),
    Drain(DamageKind, Percent),
    StatStage(BattleStatType, Stage),
    Flinch,
    Script(String),
    Chance(Vec<Self>, Percent),
    User(Vec<Self>),
    Todo,
}

#[derive(Clone, Copy)]
pub struct PokemonTarget<'a> {
    pub pokemon: &'a PokemonInstance,
    pub active: MoveTargetLocation,
}
