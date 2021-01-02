use std::collections::HashMap;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::entity::entities::player::Player;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::game::warp::warp_entry::WarpEntry;

use super::render_util::TEXTURE_SIZE;
use super::render_util::VIEW_HEIGHT;
use super::render_util::VIEW_WIDTH;

pub fn screen_coords(player: &Player) -> (isize, isize, isize, isize) {
    return (
        player.focus_x >> 4,
        (player.focus_x + (VIEW_WIDTH + TEXTURE_SIZE) as isize) >> 4,
        player.focus_y >> 4,
        (player.focus_y + (VIEW_HEIGHT + TEXTURE_SIZE) as isize) >> 4,
    );
}

pub trait GameMap {

    fn tile(&self, x: isize, y: isize) -> u16;

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry>;

    fn input(&mut self, context: &mut GameContext, player: &Player);

}

pub trait GameMapDraw: GameMap {

    fn draw_bottom_map(
        &self,
        ctx: &mut Context,
        g: &mut GlGraphics,
        textures: &HashMap<u16, Texture>,
        npc_textures: &HashMap<u8, ThreeWayTexture>,
        player: &Player,
    );

    fn draw_top_map(
        &self,
        ctx: &mut Context,
        g: &mut GlGraphics,
        textures: &HashMap<u16, Texture>,
        player: &Player,
    );

}