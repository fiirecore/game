use std::ops::AddAssign;

use serde::{Deserialize, Serialize};

use crate::positions::{Destination, Direction, Position};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Actions {
    pub queue: Vec<ActionQueue>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActionQueue {
    Move(Direction),
    Look(Direction),
    Interact,
}

impl Actions {
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn extend(&mut self, position: &Position, destination: Destination) {
        let xlen = destination.coords.x - position.coords.x;
        let xdir = if xlen.is_negative() {
            Direction::Left
        } else {
            Direction::Right
        };

        self.queue
            .extend(vec![ActionQueue::Move(xdir); xlen.abs() as usize]);

        let ylen = destination.coords.y - position.coords.y;
        let ydir = if ylen.is_negative() {
            Direction::Up
        } else {
            Direction::Down
        };
        self.queue
            .extend(vec![ActionQueue::Move(ydir); ylen.abs() as usize]);
        if let Some(direction) = destination.direction {
            self.queue.push(ActionQueue::Look(direction));
        }
    }
}

impl AddAssign for Actions {
    fn add_assign(&mut self, rhs: Self) {
        self.queue.extend(rhs.queue);
    }
}
