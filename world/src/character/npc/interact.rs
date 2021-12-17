use crate::{
    positions::{Coordinate, Destination, Direction, Position},
    script::ScriptId,
};
use serde::{Deserialize, Serialize};

use crate::{character::Character};

use super::Npc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NpcInteract {
    Message(Vec<Vec<String>>),
    Script(ScriptId),
    Nothing,
}

impl Default for NpcInteract {
    fn default() -> Self {
        Self::Nothing
    }
}

impl NpcInteract {
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::Nothing)
    }
}

impl Npc {
    pub fn find_character(&mut self, character: &mut Character) -> bool {
        if self.eye_track(&character.position.coords) {
            self.character.pathing.extend(
                &self.character.position,
                Destination::next_to(&self.character.position, character.position.coords),
            );
            character.freeze();
            true
        } else {
            false
        }
    }

    pub fn eye_track(&self, coords: &Coordinate) -> bool {
        if let Some(trainer) = self.trainer.as_ref() {
            if let Some(tracker) = trainer.tracking {
                let tracker = tracker as i32;
                match self.character.position.direction {
                    Direction::Up => {
                        if self.character.position.coords.x == coords.x
                            && self.character.position.coords.y > coords.y
                            && self.character.position.coords.y - tracker <= coords.y
                        {
                            return true;
                        }
                    }
                    Direction::Down => {
                        if self.character.position.coords.x == coords.x
                            && self.character.position.coords.y < coords.y
                            && self.character.position.coords.y + tracker >= coords.y
                        {
                            return true;
                        }
                    }
                    Direction::Left => {
                        if self.character.position.coords.y == coords.y
                            && self.character.position.coords.x > coords.x
                            && self.character.position.coords.x - tracker <= coords.x
                        {
                            return true;
                        }
                    }
                    Direction::Right => {
                        if self.character.position.coords.y == coords.y
                            && self.character.position.coords.x < coords.x
                            && self.character.position.coords.x + tracker >= coords.x
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn interact_from(&mut self, position: &Position) -> bool {
        self.can_interact_from(position)
            .map(|dir| {
                self.character.position.direction = dir;
                true
            })
            .unwrap_or_default()
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
                }
                Direction::Down => {
                    if position.coords.y + 1 == self.character.position.coords.y {
                        Some(Direction::Up)
                    } else {
                        None
                    }
                }
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
                }
                Direction::Left => {
                    if position.coords.x - 1 == self.character.position.coords.x {
                        Some(Direction::Right)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
