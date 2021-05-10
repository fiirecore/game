use serde::{Deserialize, Serialize};

use crate::moves::{Power, MoveCategory};
use crate::moves::persistent::PersistentMove;
use crate::pokemon::Health;
use crate::pokemon::status::StatusEffect;
use crate::pokemon::types::PokemonType;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum MoveAction {

    Action(MoveActionType),
    Persistent(PersistentMove),

}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum MoveActionType {

    Damage(DamageKind),
    Status(u8, StatusEffect), // u8 is 1 - 10, 1 = 10%, 10 = 100%
    Drain(DamageKind, f32),

}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DamageKind {
    Move(Power, MoveCategory, PokemonType),
    PercentCurrent(f32),
    PercentMax(f32),
    Constant(Health),
}