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
}

impl MoveTargetInstance {

    pub fn user() -> Vec<Self> {
		vec![Self::User]
    }

    pub fn opponent(index: usize) -> Vec<Self> {
		vec![Self::Opponent(index)]
    }

    pub fn team(index: usize) -> Vec<Self> {
        vec![Self::Team(index)]
    }

    pub fn opponents(size: usize) -> Vec<Self> {
        (0..size).into_iter().map(|active| Self::Opponent(active)).collect()
    }

    pub fn all_but_user(user: usize, size: usize) -> Vec<Self> {
        let mut vec = Vec::with_capacity(size * 2 - 1);
		for i in 0..size {
			if i != user {
				vec.push(Self::Team(i));
			}
		}
		(0..size).into_iter().for_each(|index| vec.push(Self::Opponent(index)));
        vec
    }

}