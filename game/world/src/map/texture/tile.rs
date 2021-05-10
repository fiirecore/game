use firecore_world_lib::serialized::SerializedTextures;
use game::deps::hash::HashMap;

use firecore_game::macroquad::prelude::{Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect};

use firecore_game::{util::TILE_SIZE, graphics::byte_texture};
use firecore_world_lib::{PaletteId, TileId};

#[derive(Default)]
pub struct TileTextureManager {

    pub palettes: HashMap<PaletteId, Texture2D>,
    animated: HashMap<TileId, Texture2D>,
    accumulator: f32,

}

impl TileTextureManager {

    const TEXTURE_TICK: f32 = 0.25; // i think its 16/60 not 15/60

    pub fn new() -> Self {
        Self {
            palettes: HashMap::new(),
            animated: HashMap::with_capacity(2),
            accumulator: 0.0,
        }
    }

    pub fn setup(&mut self, textures: SerializedTextures) {
        self.palettes = textures.palettes.into_iter().map(|(id, image)|  (id, byte_texture(&image))).collect::<HashMap<PaletteId, Texture2D>>();
        self.animated = textures.animated.into_iter().map(|(tile, image)| (tile, byte_texture(&image))).collect::<HashMap<TileId, Texture2D>>();
    }

    pub fn update(&mut self, delta: f32) {
        self.accumulator += delta;
        if self.accumulator > Self::TEXTURE_TICK * 5.0 {
            self.accumulator = 0.0;
        }
    }

    pub fn render_tile(&self, texture: Texture2D, tile: TileId, x: f32, y: f32) {

        if let Some(texture) = self.animated.get(&tile) {
            draw_texture_ex(*texture, x, y, WHITE, DrawTextureParams {
                source: Some(Rect::new(0.0, (self.accumulator / Self::TEXTURE_TICK).floor() * TILE_SIZE, TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            });
        } else {
            let tx = ((tile % 16) << 4) as f32; // width = 256
            let ty = ((tile >> 4) << 4) as f32;
            draw_texture_ex(texture, x, y, WHITE, DrawTextureParams {
                source: Some(Rect::new(tx, ty, TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            });
        }
    }

}