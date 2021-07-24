use engine::{
    util::{WIDTH, HEIGHT},
    tetra::math::Vec2,
};

use crate::worldlib::{
    character::Character,
    TILE_SIZE,
    positions::{Coordinate, CoordinateInt},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct RenderCoords {

    pub left: CoordinateInt,
    pub right: CoordinateInt,
    pub top: CoordinateInt,
    pub bottom: CoordinateInt,

    pub focus: Vec2<f32>,

    pub offset: Coordinate,

}

const HALF_WIDTH: i32 = (WIDTH as i32 + TILE_SIZE as i32) >> 1;
const HALF_HEIGHT: i32 = (HEIGHT as i32 + TILE_SIZE as i32) >> 1;

const HALF_WIDTH_TILE: i32 = HALF_WIDTH >> 4;
const HALF_HEIGHT_TILE: i32 = (HALF_HEIGHT >> 4) + 2;

impl RenderCoords {

    pub fn new(offset: Coordinate, character: &Character) -> Self {

        let coords = offset + character.position.coords;

        Self {

            left: coords.x - HALF_WIDTH_TILE,
            right: coords.x + HALF_WIDTH_TILE + 1,
            top: coords.y - HALF_HEIGHT_TILE,
            bottom: coords.y + HALF_HEIGHT_TILE,

            #[deprecated(note = "rounding may fix problem of black spaces between tiles while moving")]
            focus: Vec2::new((coords.x + 1 << 4) as f32 + character.offset.x - HALF_WIDTH as f32, (coords.y + 1 << 4) as f32 + character.offset.y - HALF_HEIGHT as f32)
            .round(),

            ..Default::default()
        }

    }

    pub fn offset(&self, offset: Coordinate) -> RenderCoords { // return offset x & y
        RenderCoords {
            offset,
            ..*self
        }
    }

}