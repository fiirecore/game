use crate::{
    util::{Reset, Completable, WIDTH},
    graphics::{byte_texture, position, draw_rectangle},
    tetra::{
        Context,
        math::Vec2,
        graphics::{
            Texture,
            Color,
        },
    },
};

use crate::battle::ui::transitions::BattleTransition;

pub struct TrainerBattleTransition {

    rect_width: f32,

    texture: Texture,

}

impl BattleTransition for TrainerBattleTransition {

    fn update(&mut self, _ctx: &mut Context, delta: f32) {
        self.rect_width += 240.0 * delta;
    }

    fn draw(&self, ctx: &mut Context) {
        self.draw_lines(ctx, 0.0, false);
        self.draw_lines(ctx, 32.0, true);
        self.draw_lines(ctx, 64.0, false);
        self.draw_lines(ctx, 96.0, true);
        self.draw_lines(ctx, 128.0, false);
    }
}

impl TrainerBattleTransition { // To - do: Two grey flashes before rectangles scroll through screen

    const DEF_RECT_WIDTH: f32 = -16.0;

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            rect_width: Self::DEF_RECT_WIDTH,
            texture: byte_texture(ctx, include_bytes!("../../../../../assets/battle/encounter_ball.png")),
        }
    }

    fn draw_lines(&self, ctx: &mut Context, y: f32, invert: bool) {
        let o = (self.texture.width() >> 1) as f32;
        draw_rectangle(
            ctx,
            if invert {
                WIDTH - self.rect_width
            } else {
                0.0
            }, 
            y, 
            self.rect_width, 
            32.0,
            Color::BLACK,
        );
        self.texture.draw(ctx, position(if invert {
            WIDTH - self.rect_width
        } else {
            self.rect_width
        }, 
        y + o, ).origin(Vec2::new(o, o)).rotation((self.rect_width * 2.0).to_radians()));
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