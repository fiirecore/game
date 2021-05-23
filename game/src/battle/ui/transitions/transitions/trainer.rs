use crate::{
    util::{Reset, Completable, WIDTH},
    graphics::byte_texture,
    macroquad::prelude::{Texture2D, draw_texture_ex, draw_rectangle, DrawTextureParams, WHITE, BLACK},
};

use crate::battle::ui::transitions::BattleTransition;

pub struct TrainerBattleTransition {

    rect_width: f32,

    texture: Texture2D,

}

impl Default for TrainerBattleTransition {
    fn default() -> Self {
        Self {
            rect_width: Self::DEF_RECT_WIDTH,
            texture: byte_texture(include_bytes!("../../../../../assets/battle/encounter_ball.png")),
        }
    }
}

impl BattleTransition for TrainerBattleTransition {

    fn update(&mut self, delta: f32) {
        self.rect_width += 240.0 * delta;
    }

    fn render(&self) {
        self.draw_lines(0.0, false);
        self.draw_lines(32.0, true);
        self.draw_lines(64.0, false);
        self.draw_lines(96.0, true);
        self.draw_lines(128.0, false);
    }
}

impl TrainerBattleTransition { // To - do: Two grey flashes before rectangles scroll through screen

    const DEF_RECT_WIDTH: f32 = -16.0;

    fn draw_lines(&self, y: f32, invert: bool) {
        draw_rectangle(
            if invert {
                WIDTH - self.rect_width
            } else {
                0.0
            }, 
            y, 
            self.rect_width, 
            32.0,
            BLACK,
        );
        draw_texture_ex(
            self.texture, 
            if invert {
                WIDTH - self.rect_width - 16.0
            } else {
                self.rect_width - 16.0
            }, 
            y, 
            WHITE, 
            DrawTextureParams {
                rotation: (self.rect_width * 2.0).to_radians(),
                ..Default::default()
            },
        );
        
    }

}

impl Reset for TrainerBattleTransition {
    fn reset(&mut self) {
        self.rect_width = Self::DEF_RECT_WIDTH;
    }
}
impl Completable for TrainerBattleTransition {
    fn finished(&self) -> bool {
        self.rect_width >= WIDTH + 16.0
    } 
}