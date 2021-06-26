use serde::{Deserialize, Serialize};

use crate::pokemon::{stat::{StatType, Stage}, status::StatusEffect};

mod damage;
pub use damage::*;

mod result;
pub use result::*;

pub mod pokemon;

pub mod script;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MoveUseType {

    Damage(DamageKind),
    Status(u8, StatusEffect), // u8 is 1 - 10, 1 = 10%, 10 = 100%
    Drain(DamageKind, f32),
    StatStage(StatType, Stage),
    // Linger(u8, DamageKind),
    // TriggerLate(u8, Box<MoveUseType>),
    Todo,
    Script(String),

}