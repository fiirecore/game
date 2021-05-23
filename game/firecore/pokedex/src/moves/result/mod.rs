use serde::{Deserialize, Serialize};

use crate::pokemon::status::StatusEffect;

mod damage;
pub use damage::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MoveUseType {

    Damage(DamageKind),
    Status(u8, StatusEffect), // u8 is 1 - 10, 1 = 10%, 10 = 100%
    Drain(DamageKind, f32),
    // Linger(u8, DamageKind),
    // TriggerLate(u8, Box<MoveUseType>),
    Todo,
    Script(String),

}