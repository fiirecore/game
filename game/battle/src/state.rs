use crate::pokemon::BattleActionInstance;

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