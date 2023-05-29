pub use notan::graphics::*;

pub use notan::graphics::color::*;

pub use notan::draw::*;
pub use notan::prelude::Graphics;

use notan::math::Rect;

#[derive(Debug, Clone, Copy)]
pub struct DrawParams {
    pub source: Option<Rect>,
    pub flip_x: bool,
    pub color: Color,
}

pub trait DrawExt {
    fn texture(&mut self, texture: &Texture, x: f32, y: f32, params: DrawParams);
}

impl DrawExt for Draw {
    fn texture(&mut self, texture: &Texture, x: f32, y: f32, params: DrawParams) {
        match params.source {
            Some(source) => {
                self.image(texture)
                    .size(
                        source.width * if params.flip_x { -1.0 } else { 1.0 },
                        source.height,
                    )
                    .crop((source.x, source.y), (source.width, source.height))
                    .position(if params.flip_x { x + source.width } else { x }, y);
            }
            None => {
                self.image(texture).position(
                    if params.flip_x {
                        x + texture.width()
                    } else {
                        x
                    },
                    y,
                );
            }
        }
    }
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            source: Default::default(),
            flip_x: Default::default(),
            color: Color::WHITE,
        }
    }
}
