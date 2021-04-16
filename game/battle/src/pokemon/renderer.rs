use game::{
    util::{Reset, Completable},
    macroquad::prelude::{Texture2D, Vec2, draw_texture_ex, DrawTextureParams, Rect, WHITE},
};

const MISSING: f32 = 0.0;
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
            missing: MISSING,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.missing += delta * 128.0;
    }

    pub fn render(&self, texture: Texture2D, y_offset: f32) {
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
        self.missing = MISSING;
    }
}

impl Completable for ActivePokemonRenderer {
    fn is_finished(&self) -> bool {
        self.missing >= SIZE
    }
}