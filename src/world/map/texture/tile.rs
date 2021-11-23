use crate::engine::{
    graphics::{Color, DrawParams, Texture},
    math::Rectangle,
    Context,
};
use std::collections::HashMap;
use worldlib::{
    map::{PaletteId, TileId},
    serialized::SerializedTextures,
    TILE_SIZE,
};

use crate::world::map::warp::WarpTransition;

#[derive(Default)]
pub struct TileTextureManager {
    pub palettes: HashMap<PaletteId, Texture>,
    animated: HashMap<TileId, Texture>,
    accumulator: f32,
}

impl TileTextureManager {
    const TEXTURE_TICK: f32 = 0.25; // i think its 16/60 not 15/60

    pub fn setup(
        &mut self,
        ctx: &mut Context,
        warper: &mut WarpTransition,
        textures: SerializedTextures,
    ) {
        self.palettes = textures
            .palettes
            .into_iter()
            .map(|(id, image)| (id, Texture::new(ctx, &image).unwrap()))
            .collect::<HashMap<PaletteId, Texture>>();
        self.animated = textures
            .animated
            .into_iter()
            .map(|(tile, image)| (tile, Texture::new(ctx, &image).unwrap()))
            .collect::<HashMap<TileId, Texture>>();

        let mut map = HashMap::with_capacity(textures.doors.len());
        for (loc, image) in textures.doors {
            let texture = Texture::new(ctx, &image).unwrap();
            for loc in loc {
                map.insert(loc, texture.clone());
            }
        }
        warper.doors = map;
    }

    pub fn update(&mut self, delta: f32) {
        self.accumulator += delta;
        if self.accumulator > Self::TEXTURE_TICK * 5.0 {
            self.accumulator = 0.0;
        }
    }

    pub fn draw_tile(
        &self,
        ctx: &mut Context,
        texture: &Texture,
        tile: TileId,
        x: f32,
        y: f32,
        color: Color,
    ) {
        if let Some(texture) = self.animated.get(&tile) {
            texture.draw(
                ctx,
                x,
                y,
                DrawParams {
                    source: Some(Rectangle::new(
                        0.0,
                        (self.accumulator / Self::TEXTURE_TICK).floor() * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    )),
                    color,
                    ..Default::default()
                },
            );
        } else {
            let tx = ((tile % 16) << 4) as f32; // width = 256
            let ty = ((tile >> 4) << 4) as f32;
            texture.draw(
                ctx,
                x,
                y,
                DrawParams {
                    source: Some(Rectangle::new(tx, ty, TILE_SIZE, TILE_SIZE)),
                    color,
                    ..Default::default()
                },
            );
        }
    }
}
