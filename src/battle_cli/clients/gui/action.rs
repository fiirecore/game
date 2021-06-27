use battle::pokemon::{BattleClientAction, BattleClientMove};
use pokedex::{
    moves::{target::MoveTargetInstance, MoveRef},
    pokemon::{Experience, Level},
    battle::ActionInstance,
};

#[derive(Debug, Clone)]
pub enum BattleClientGuiAction<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> {
    Action(BattleClientAction<ID>),
    Faint,
    Catch,
    GainExp(Level, Experience),
    LevelUp(Level, Option<Vec<MoveRef>>),
    Replace(Option<usize>),
}

#[derive(Debug)]
pub enum BattleClientGuiCurrent<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> {
    Move(Vec<(MoveTargetInstance, Vec<BattleClientMove<ID>>)>),
    Switch(usize),
    UseItem(MoveTargetInstance),
    Faint,
    Catch,
    Replace(bool),
    GainExp,
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> BattleClientGuiAction<ID> {
    pub fn requires_user(&self) -> bool {
        matches!(self, Self::Faint)
    }
}
