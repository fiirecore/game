use crate::{moves::Power, pokemon::Health, types::Effective};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DamageKind {
    Power(Power),
    PercentCurrent(f32),
    PercentMax(f32),
    Constant(Health),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageResult<INT> {
    pub damage: INT,
    pub effective: Effective,
    pub crit: bool,
}