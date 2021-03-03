use ahash::{
    AHashMap as HashMap,
    // AHashSet as HashSet
};
use crate::util::TILE_SIZE;
use crate::util::graphics::Texture;
use crate::world::TileId;

const TEXTURE_TICK: f32 = 0.25; // i think its 16/60 not 15/60

pub struct TileTextureManager {

    pub tile_textures: HashMap<TileId, Texture>,
    // animated_textures: HashSet<TileId>,
    accumulator: f32,

}

impl TileTextureManager {

    pub fn new() -> Self {
        Self {
            tile_textures: HashMap::new(),
            // animated_textures: HashSet::new(),
            accumulator: 0.0,
        }
    }

    pub fn setup(&mut self) {
        self.tile_textures.insert(4, crate::util::graphics::texture::byte_texture(include_bytes!("../../../build/assets/flower_texture.png")));
    }

    pub fn update(&mut self, delta: f32) {
        self.accumulator += delta;
        if self.accumulator > TEXTURE_TICK * 5.0 {
            self.accumulator = 0.0;
        }
    }

    pub fn render_tile(&self, tile_id: &u16, x: f32, y: f32) {
        if 4.eq(tile_id) {
            if let Some(texture) = self.tile_textures.get(tile_id) {
                macroquad::prelude::draw_texture_ex(*texture, x, y, macroquad::prelude::WHITE, macroquad::prelude::DrawTextureParams {
                    source: Some(macroquad::prelude::Rect::new(0.0, (self.accumulator / TEXTURE_TICK).floor() * TILE_SIZE as f32, TILE_SIZE as f32, TILE_SIZE as f32)),
                    ..Default::default()
                });
            }
        } else {
            crate::util::graphics::draw_o(self.tile_textures.get(tile_id), x, y);
        }
    }

}