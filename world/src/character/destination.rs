use std::collections::VecDeque;

use crate::positions::{Coordinate, Direction, Destination};

#[derive(Debug, Clone)]
pub struct DestinationPath {
    pub started: bool,
    pub queued_movements: VecDeque<Direction>,
    pub final_direction: Option<Direction>,
}

impl DestinationPath {
    pub fn new_path(origin: Coordinate, destination: Destination) -> Self {
        // if origin == destination.coords {
        //     return None;
        // }
        let xlen = destination.coords.x - origin.x;
        let xdir = if xlen.is_negative() {
            Direction::Left
        } else {
            Direction::Right
        };
        let mut vec = vec![xdir; xlen.abs() as usize];

        let ylen = destination.coords.y - origin.y;
        let ydir = if ylen.is_negative() {
            Direction::Up
        } else {
            Direction::Down
        };
        vec.append(&mut vec![ydir; ylen.abs() as usize]);
        let vec = VecDeque::from(vec);
        Self {
            started: false,
            queued_movements: vec,
            final_direction: destination.direction,
        }
    }
}
