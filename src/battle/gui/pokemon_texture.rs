use firecore_util::{Reset, Completable};
use macroquad::prelude::{Vec2, draw_texture_ex, DrawTextureParams, Rect, WHITE};

use crate::util::graphics::Texture;

const SIZE: f32 = 64.0;

#[derive(Default)]
pub struct ActivePokemonRenderer {

    pub pos: Vec2,
    missing: f32,

}

impl ActivePokemonRenderer {

    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            missing: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.missing += delta * 128.0;
    }

    pub fn render(&self, texture: Texture, y_offset: f32) {
        if self.missing < SIZE {
            draw_texture_ex(
                texture,
                self.pos.x,
                self.pos.y - texture.height() + y_offset + self.missing,
                WHITE,
                DrawTextureParams {
                    source: if self.missing > 0.0 {
                        Some(
                            Rect::new(0.0, 0.0, texture.width(), SIZE - self.missing)
                        )
                    } else {
                        None
                    },
                    ..Default::default()
                }
            );
        }
    }

}

impl Reset for ActivePokemonRenderer {
    fn reset(&mut self) {
        self.missing = 0.0;
    }
}

impl Completable for ActivePokemonRenderer {
    fn is_finished(&self) -> bool {
        self.missing >= 64.0
    }
}