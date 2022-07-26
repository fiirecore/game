use std::{
    iter::Rev,
    ops::{Range, RangeInclusive},
};

use crate::engine::{math::{IVec2, Vec2}, graphics::Draw};

use crate::worldlib::{character::CharacterState, positions::CoordinateInt, TILE_SIZE};

#[derive(Debug, Clone)]
pub struct CharacterCamera {
    pub x: RangeInclusive<CoordinateInt>,
    pub y: Rev<Range<CoordinateInt>>,

    pub focus: Vec2,

    pub offset: IVec2,
}

const fn half_width(w: f32) -> CoordinateInt {
    (w as CoordinateInt + TILE_SIZE as CoordinateInt) >> 1
}

const fn half_height(h: f32) -> CoordinateInt {
    (h as CoordinateInt + TILE_SIZE as CoordinateInt) >> 1
}

const fn half_width_tile(w: CoordinateInt) -> CoordinateInt {
    w >> 4
}

const fn half_height_tile(h: CoordinateInt) -> CoordinateInt {
    (h >> 4) + 2
}

impl CharacterCamera {
    #[deprecated(note = "maybe move to characterstate")]
    pub fn new(draw: &Draw, character: &CharacterState) -> Self {
        let coords = character.position.coords;

        let (hw, hh) = (half_width(draw.width()), half_height(draw.height()));

        Self {
            x: coords.x - half_width_tile(hw)..=coords.x + half_width_tile(hw),
            y: (coords.y - half_height_tile(hh)..coords.y + half_height_tile(hh)).rev(),

            focus: Vec2::new(
                ((coords.x + 1) << 4) as f32 + character.offset.x - hw as f32,
                ((coords.y + 1) << 4) as f32 + character.offset.y - hh as f32,
            )
            .round(),

            offset: Default::default(),
        }
    }

    pub fn offset(&self, offset: IVec2) -> Self {
        // return offset x & y
        Self {
            offset,
            ..self.clone()
        }
    }
}
