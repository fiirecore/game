use crate::{
    positions::{Direction, Position},
    script::ScriptId,
};
use serde::{Deserialize, Serialize};

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
