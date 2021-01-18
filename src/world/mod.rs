pub mod map;
pub mod warp;
pub mod npc;
pub mod pokemon;
pub mod gui {
    pub mod player_world_gui;
    pub mod map_window_manager;
}

pub mod player;

use std::collections::HashMap;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::util::TILE_SIZE;
use crate::util::context::GameContext;
use crate::entity::texture::three_way_texture::ThreeWayTexture;

use crate::world::warp::WarpEntry;

use self::player::Player;

pub trait World {

    fn in_bounds(&self, x: isize, y: isize) -> bool;

    fn tile(&self, x: isize, y: isize) -> u16;

    fn walkable(&self, x: isize, y: isize) -> u8;

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry>;

    fn on_tile(&mut self, context: &mut GameContext, x: isize, y: isize);

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, textures: &HashMap<u16, Texture>, npc_textures: &HashMap<u8, ThreeWayTexture>, screen: RenderCoords, border: bool);

    fn input(&mut self, context: &mut GameContext, player: &Player);

}

#[derive(Default, Clone, Copy)]
pub struct RenderCoords {

    pub left: isize,
    pub right: isize,
    pub top: isize,
    pub bottom: isize,

    pub x_focus: isize,
    pub y_focus: isize,

    pub x_tile_offset: isize,
    pub y_tile_offset: isize,

}

static HALF_WIDTH: isize = (crate::BASE_WIDTH as isize + TILE_SIZE as isize) >> 1;
static HALF_HEIGHT: isize = (crate::BASE_HEIGHT as isize + TILE_SIZE as isize) >> 1;

static HALF_WIDTH_TILE: isize = HALF_WIDTH >> 4;
static HALF_HEIGHT_TILE: isize = (HALF_HEIGHT >> 4) + 2;

impl RenderCoords {

    pub fn new(player: &Player) -> Self {

        Self {

            left: player.position.x - HALF_WIDTH_TILE,
            right: player.position.x + HALF_WIDTH_TILE + 1,
            top: player.position.y - HALF_HEIGHT_TILE,
            bottom: player.position.y + HALF_HEIGHT_TILE,

            x_focus: (player.position.x + 1 << 4) + player.position.x_offset as isize - HALF_WIDTH,
            y_focus: (player.position.y + 1 << 4) + player.position.y_offset as isize - HALF_HEIGHT,

            ..Default::default()
        }

    }

    pub fn offset(&self, x: isize, y: isize) -> RenderCoords { // return offset x & y
        RenderCoords {
            x_tile_offset: x,
            y_tile_offset: y,
            ..*self
        }
    }

}