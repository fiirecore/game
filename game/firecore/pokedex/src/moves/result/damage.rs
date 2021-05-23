use serde::{Deserialize, Serialize};
use crate::{
    pokemon::Health,
    moves::Power,
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DamageKind {
    Power(Power),
    PercentCurrent(f32),
    PercentMax(f32),
    Constant(Health),
}