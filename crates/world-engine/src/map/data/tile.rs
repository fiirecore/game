use worldlib::{
    map::{PaletteId, Palettes, TileId, WorldTile},
    serialized::SerializedPaletteMap,
    TILE_SIZE,
};

use crate::engine::{
    graphics::{Color, DrawParams, Texture},
    math::Rectangle,
    utils::HashMap,
    Context,
};

pub struct PaletteTextureManager {
    pub palettes: HashMap<PaletteId, Palette>,
    accumulator: f32,
}

pub struct Palette {
    pub texture: Texture,
    pub animated: HashMap<TileId, Texture>,
    pub doors: HashMap<TileId, Texture>,
    pub size: TileId,
}

pub struct PaletteTile<P> {
    pub palette: P,
    pub tile: TileId,
}

impl PaletteTextureManager {
    const TEXTURE_TICK: f32 = 0.25; // i think its 16/60 not 15/60

    pub fn new(ctx: &mut Context, palettes: SerializedPaletteMap) -> Self {
        Self {
            palettes: palettes
                .into_iter()
                .map(|(id, palette)| {
                    let texture = Texture::new(ctx, &palette.texture).unwrap();
                    let size = texture.height() as TileId;
                    (
                        id,
                        Palette {
                            texture,
                            animated: palette
                                .animated
                                .into_iter()
                                .map(|(tile, image)| (tile, Texture::new(ctx, &image).unwrap()))
                                .collect(),
                            doors: palette
                                .doors
                                .into_iter()
                                .map(|(tile, image)| (tile, Texture::new(ctx, &image).unwrap()))
                                .collect(),
                            size,
                        },
                    )
                })
                .collect(),

            accumulator: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.accumulator += delta;
        if self.accumulator > Self::TEXTURE_TICK * 5.0 {
            self.accumulator = 0.0;
        }
    }

    pub fn get<'a>(
        &'a self,
        palettes: &Palettes,
        tile: WorldTile,
    ) -> Option<PaletteTile<&'a Palette>> {
        self.palettes
            .get(tile.palette(palettes))
            .map(|palette| PaletteTile {
                palette,
                tile: tile.id(),
            })
    }

    pub fn draw_tile(
        &self,
        ctx: &mut Context,
        palettes: &Palettes,
        tile: WorldTile,
        x: f32,
        y: f32,
        color: Color,
    ) {
        if let Some(PaletteTile { palette, tile }) = self.get(palettes, tile) {
            match palette.animated.get(&tile) {
                Some(texture) => texture.draw(
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
                ),
                None => {
                    let tx = ((tile % 16) << 4) as f32; // width = 256
                    let ty = ((tile >> 4) << 4) as f32;
                    palette.texture.draw(
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
    }
}
