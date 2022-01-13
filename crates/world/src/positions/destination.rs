use serde::{Deserialize, Serialize};

use crate::positions::{Coordinate, Direction, Position};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Destination {
    pub coords: Coordinate,
    pub direction: Option<Direction>,
}

impl Destination {
    pub fn to(from: &Position, to: Coordinate) -> Self {
        Self {
            coords: to,
            direction: Some(from.coords.towards(to)),
        }
    }

    pub fn next_to(from: &Position, to: Coordinate) -> Self {
        let direction = from.coords.towards(to);
        Destination {
            coords: to + direction.inverse().tile_offset(),
            direction: Some(direction),
        }
    }
}
