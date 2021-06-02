use std::collections::VecDeque;

use pokedex::moves::target::Team;

use crate::battle::pokemon::BattleActionInstance;

#[derive(Debug, PartialEq)]
pub enum BattleManagerState {
	Begin,
	Transition,
	Opener,
	Introduction,
	Battle,
	Closer,
}

impl Default for BattleManagerState {
    fn default() -> Self {
        Self::Begin
    }
}

#[derive(PartialEq)]
pub enum TransitionState {
	Begin, // runs on spawn methods
	Run,
	End, // spawns next state and goes back to beginning
}

impl Default for TransitionState {
    fn default() -> Self {
        Self::Begin
    }
}

#[derive(Debug)]
pub enum BattleState {
	Begin,
	Selecting(bool, bool, bool), // started, player done, opponent done
	// Waiting (for opponent)
	Moving(MoveState),
	End,
}

impl BattleState {
	pub const SELECTING_START: Self = Self::Selecting(false, false, false);
	pub const MOVE_START: Self = Self::Moving(MoveState::Start);
}

impl Default for BattleState {
    fn default() -> Self {
        Self::Begin
    }
}

#[derive(Debug)]
pub enum MoveState {

	Start,
	SetupPokemon,
	Pokemon(MoveQueue), // queue of pokemon
	SetupPost,
	Post,
	End,

}

#[derive(Debug)]
pub struct MoveQueue {
	pub actions: VecDeque<BattleActionInstance>,
	pub current: Option<BattleActionInstance>,
}

impl MoveQueue {
	pub fn new(actions: VecDeque<BattleActionInstance>) -> Self {
		Self {
			actions,
			current: None,
		}
	}
}