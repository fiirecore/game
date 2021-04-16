use firecore_util::Coordinate;
use firecore_util::Direction;
use firecore_util::Position;

use crate::character::Character;

use super::NPC;

impl NPC {

    pub fn find_character(&mut self, coords: Coordinate, character: &mut impl Character) -> bool {
        if self.eye_track(&coords) {
            self.go_next_to(coords);
            character.freeze();
            true
        } else {
            false
        }
    }

    pub fn eye_track(&self, coords: &Coordinate) -> bool {
        if let Some(trainer) = self.trainer.as_ref() {
            if let Some(tracker) = trainer.tracking_length {
                let tracker = tracker as isize;
                match self.position.direction {
                    firecore_util::Direction::Up => if self.position.coords.x == coords.x {
                        if self.position.coords.y > coords.y && self.position.coords.y - tracker <= coords.y {
                            return true;
                        }
                    }
                    firecore_util::Direction::Down => if self.position.coords.x == coords.x {
                        if self.position.coords.y < coords.y && self.position.coords.y + tracker >= coords.y {
                            return true;
                        }
                    }
                    firecore_util::Direction::Left => if self.position.coords.y == coords.y {
                        if self.position.coords.x > coords.x && self.position.coords.x - tracker <= coords.x {
                            return true;
                        }
                    }
                    firecore_util::Direction::Right => if self.position.coords.y == coords.y {
                        if self.position.coords.x < coords.x && self.position.coords.x + tracker >= coords.x {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn interact_from(&mut self, position: &Position) -> bool {
        self.can_interact_from(position).map(|dir| {
            self.position.direction = dir;
            true
        }).unwrap_or_default()
    }

    pub fn can_interact_from(&self, position: &Position) -> Option<Direction> {
        if position.coords.x == self.position.coords.x {
            match position.direction {
                Direction::Up => {
                    if position.coords.y - 1 == self.position.coords.y {
                        Some(Direction::Down)
                    } else {
                        None
                    }
                },
                Direction::Down => {
                    if position.coords.y + 1 == self.position.coords.y {
                        Some(Direction::Up)
                    } else {
                        None
                    }
                },
                _ => None,
            }
        } else if position.coords.y == self.position.coords.y {
            match position.direction {
                Direction::Right => {
                    if position.coords.x + 1 == self.position.coords.x {
                        Some(Direction::Left)
                    } else {
                        None
                    }
                },
                Direction::Left => {
                    if position.coords.x - 1 == self.position.coords.x {
                        Some(Direction::Right)
                    } else {
                        None
                    }
                },
                _ => None,
            }
        } else {
            None
        }
    }

}