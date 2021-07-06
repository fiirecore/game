use serde::{Deserialize, Serialize};
use crate::positions::CoordNum;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum MovementType {

    Still,
    LookAround,
    WalkUpAndDown(CoordNum),

}

impl Default for MovementType {
    fn default() -> Self {
        Self::Still
    }
}