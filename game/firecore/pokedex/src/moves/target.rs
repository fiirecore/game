use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum MoveTarget {
	// All,
	// AllButSelf,
	// OnlySelf,
	// Players,
	// Opponents,
	Player,
	Opponent,
	// Random(Box<Self>),
}

pub const fn move_target_player() -> MoveTarget {
	MoveTarget::Player
}

pub const fn move_target_opponent() -> MoveTarget {
	MoveTarget::Opponent
}

#[derive(Debug, Clone, Copy)]
pub enum MoveTargetInstance {
	User,
	// Team(usize),
	Opponent(usize),
}