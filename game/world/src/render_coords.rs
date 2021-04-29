use firecore_game::util::{WIDTH, HEIGHT, TILE_SIZE, Coordinate};
use firecore_world_lib::character::Character;

use firecore_game::macroquad::prelude::Vec2;

#[derive(Default, Debug, Clone, Copy)]
pub struct RenderCoords {

    pub left: isize,
    pub right: isize,
    pub top: isize,
    pub bottom: isize,

    pub focus: Vec2,

    pub offset: Coordinate,

}

const HALF_WIDTH: isize = (WIDTH as isize + TILE_SIZE as isize) >> 1;
const HALF_HEIGHT: isize = (HEIGHT as isize + TILE_SIZE as isize) >> 1;

const HALF_WIDTH_TILE: isize = HALF_WIDTH >> 4;
const HALF_HEIGHT_TILE: isize = (HALF_HEIGHT >> 4) + 2;

impl RenderCoords {

    pub fn new(offset: Coordinate, character: &Character) -> Self {

        let coords = offset + character.position.coords;

        Self {

            left: coords.x - HALF_WIDTH_TILE,
            right: coords.x + HALF_WIDTH_TILE + 1,
            top: coords.y - HALF_HEIGHT_TILE,
            bottom: coords.y + HALF_HEIGHT_TILE,

            focus: Vec2::new((coords.x + 1 << 4) as f32 + character.position.offset.x - HALF_WIDTH as f32, (coords.y + 1 << 4) as f32 + character.position.offset.y - HALF_HEIGHT as f32),

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