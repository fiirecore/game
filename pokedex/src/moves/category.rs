use crate::pokemon::stat::StatType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum MoveCategory {
    Status,
    Physical,
    Special,
}

impl MoveCategory {
    pub fn stats(&self) -> (StatType, StatType) {
        (self.attack(), self.defense())
    }
    pub fn attack(&self) -> StatType {
        match self {
            MoveCategory::Physical => StatType::Attack,
            MoveCategory::Special => StatType::SpAttack,
            MoveCategory::Status => unreachable!("Cannot get attack stat for status move!"),
        }
    }
    pub fn defense(&self) -> StatType {
        match self {
            MoveCategory::Physical => StatType::Defense,
            MoveCategory::Special => StatType::SpDefense,
            MoveCategory::Status => unreachable!("Cannot get defense stat for status move!"),
        }
    }
}
