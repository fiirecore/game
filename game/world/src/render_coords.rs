use firecore_game::util::{WIDTH, HEIGHT, TILE_SIZE, Coordinate};
use firecore_world_lib::character::player::PlayerCharacter;

use firecore_game::macroquad::prelude::Vec2;

#[derive(Default, Clone, Copy)]
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

    pub fn new(player: &PlayerCharacter) -> Self {

        Self {

            left: player.position.get_x() - HALF_WIDTH_TILE,
            right: player.position.get_x() + HALF_WIDTH_TILE + 1,
            top: player.position.get_y() - HALF_HEIGHT_TILE,
            bottom: player.position.get_y() + HALF_HEIGHT_TILE,

            focus: Vec2::new((player.position.get_x() + 1 << 4) as f32 + player.position.local.offset.x - HALF_WIDTH as f32, (player.position.get_y() + 1 << 4) as f32 + player.position.local.offset.y - HALF_HEIGHT as f32),

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