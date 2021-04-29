use std::ops::AddAssign;

use serde::{Deserialize, Serialize};

// use crate::Coordinate;
use crate::Direction;

const MAX: f32 = crate::TILE_SIZE as f32;

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct PixelOffset {
	pub x: f32,
	pub y: f32,
}

impl PixelOffset {

    pub fn is_none(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn update(&mut self, delta: f32, direction: &Direction) -> bool {
        let offsets = direction.pixel_offset();
        self.add_assign(offsets.scale(60.0 * delta));
        if self.y.abs() >= MAX {
            self.y = 0.0;
            true
        } else if self.x.abs() >= MAX {
            self.x = 0.0;
            true
        } else {
            false
        }
    }

    pub fn scale(self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

}

impl AddAssign for PixelOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}