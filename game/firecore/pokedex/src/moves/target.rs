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

impl Default for MoveTarget {
    fn default() -> Self {
		Self::Opponent
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MoveTargetInstance {
	// Player,
	// Team(usize),
	Opponent(usize),
}