use serde::{Deserialize, Serialize};

use crate::Coordinate;
use crate::PixelOffset;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Direction {
	
	Down,
	Up,
	Left,
	Right,
	
}

impl Direction {

	pub const DIRECTIONS: [Direction; 4] = [
        Direction::Down,
        Direction::Up,
        Direction::Left,
        Direction::Right,
    ];

	pub const fn inverse(&self) -> Direction {
		match self {
		    Direction::Up => Direction::Down,
		    Direction::Down => Direction::Up,
		    Direction::Left => Direction::Right,
		    Direction::Right => Direction::Left,
		}
	}

	pub const fn horizontal(&self) -> bool {
		match self {
			Self::Left | Self::Right => true,
			_ => false,
		}
	}

	#[inline]
	pub const fn vertical(&self) -> bool {
		!self.horizontal()
	}

	// Input

	pub const fn tile_offset(&self) -> Coordinate {
		match self {
		    Direction::Up => Coordinate { x: 0, y: -1 },
		    Direction::Down => Coordinate { x: 0, y: 1 },
		    Direction::Left => Coordinate { x: -1, y: 0 },
		    Direction::Right => Coordinate { x: 1, y: 0 },
		}
	}

	pub const fn pixel_offset(&self) -> PixelOffset {
		match self {
		    Direction::Up => PixelOffset { x: 0.0, y: -1.0 },
		    Direction::Down => PixelOffset { x: 0.0, y: 1.0 },
		    Direction::Left => PixelOffset { x: -1.0, y: 0.0 },
		    Direction::Right => PixelOffset { x: 1.0, y: 0.0 },
		}
	}

}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}