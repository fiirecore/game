use firecore_battle::pokemon::{ActionInstance, BattleClientAction, BattleClientMove};
use pokedex::{
    moves::{target::MoveTargetInstance, MoveRef},
    pokemon::{Experience, Level},
};

#[derive(Debug, Clone)]
pub enum BattleClientGuiAction {
    Action(BattleClientAction),
    Faint,
    Catch,
    GainExp(Level, Experience),
    LevelUp(Level, Option<Vec<MoveRef>>),
    Replace(Option<usize>),
}

#[derive(Debug)]
pub enum BattleClientGuiCurrent {
    Move(Vec<(MoveTargetInstance, Vec<BattleClientMove>)>),
    Switch(usize),
    UseItem(MoveTargetInstance),
    Faint,
    Catch,
    Replace(bool),
}

impl BattleClientGuiAction {
    pub fn requires_user(&self) -> bool {
        matches!(self, Self::Faint)
    }
}

pub type BattleClientGuiActionInstance = ActionInstance<BattleClientGuiAction>;
pub type BattleClientGuiCurrentInstance = ActionInstance<BattleClientGuiCurrent>;
