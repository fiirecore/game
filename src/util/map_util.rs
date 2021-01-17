use std::collections::HashMap;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::util::context::GameContext;
use crate::entity::entities::player::Player;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::world::warp::WarpEntry;

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