use crate::engine::graphics::{
    Color, Draw, DrawImages, DrawShapes, DrawTransform, Graphics, Texture,
};

use crate::battle_wrapper::transition::BattleTransition;

pub struct TrainerBattleTransition {
    rect_width: f32,
    max_width: f32,
    texture: Texture,
}

impl BattleTransition for TrainerBattleTransition {
    fn update(&mut self, delta: f32) -> bool {
        self.rect_width += 240.0 * delta;
        self.rect_width >= self.max_width + 16.0
    }

    fn draw(&self, draw: &mut Draw) {
        self.draw_lines(draw, 0.0, false);
        self.draw_lines(draw, 32.0, true);
        self.draw_lines(draw, 64.0, false);
        self.draw_lines(draw, 96.0, true);
        self.draw_lines(draw, 128.0, false);
    }

    fn reset(&mut self) {
        self.rect_width = Self::DEF_RECT_WIDTH;
    }
}

impl TrainerBattleTransition {
    // To - do: Two grey flashes before rectangles scroll through screen

    const DEF_RECT_WIDTH: f32 = -16.0;

    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        Ok(Self {
            rect_width: Self::DEF_RECT_WIDTH,
            max_width: gfx.size().0 as f32,
            texture: gfx
                .create_texture()
                .from_image(include_bytes!(
                    "../../../../assets/battle/encounter_ball.png"
                ))
                .build()?,
        })
    }

    fn draw_lines(&self, draw: &mut Draw, y: f32, invert: bool) {
        let o = (self.texture.width() / 2.0) as f32;
        let w = draw.width();
        draw.rect(
            (if invert { w - self.rect_width } else { 0.0 }, y),
            (self.rect_width, 32.0),
        )
        .color(Color::BLACK);
        draw.image(&self.texture)
            .position(
                if invert {
                    w - self.rect_width
                } else {
                    self.rect_width
                },
                y + o,
            )
            .rotate((self.rect_width * 2.0).to_radians());
    }
}
