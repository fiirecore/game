use serde::{Deserialize, Serialize};

use crate::moves::persistent::PersistentMove;
use crate::pokemon::Health;
use crate::pokemon::status::StatusEffect;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MoveAction {

    Damage(DamageKind),
    Status(u8, StatusEffect), // u8 is 1 - 10, 1 = 10%, 10 = 100%
    Persist(Box<PersistentMove>, bool), // bool = do on current turn

}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DamageKind {
    PercentCurrent(f32),
    PercentMax(f32),
    Constant(Health),
}