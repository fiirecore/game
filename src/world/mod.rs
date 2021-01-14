pub mod world_map;

pub mod world_map_set;
pub mod world_map_set_manager;

pub mod world_chunk;
pub mod world_chunk_map;

pub mod world_map_manager;

pub mod warp;

pub mod gui {
    pub mod player_world_gui;
    pub mod map_window_manager;
}

pub mod pokemon;

use std::collections::HashMap;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::entity::entities::player::Player;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::util::render_util::TEXTURE_SIZE;
use crate::util::render_util::VIEW_HEIGHT;
use crate::util::render_util::VIEW_WIDTH;

use crate::world::warp::WarpEntry;

pub trait World {

    fn in_bounds(&self, x: isize, y: isize) -> bool;

    fn tile(&self, x: isize, y: isize) -> u16;

    fn walkable(&mut self, x: isize, y: isize) -> u8;

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry>;

    fn on_tile(&mut self, context: &mut GameContext, tile_id: u16);

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, textures: &HashMap<u16, Texture>, npc_textures: &HashMap<u8, ThreeWayTexture>, screen: ScreenCoords, border: bool);

    fn input(&mut self, context: &mut GameContext, player: &Player);

}

// #[derive(Debug, serde::Deserialize)]
// pub struct WorldConfig {
//     pub palette_sizes: Option<Vec<u16>>,
// }

#[derive(Default, Clone, Copy)]
pub struct ScreenCoords {

    pub x0: isize,
    pub x1: isize,
    pub y0: isize,
    pub y1: isize,

    pub focus_x: isize,
    pub focus_y: isize,

    pub offset_x: isize,
    pub offset_y: isize,

}

impl ScreenCoords {

    pub fn new(player: &Player) -> Self {

        Self {

            x0: player.focus_x >> 4,
            x1: (player.focus_x + (VIEW_WIDTH + TEXTURE_SIZE) as isize) >> 4,
            y0: player.focus_y >> 4,
            y1: (player.focus_y + (VIEW_HEIGHT + TEXTURE_SIZE) as isize) >> 4,

            focus_x: player.focus_x,
            focus_y: player.focus_y,

            offset_x: 0,
            offset_y: 0,

        }

    }

    pub fn offset(&self, x: isize, y: isize) -> ScreenCoords {
        ScreenCoords {
            offset_x: x,
            offset_y: y,
            ..*self
        }
    }

}