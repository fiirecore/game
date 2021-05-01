use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
};

use serde::{Serialize, Deserialize};

use crate::Direction;
use crate::Position;

pub type CoordNum = i32;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Coordinate {

	pub x: CoordNum,
	pub y: CoordNum,

}

impl Coordinate {

    pub fn new(x: CoordNum, y: CoordNum) -> Self {
        Self {
            x,
            y,
        }
    }

	pub const fn towards(&self, destination: Coordinate) -> Direction {
		if (self.x - destination.x).abs() > (self.y - destination.y).abs() {
			if self.x > destination.x {
				Direction::Left
			} else {
				Direction::Right
			}
		} else {
			if self.y > destination.y {
				Direction::Up
			} else {
				Direction::Down
			}
		}
	}

    pub fn in_direction(self, direction: Direction) -> Self {
        self + direction.tile_offset()
    }

    pub fn position(self, direction: Direction) -> Position {
        Position {
            coords: self,
            direction,
            ..Default::default()
        }
    }

}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl core::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}