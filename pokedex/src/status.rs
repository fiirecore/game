use deps::random::Random;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub enum Status {
    Paralysis,
    Sleep,
    Freeze,
    Burn, 
    Poison,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub enum StatusRange {
    Permanent,
    Temporary(u8, u8),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct StatusEffect {
    pub status: Status,
    pub range: StatusRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct StatusEffectInstance {
    pub status: Status,
    pub remaining: Option<u8>,
}

impl StatusRange {
    pub fn init(&self, status: Status, random: &Random) -> StatusEffectInstance {
        StatusEffectInstance {
            status,
            remaining: match self {
                StatusRange::Temporary(min, max) => Some(random.gen_range(*min, max + 1)),
                StatusRange::Permanent => None,
            }
        }
    }
}