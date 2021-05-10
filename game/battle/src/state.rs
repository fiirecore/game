use std::collections::VecDeque;

use crate::pokemon::{
    ActivePokemonIndex,
    BattleAction,
};



#[derive(Debug)]
pub enum BattleState {
	Selecting { index: usize, started: bool },
	// Waiting (for opponent)
	Moving(MoveState),
}

impl Default for BattleState {
    fn default() -> Self {
        Self::Selecting { index: crate::Battle::DEFAULT_ACTIVE, started: false }
    }
}

#[derive(Debug, Clone)]
pub enum MoveState {

	Start,
	Pokemon { queue: VecDeque<ActivePokemonIndex>, current: Option<(BattleAction, ActivePokemonIndex, bool)> }, // queue of pokemon
	Post,

}