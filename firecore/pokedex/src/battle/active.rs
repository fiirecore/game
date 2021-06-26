use super::BattleMove;

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
    pub fn new(index: usize, some: bool) -> Self {
        match some {
            true => Self::Some(index, None),
            false => Self::None,
        }
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