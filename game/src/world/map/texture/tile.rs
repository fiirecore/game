use crate::{
    deps::hash::HashMap,
    util::TILE_SIZE,
    graphics::{byte_texture, position},
    tetra::{
        Context,
        graphics::{
            Texture,
            Rectangle,
        }
    },
};

use worldlib::{
    PaletteId,
    TileId,
    serialized::SerializedTextures,
};

pub struct TileTextureManager {

    pub palettes: HashMap<PaletteId, Texture>,
    animated: HashMap<TileId, Texture>,
    accumulator: f32,

}

impl TileTextureManager {

    const TEXTURE_TICK: f32 = 0.25; // i think its 16/60 not 15/60

    pub fn new() -> Self {
        Self {
            palettes: HashMap::new(),
            animated: HashMap::new(),
            accumulator: 0.0,
        }
    }

    pub fn setup(&mut self, ctx: &mut Context, textures: SerializedTextures) {
        self.palettes = textures.palettes.into_iter().map(|(id, image)|  (id, byte_texture(ctx, &image))).collect::<HashMap<PaletteId, Texture>>();
        self.animated = textures.animated.into_iter().map(|(tile, image)| (tile, byte_texture(ctx, &image))).collect::<HashMap<TileId, Texture>>();
    }

    pub fn update(&mut self, delta: f32) {
        self.accumulator += delta;
        if self.accumulator > Self::TEXTURE_TICK * 5.0 {
            self.accumulator = 0.0;
        }
    }

    pub fn draw_tile(&self, ctx: &mut Context, texture: &Texture, tile: TileId, x: f32, y: f32) {

        if let Some(texture) = self.animated.get(&tile) {
            texture.draw_region(
                ctx, 
                Rectangle::new(0.0, (self.accumulator / Self::TEXTURE_TICK).floor() * TILE_SIZE, TILE_SIZE, TILE_SIZE), 
                position(x, y)
            );
        } else {
            let tx = ((tile % 16) << 4) as f32; // width = 256
            let ty = ((tile >> 4) << 4) as f32;
            texture.draw_region(
                ctx,
                Rectangle::new(tx, ty, TILE_SIZE, TILE_SIZE),
                position(x, y),
            );
        }
    }

}