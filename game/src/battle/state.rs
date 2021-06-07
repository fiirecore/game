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
	StartWait,
	Selecting(bool), // started, player done, opponent done
	// Waiting (for opponent)
	Moving(MoveState),
	End,
}

impl BattleState {
	pub const SELECTING_START: Self = Self::Selecting(false);
	pub const MOVE_START: Self = Self::Moving(MoveState::Start);
}

impl Default for BattleState {
    fn default() -> Self {
        Self::StartWait
    }
}

#[derive(Debug)]
pub enum MoveState {

	Start,
	SetupPokemon,
	Pokemon(Vec<BattleActionInstance>), // queue of pokemon
	SetupPost,
	Post,
	End,

}