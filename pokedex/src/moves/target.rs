use serde::{Deserialize, Serialize};

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

impl MoveTarget {
    pub const fn player() -> MoveTarget {
        MoveTarget::User
    }

    pub const fn opponent() -> MoveTarget {
        MoveTarget::Opponent
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum MoveTargetInstance {
    Opponent(usize), // maybe add TrainerId
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
        (0..size).into_iter().map(Self::Opponent).collect()
    }

    pub fn all_but_user(user: usize, size: usize) -> Vec<Self> {
        let mut vec = Vec::with_capacity(size * 2 - 1);
        for i in 0..size {
            if i != user {
                vec.push(Self::Team(i));
            }
        }
        vec.extend((0..size).into_iter().map(|index| Self::Opponent(index)));
        vec
    }
}
