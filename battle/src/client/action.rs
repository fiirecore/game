use core::fmt::{Debug, Display};
use serde::{Deserialize, Serialize};
use pokedex::{
    pokemon::{Level, Experience, stat::StatStage},
    item::ItemRef,
    moves::{target::MoveTargetLocation, MoveRef, usage::Critical},
    types::Effective,
    battle::PokemonIndex,
    status::StatusEffectInstance,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BattleClientMove<ID: Sized + Copy + Debug + Display + PartialEq> {
    Miss,
    TargetHP(f32, Critical), // bool = crit
    UserHP(f32), // dont heal the target
    Effective(Effective),
    StatStage(StatStage),
    Status(StatusEffectInstance),
    Faint(PokemonIndex<ID>), // target that is fainting
    SetExp(Experience, Level),
    Fail,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BattleClientAction<ID: Sized + Copy + Debug + Display + PartialEq> {
    Move(MoveRef, Vec<(MoveTargetLocation, Vec<BattleClientMove<ID>>)>),
    Switch(usize),
    UseItem(ItemRef, MoveTargetLocation),
}