use ahash::{
    AHashMap as HashMap,
    AHashSet as HashSet
};

use macroquad::prelude::{Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect};

use crate::util::graphics::byte_texture;
use crate::world::TileId;

const SIZE: f32 = crate::util::TILE_SIZE as f32;
const TEXTURE_TICK: f32 = 0.25; // i think its 16/60 not 15/60

pub struct TileTextureManager {

    pub tile_textures: HashMap<TileId, Texture2D>,
    animated_textures: HashSet<TileId>,
    accumulator: f32,

}

impl TileTextureManager {

    pub fn new() -> Self {
        Self {
            tile_textures: HashMap::new(),
            animated_textures: HashSet::new(),
            accumulator: 0.0,
        }
    }

    pub fn setup(&mut self) {
        self.insert(4, byte_texture(include_bytes!("../../../build/assets/tiles/flower.png")));
        self.insert(299, byte_texture(include_bytes!("../../../build/assets/tiles/water.png")));
    }

    pub fn insert(&mut self, tile_id: u16, texture: Texture2D) {
        self.tile_textures.insert(tile_id, texture);
        self.animated_textures.insert(tile_id);
    }

    pub fn update(&mut self, delta: f32) {
        self.accumulator += delta;
        if self.accumulator > TEXTURE_TICK * 5.0 {
            self.accumulator = 0.0;
        }
    }

    pub fn render_tile(&self, tile_id: &u16, x: f32, y: f32) {
        if self.animated_textures.contains(tile_id) {
            if let Some(texture) = self.tile_textures.get(tile_id) {
                draw_texture_ex(*texture, x, y, WHITE, DrawTextureParams {
                    source: Some(Rect::new(0.0, (self.accumulator / TEXTURE_TICK).floor() * SIZE, SIZE, SIZE)),
                    ..Default::default()
                });
            }
        } else {
            crate::util::graphics::draw_o(self.tile_textures.get(tile_id), x, y);
        }
    }

}