use crate::{moves::Power, pokemon::Health, types::Effective};
use serde::{Deserialize, Serialize};

use super::Percent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DamageKind {
    Power(Power),
    PercentCurrent(Percent),
    PercentMax(Percent),
    Constant(Health),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageResult<INT> {
    pub damage: INT,
    pub effective: Effective,
    pub crit: bool,
}