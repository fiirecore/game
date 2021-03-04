use serde::{Deserialize, Serialize};

use crate::util::Coordinate;
use crate::util::Direction;
use crate::util::Position;

use super::NPC;

#[derive(Clone, Deserialize, Serialize)]
pub struct NPCDestination {

    pub coords: Coordinate,
    pub direction: Direction,
    // pub start_direction: Direction,

}

impl NPCDestination {

    pub fn to(from: &Position, to: &Coordinate) -> Self {
        Self {
            coords: *to,
            direction: from.coords.towards(&to),
        }
    }

    pub fn next_to(from: &Position, to: &Coordinate) -> Self {
        let direction = from.coords.towards(to);
        // macroquad::prelude::debug!("Trainer direction: {:?}. \n Trying to go to {:?}", direction, to);
        let offset = direction.inverse().offset();
        let coords = Coordinate {
            x: to.x + offset.0 as isize,
            y: to.y + offset.1 as isize,
        };
        NPCDestination {
            coords,
            direction,
        }
    }

}

impl NPC {

    pub fn walk_to(&mut self, to: &Coordinate) {
        self.offset = Some(NPCDestination::to(&self.position, to));
    }

    pub fn walk_next_to(&mut self, to: &Coordinate) {
        self.offset = Some(NPCDestination::next_to(&self.position, to));
    }

    pub fn should_move(&self) -> bool {
        if let Some(offset) = self.offset.as_ref() {
            self.position.coords != offset.coords
        } else {
            false
        }
    }

    pub fn do_move(&mut self, delta: f32) {

        if let Some(offset) = self.offset.as_mut() {

            if self.position.coords.y == offset.coords.y {
                self.position.direction = if self.position.coords.x < offset.coords.x {
                    Direction::Right
                } else {
                    Direction::Left
                };
            } else if self.position.coords.x == offset.coords.x {
                self.position.direction = if self.position.coords.y < offset.coords.y {
                    Direction::Down
                } else {
                    Direction::Up
                };
            }

            let offsets = self.position.direction.offset();
            let offset = 60.0 * self.speed * delta;
            self.position.offset.x += offsets.0 * offset;
            self.position.offset.y += offsets.1 * offset;

            if self.position.offset.y * offsets.1 >= 16.0 {
                self.position.coords.y += offsets.1 as isize;
                self.position.offset.y = 0.0;
            }
            
            if self.position.offset.x * offsets.0 >= 16.0 {
                self.position.coords.x += offsets.0 as isize;
                self.position.offset.x = 0.0;
            }
            
        }
    
    }

}