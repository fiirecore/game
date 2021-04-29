use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BattleType { // move somewhere else

    Wild,
    Trainer,
    GymLeader,

}

impl Default for BattleType {
    fn default() -> Self {
        Self::Wild
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum BattleScreenTransitions {

    Flash,
    Trainer,

}

impl Default for BattleScreenTransitions {
    fn default() -> Self {
        Self::Flash
    }
}