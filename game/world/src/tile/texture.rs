use game::deps::hash::HashMap;

use firecore_game::macroquad::prelude::{Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect};

use firecore_game::{util::TILE_SIZE, graphics::{byte_texture, draw_o}};
use firecore_world_lib::TileId;

const TEXTURE_TICK: f32 = 0.25; // i think its 16/60 not 15/60

pub struct TileTextureManager {

    pub textures: HashMap<TileId, Texture2D>,
    pub animated: Vec<TileId>,
    accumulator: f32,

}

impl TileTextureManager {

    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            animated: Vec::with_capacity(2),
            accumulator: 0.0,
        }
    }

    #[deprecated(note = "To - do: load from config file")]
    pub fn setup(&mut self) {
        self.insert(4, byte_texture(include_bytes!("../../assets/tiles/flower.png")));
        self.insert(299, byte_texture(include_bytes!("../../assets/tiles/water.png")));
    }

    pub fn insert(&mut self, tile_id: u16, texture: Texture2D) {
        self.textures.insert(tile_id, texture);
    }

    pub fn update(&mut self, delta: f32) {
        self.accumulator += delta;
        if self.accumulator > TEXTURE_TICK * 5.0 {
            self.accumulator = 0.0;
        }
    }

    pub fn render_tile(&self, tile_id: &TileId, x: f32, y: f32) {
        if self.animated.contains(tile_id) {
            if let Some(texture) = self.textures.get(tile_id) {
                draw_texture_ex(*texture, x, y, WHITE, DrawTextureParams {
                    source: Some(Rect::new(0.0, (self.accumulator / TEXTURE_TICK).floor() * TILE_SIZE, TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                });
            }
        } else {
            draw_o(self.textures.get(tile_id).map(|texture| *texture), x, y);
        }
    }

}