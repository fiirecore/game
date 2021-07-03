use serde::{Deserialize, Serialize};
use pokedex::{
    pokemon::{Level, Experience, stat::StatStage},
    item::ItemRef,
    moves::{target::MoveTargetInstance, MoveRef, Critical},
    battle::view::UnknownPokemon,
    types::Effective,
    battle::PokemonIndex,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BattleClientMove<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> {
    Miss,
    TargetHP(f32, Critical), // bool = crit
    UserHP(f32), // dont heal the target
    Effective(Effective),
    StatStage(StatStage),
    Faint(PokemonIndex<ID>), // target that is fainting
    SetExp(Experience, Level),
    Fail,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BattleClientAction<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> {
    Move(MoveRef, Vec<(MoveTargetInstance, Vec<BattleClientMove<ID>>)>),
    Switch(usize, Option<UnknownPokemon>),
    UseItem(ItemRef, MoveTargetInstance),
}