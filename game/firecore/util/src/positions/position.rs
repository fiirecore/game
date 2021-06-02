use serde::{Deserialize, Serialize};

use crate::Coordinate;
use crate::Destination;
use crate::Direction;
use crate::PixelOffset;

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct Position {

	pub coords: Coordinate,
	pub direction: Direction,
    #[serde(skip)]
    pub offset: PixelOffset,

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

}