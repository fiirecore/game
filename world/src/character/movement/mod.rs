use serde::{Deserialize, Serialize};
use util::positions::coordinate::CoordNum;

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