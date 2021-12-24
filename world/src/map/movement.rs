use crate::{character::Movement, positions::Direction};

use super::chunk::Connection;

pub type MovementId = u8;

pub enum MovementResult<'a> {
    Option(Option<MovementId>),
    /// Second argument is for offset
    Chunk(Direction, i32, &'a Connection),
}

impl MovementResult<'_> {
    pub const NONE: Self = Self::Option(None);
}

impl<'a> From<Option<MovementId>> for MovementResult<'a> {
    fn from(id: Option<MovementId>) -> Self {
        Self::Option(id)
    }
}

impl<'a> From<Option<(&'a Direction, i32, &'a Connection)>> for MovementResult<'a> {
    fn from(connection: Option<(&'a Direction, i32, &'a Connection)>) -> Self {
        match connection {
            Some((direction, offset, connection)) => Self::Chunk(*direction, offset as _, connection),
            None => Self::Option(None),
        }
    }
}

pub fn can_move(movement: Movement, code: MovementId) -> bool {
    match movement {
        Movement::Swimming => can_swim(code),
        _ => can_walk(code),
    }
}

pub fn can_walk(code: MovementId) -> bool {
    matches!(code, 0 | 0xC | 14)
}

pub fn can_swim(code: MovementId) -> bool {
    code == 0x4
}
