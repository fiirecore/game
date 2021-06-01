use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Team {
	Player,
	Opponent,
}

impl Team {

    pub const fn other(&self) -> Self {
        match self {
            Self::Player => Self::Opponent,
            Self::Opponent => Self::Player,
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum MoveTarget {

	User,
	// Team,
	Opponent,
	Opponents,
	AllButUser,
	// All,
	// Singular(Team),
	// Team(Team),
	// TeamButSelf,
	// ReachAll(u8),
}

pub const fn move_target_player() -> MoveTarget {
	MoveTarget::User
}

pub const fn move_target_opponent() -> MoveTarget {
	MoveTarget::Opponent
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveTargetInstance {
	Opponent(usize),
	Team(usize),
	User,
	AllButUser, // to - do: remove in favor of vec<opponent, team or user>
	Opponents,
}