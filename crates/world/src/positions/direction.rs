use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::positions::{Coordinate, PixelOffset};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Enum, Deserialize, Serialize)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

impl Direction {
    pub const fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub const fn horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
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

    pub fn pixel_offset(&self, increment: f32) -> PixelOffset {
        match self {
            Direction::Up => PixelOffset {
                x: 0.0,
                y: -increment,
            },
            Direction::Down => PixelOffset {
                x: 0.0,
                y: increment,
            },
            Direction::Left => PixelOffset {
                x: -increment,
                y: 0.0,
            },
            Direction::Right => PixelOffset {
                x: increment,
                y: 0.0,
            },
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}
