use worldlib::{
    map::{PaletteId, Palettes, TileId, WorldTile},
    serialized::SerializedPaletteMap,
    TILE_SIZE,
};

use crate::engine::{
    graphics::{Color, Draw, DrawExt, DrawParams, Graphics, Texture},
    math::Rect,
    HashMap,
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

    pub fn new(gfx: &mut Graphics, palettes: SerializedPaletteMap) -> Result<Self, String> {
        Ok(Self {
            palettes: {
                let mut textures = HashMap::with_capacity(palettes.len());
                for (id, palette) in palettes {
                    let texture = gfx.create_texture().from_image(&palette.texture).build()?;
                    let size = texture.height() as TileId;
                    textures.insert(
                        id,
                        Palette {
                            texture,
                            animated: {
                                let mut animated = HashMap::with_capacity(palette.animated.len());
                                for (tile, image) in palette.animated {
                                    animated.insert(
                                        tile,
                                        gfx.create_texture().from_image(&image).build()?,
                                    );
                                }
                                animated
                            },
                            doors: {
                                let mut doors = HashMap::with_capacity(palette.doors.len());
                                for (tile, image) in palette.doors {
                                    doors.insert(
                                        tile,
                                        gfx.create_texture().from_image(&image).build()?,
                                    );
                                }
                                doors
                            },
                            size,
                        },
                    );
                }
                textures
            },
            accumulator: 0.0,
        })
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
        draw: &mut Draw,
        palettes: &Palettes,
        tile: WorldTile,
        x: f32,
        y: f32,
        color: Color,
    ) {
        if let Some(PaletteTile { palette, tile }) = self.get(palettes, tile) {
            match palette.animated.get(&tile) {
                Some(texture) => {
                    draw.texture(
                        texture,
                        x,
                        y,
                        DrawParams {
                            source: Some(Rect {
                                x: 0.0,
                                y: (self.accumulator / Self::TEXTURE_TICK).floor() * TILE_SIZE,
                                width: TILE_SIZE,
                                height: TILE_SIZE,
                            }),
                            ..Default::default()
                        },
                    );
                }
                None => {
                    let tx = ((tile % 16) << 4) as f32; // width = 256
                    let ty = ((tile >> 4) << 4) as f32;
                    draw.texture(
                        &palette.texture,
                        x,
                        y,
                        DrawParams {
                            source: Some(Rect {
                                x: tx,
                                y: ty,
                                width: TILE_SIZE,
                                height: TILE_SIZE,
                            }),
                            color,
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}
