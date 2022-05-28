use serde::{Serialize, Deserialize};

use crate::positions::Direction;

use super::chunk::{ChunkOffset, Connection};

pub type MovementId = u8;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Elevation(pub u8);

impl Elevation {
    
    pub const CROSSING: MovementId = 0x0;
    pub const OBSTACLE: MovementId = 0x1;
    /// Height level 0 is used for water.
    pub const WATER: MovementId = 0x4;
    /// Height level 0 obstacle
    pub const WATER_OBSTACLE: MovementId = 0x5;
    pub const HL1: MovementId = 0x8;
    pub const HL1_OBSTACLE: MovementId = 0x9;
    pub const HL2: MovementId = 0xC;
    pub const HL2_OBSTACLE: MovementId = 0xD;
    pub const HL3: MovementId = 0x10;
    pub const HL3_OBSTACLE: MovementId = 0x11;
    pub const HL4: MovementId = 0x14;
    pub const HL4_OBSTACLE: MovementId = 0x15;

    pub const fn can_move(self, code: MovementId) -> bool {
        match self {
            Self(0) | Self(1) => code % 4 == 0,
            Self(elevation) => elevation << 2 == code || matches!(code, Self::CROSSING | Self::WATER),
        }
    }

    pub fn change(&mut self, code: MovementId) {
        if code % 4 == 0 {
            *self = Self(code >> 2);
        }
    }
    
}

pub enum MapMovementResult<'a> {
    Option(Option<MovementId>),
    /// Second argument is for offset
    Chunk(Direction, ChunkOffset, &'a [Connection]),
}

impl MapMovementResult<'_> {
    pub const NONE: Self = Self::Option(None);
}

impl<'a> From<Option<MovementId>> for MapMovementResult<'a> {
    fn from(id: Option<MovementId>) -> Self {
        Self::Option(id)
    }
}

impl<'a> From<Option<(&'a Direction, i32, &'a [Connection])>> for MapMovementResult<'a> {
    fn from(connection: Option<(&'a Direction, i32, &'a [Connection])>) -> Self {
        match connection {
            Some((direction, offset, connection)) => {
                Self::Chunk(*direction, offset as _, connection)
            }
            None => Self::Option(None),
        }
    }
}