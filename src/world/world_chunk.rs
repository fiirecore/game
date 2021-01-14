use std::collections::HashMap;

use opengl_graphics::Texture;

use crate::entity::texture::three_way_texture::ThreeWayTexture;

use super::ScreenCoords;
use super::World;
use super::warp::WarpEntry;
use super::world_map::WorldMap;

#[derive(Default)]
pub struct WorldChunk {

    pub x: isize,
    pub y: isize,

    pub map: WorldMap,

    pub connections: Vec<u16>,

}

impl WorldChunk {

    pub fn safe_tile(&self, x: isize, y: isize) -> Option<u16> {
        let x = x - self.x;
        let y = y - self.y;
        if self.map.in_bounds(x, y) {
            Some(self.map.tile(x, y))
        } else {
            None
        }
    }

}

impl World for WorldChunk {

    fn walkable(&mut self, x: isize, y: isize) -> u8 {
        if self.in_bounds(x, y) {
            return self.map.walkable(x - self.x, y - self.y);
        } else {
            1
        }        
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        self.map.check_warp(x - self.x, y - self.y)
    }

    fn render(&self, ctx: &mut piston_window::Context, g: &mut opengl_graphics::GlGraphics, textures: &HashMap<u16, Texture>, npc_textures: &HashMap<u8, ThreeWayTexture>, screen: ScreenCoords, border: bool) {
        self.map.render(ctx, g, textures, npc_textures, screen.offset(self.x, self.y), border)
    }

    fn on_tile(&mut self, context: &mut crate::engine::game_context::GameContext, tile_id: u16) {
        self.map.on_tile(context, tile_id)
    }

    fn input(&mut self, context: &mut crate::engine::game_context::GameContext, player: &crate::entity::entities::player::Player) {
        self.map.input(context, player)
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        self.map.in_bounds(x - self.x, y - self.y)
    }

    fn tile(&self, x: isize, y: isize) -> u16 {
        self.map.tile(x - self.x, y - self.y)
    }

}