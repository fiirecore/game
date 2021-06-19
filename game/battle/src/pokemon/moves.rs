use serde::{Deserialize, Serialize};

use pokedex::{
    types::Effective,
    pokemon::{Experience, stat::StatType},
    moves::target::MoveTargetInstance,
    item::ItemRef,
};

use super::ActivePokemonIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattleMove {
    Move(usize, Vec<MoveTargetInstance>),
    UseItem(ItemRef, MoveTargetInstance),
    Switch(usize),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BattleClientMove {
    Miss,
    TargetHP(f32),
    UserHP(f32), // dont heal the target
    Effective(Effective),
    StatStage(StatType, i8),
    Faint(ActivePokemonIndex), // target that is fainting
    #[deprecated(note = "only needs to be sent to one client")]
    GainExp(Experience),
    Fail,
}