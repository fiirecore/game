use serde::{Deserialize, Serialize};

use crate::{
    map::movement::Elevation,
    positions::{Coordinate, Destination, Direction},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Position {
    pub coords: Coordinate,
    pub direction: Direction,
    #[deprecated]
    #[serde(default)]
    pub elevation: Elevation,
}

impl Position {
    pub fn from_destination(&mut self, destination: Destination) {
        self.coords = destination.coords;
        if let Some(direction) = destination.direction {
            self.direction = direction;
        }
    }

    pub fn in_direction(&self, direction: Direction) -> Self {
        Self {
            coords: self.coords.in_direction(direction),
            ..*self
        }
    }

    pub fn forwards(&self) -> Coordinate {
        self.coords.in_direction(self.direction)
    }

    pub fn next(&self) -> Position {
        self.in_direction(self.direction)
    }
}
