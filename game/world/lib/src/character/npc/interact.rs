use firecore_util::Coordinate;
use firecore_util::Direction;
use firecore_util::Position;

use crate::character::Character;

use super::NPC;

impl NPC {

    pub fn find_character(&mut self, character: &mut Character) -> bool {
        if self.eye_track(&character.position.coords) {
            self.character.go_next_to(character.position.coords);
            character.freeze();
            true
        } else {
            false
        }
    }

    pub fn eye_track(&self, coords: &Coordinate) -> bool {
        if let Some(trainer) = self.trainer.as_ref() {
            if let Some(tracker) = trainer.tracking {
                let tracker = tracker as isize;
                match self.character.position.direction {
                    firecore_util::Direction::Up => if self.character.position.coords.x == coords.x {
                        if self.character.position.coords.y > coords.y && self.character.position.coords.y - tracker <= coords.y {
                            return true;
                        }
                    }
                    firecore_util::Direction::Down => if self.character.position.coords.x == coords.x {
                        if self.character.position.coords.y < coords.y && self.character.position.coords.y + tracker >= coords.y {
                            return true;
                        }
                    }
                    firecore_util::Direction::Left => if self.character.position.coords.y == coords.y {
                        if self.character.position.coords.x > coords.x && self.character.position.coords.x - tracker <= coords.x {
                            return true;
                        }
                    }
                    firecore_util::Direction::Right => if self.character.position.coords.y == coords.y {
                        if self.character.position.coords.x < coords.x && self.character.position.coords.x + tracker >= coords.x {
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
            self.character.position.direction = dir;
            true
        }).unwrap_or_default()
    }

    pub fn can_interact_from(&self, position: &Position) -> Option<Direction> {
        if position.coords.x == self.character.position.coords.x {
            match position.direction {
                Direction::Up => {
                    if position.coords.y - 1 == self.character.position.coords.y {
                        Some(Direction::Down)
                    } else {
                        None
                    }
                },
                Direction::Down => {
                    if position.coords.y + 1 == self.character.position.coords.y {
                        Some(Direction::Up)
                    } else {
                        None
                    }
                },
                _ => None,
            }
        } else if position.coords.y == self.character.position.coords.y {
            match position.direction {
                Direction::Right => {
                    if position.coords.x + 1 == self.character.position.coords.x {
                        Some(Direction::Left)
                    } else {
                        None
                    }
                },
                Direction::Left => {
                    if position.coords.x - 1 == self.character.position.coords.x {
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