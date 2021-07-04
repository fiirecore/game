use battle::client::action::{BattleClientAction, BattleClientMove};
use game::pokedex::{
    moves::{target::MoveTargetLocation, MoveRef},
    pokemon::{Experience, Level},
};

#[derive(Debug, Clone)]
pub enum BattleClientGuiAction<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> {
    Action(BattleClientAction<ID>),
    Faint,
    Catch,
    SetExp(Level, Experience, Vec<MoveRef>),
    LevelUp(Vec<MoveRef>),
    Replace(Option<usize>),
}

#[derive(Debug)]
pub enum BattleClientGuiCurrent<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> {
    Move(Vec<(MoveTargetLocation, Vec<BattleClientMove<ID>>)>),
    Switch(usize),
    UseItem(MoveTargetLocation),
    Faint,
    Catch,
    Replace(bool),
    SetExp,
    LevelUp,
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> BattleClientGuiAction<ID> {
    pub fn requires_user(&self) -> bool {
        matches!(self, Self::Faint)
    }
}
