use ahash::AHashMap;
use crate::util::texture::Texture;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::world::RenderCoords;
use crate::world::World;
use crate::world::map::WorldMap;
use crate::world::player::Player;
use crate::world::warp::WarpEntry;

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

    fn walkable(&self, x: isize, y: isize) -> u8 {
        if self.in_bounds(x, y) {
            return self.map.walkable(x - self.x, y - self.y);
        } else {
            1
        }        
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        self.map.check_warp(x - self.x, y - self.y)
    }

    fn render(&self, textures: &AHashMap<u16, Texture>, npc_textures: &AHashMap<u8, ThreeWayTexture>, screen: RenderCoords, border: bool) {
        self.map.render(textures, npc_textures, screen.offset(self.x, self.y), border)
    }

    fn on_tile(&mut self, x: isize, y: isize) {
        self.map.on_tile(x - self.x, y - self.y)
    }

    fn input(&mut self, delta: f32, player: &Player) {
        self.map.input(delta, player)
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        self.map.in_bounds(x - self.x, y - self.y)
    }

    fn tile(&self, x: isize, y: isize) -> u16 {
        self.map.tile(x - self.x, y - self.y)
    }

}