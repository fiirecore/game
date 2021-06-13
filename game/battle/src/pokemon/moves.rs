use pokedex::{
    types::Effective,
    pokemon::stat::StatType,
    moves::target::MoveTargetInstance,
    item::ItemRef,
};

use super::ActivePokemonIndex;

#[derive(Debug, Clone)]
pub enum BattleMove {
    Move(usize, Vec<MoveTargetInstance>),
    UseItem(ItemRef, MoveTargetInstance),
    Switch(usize),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BattleClientMove {
    Miss,
    TargetHP(f32),
    UserHP(f32), // dont heal the target
    Effective(Effective),
    StatStage(StatType, i8),
    Faint(ActivePokemonIndex), // target that is fainting
    Fail,
}