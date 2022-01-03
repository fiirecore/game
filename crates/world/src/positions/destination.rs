use std::ops::AddAssign;

use serde::{Deserialize, Serialize};

use crate::positions::{Coordinate, Direction, Position};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Destination {
    pub coords: Coordinate,
    pub direction: Option<Direction>,
}

#[derive(Debug, Clone, Default)]
pub struct Path {
    pub queue: Vec<Direction>,
    pub turn: Option<Direction>,
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

impl Path {
    pub fn clear(&mut self) {
        self.queue.clear();
        self.turn = None;
    }

    pub fn extend(&mut self, position: &Position, destination: Destination) {
        let xlen = destination.coords.x - position.coords.x;
        let xdir = if xlen.is_negative() {
            Direction::Left
        } else {
            Direction::Right
        };

        self.queue.extend(vec![xdir; xlen.abs() as usize]);

        let ylen = destination.coords.y - position.coords.y;
        let ydir = if ylen.is_negative() {
            Direction::Up
        } else {
            Direction::Down
        };
        self.queue.extend(vec![ydir; ylen.abs() as usize]);
        self.turn = destination.direction;
    }
}

impl AddAssign for Path {
    fn add_assign(&mut self, rhs: Self) {
        self.queue.extend(rhs.queue);
        self.turn = rhs.turn;
    }
}

impl From<Position> for Destination {
    fn from(pos: Position) -> Self {
        Self {
            coords: pos.coords,
            direction: Some(pos.direction),
        }
    }
}
