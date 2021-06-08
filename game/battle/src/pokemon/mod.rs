
mod moves;
mod party;

pub mod view;

pub use moves::*;
pub use party::*;

#[derive(Debug, Clone)]
pub enum ActivePokemon {
    None,
    Some(usize, Option<BattleMove>),
    ToReplace,
}

impl Default for ActivePokemon {
    fn default() -> Self {
        Self::None
    }
}

impl ActivePokemon {

    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }

    pub fn replace(&mut self) {
        *self = Self::ToReplace;
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Some(..))
    }

    pub fn index(&self) -> Option<usize> {
        match self {
            Self::Some(index, ..) => Some(*index),
            _ => None,
        }
    }

    pub fn use_move(&mut self) -> Option<BattleMove> {
        match self {
            Self::Some(_, queued) => queued.take(),
            _ => None,
        }
    }

}