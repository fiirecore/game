use std::ops::AddAssign;

use serde::{Deserialize, Serialize};

use crate::{positions::Direction, TILE_SIZE};

type OffsetNum = f32;
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct PixelOffset {
    pub x: OffsetNum,
    pub y: OffsetNum,
}

impl PixelOffset {
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn update(&mut self, direction: &Direction, increment: OffsetNum) -> bool {
        let offsets = direction.pixel_offset(increment);
        self.add_assign(offsets);
        if self.y.abs() >= TILE_SIZE {
            self.y = 0.0;
            true
        } else if self.x.abs() >= TILE_SIZE {
            self.x = 0.0;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn offset(&self) -> OffsetNum {
        if self.x != 0.0 {
            self.x
        } else {
            self.y
        }
    }
}

impl AddAssign for PixelOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
