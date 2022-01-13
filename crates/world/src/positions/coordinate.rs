use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    map::movement::Elevation,
    positions::{Direction, Position},
};

pub type CoordinateInt = i32;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: CoordinateInt,
    pub y: CoordinateInt,
}

pub struct Coordinate3d {
    pub xy: Coordinate,
    pub elevation: Elevation,
}

impl Coordinate {
    // pub type Integer = i32;
    pub const ZERO: Coordinate = Coordinate { x: 0, y: 0 };

    pub fn new(x: CoordinateInt, y: CoordinateInt) -> Self {
        Self { x, y }
    }

    pub const fn towards(&self, destination: Coordinate) -> Direction {
        if (self.x - destination.x).abs() > (self.y - destination.y).abs() {
            if self.x > destination.x {
                Direction::Left
            } else {
                Direction::Right
            }
        } else if self.y > destination.y {
            Direction::Up
        } else {
            Direction::Down
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

    pub fn equal(&self, x: &CoordinateInt, y: &CoordinateInt) -> bool {
        self.x.eq(x) && self.y.eq(y)
    }
}

impl From<(CoordinateInt, CoordinateInt)> for Coordinate {
    fn from(c: (CoordinateInt, CoordinateInt)) -> Self {
        Self { x: c.0, y: c.1 }
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

impl<'de> serde::Deserialize<'de> for Coordinate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        type C = (CoordinateInt, CoordinateInt);
        C::deserialize(deserializer).map(Into::into)
    }
}

impl serde::Serialize for Coordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        (self.x, self.y).serialize(serializer)
    }
}