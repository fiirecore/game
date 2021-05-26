use crate::{
    util::{Reset, Completable, WIDTH, HEIGHT},
    graphics::draw_rectangle,
    tetra::{
        Context,
        graphics::{Camera, get_transform_matrix, set_transform_matrix, reset_transform_matrix},
        graphics::Color,
    },
};

use crate::battle::ui::transitions::BattleTransition;


pub struct FlashBattleTransition {
    screen: Color,
    waning: bool,
    index: u8,
    fade: f32,
    zoom: bool,
    zoom_offset: f32,
    // camera: Camera,
}

impl FlashBattleTransition {
    const ZOOM_OFFSET: f32 = 1.0;
    const DEFAULT_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 0.0);
    const FINAL_INDEX: u8 = 4;
}

impl Default for FlashBattleTransition {
    fn default() -> Self {
        Self {
            screen: Self::DEFAULT_COLOR,
            waning: false,
            index: 0,
            fade: 1.0 / 8.0,
            zoom: false,
            zoom_offset: Self::ZOOM_OFFSET,
            // camera: Camera::new(WIDTH, HEIGHT),
        }
    }
}

impl BattleTransition for FlashBattleTransition {

    fn update(&mut self, ctx: &mut Context, delta: f32) {
        if self.waning {
            self.screen.a -= self.fade * 60.0 * delta;
        } else {
            self.screen.a += self.fade * 60.0 * delta;
        }
        if self.screen.a <= 0.0 {
            self.waning = false;
            self.index += 1;
        } else if self.screen.a >= 1.0 {
            self.waning = true;
        }
        if self.index == Self::FINAL_INDEX && self.screen.a <= 0.0 {
            self.screen.a = -2.0;
            self.screen.r = 0.0;
            self.screen.g = 0.0;
            self.screen.b = 0.0;
            self.fade = 1.0 / 16.0;
            self.zoom = true;
        }
        if self.zoom {

            use deps::tetra::math::{Vec2, Vec3};

            self.zoom_offset += 6.0 * delta;

            let mut mat = get_transform_matrix(ctx);

            mat.scale_3d(Vec3::new(self.zoom_offset, self.zoom_offset, 1.0));
            mat.translate_2d(Vec2::new(-(self.zoom_offset - 1.0) * (WIDTH / 2.0), -(self.zoom_offset - 1.0) * (HEIGHT / 2.0)));
            
            set_transform_matrix(ctx, mat);
        }
        if self.finished() {
            reset_transform_matrix(ctx);
        }
    }

    fn draw(&self, ctx: &mut Context) {
        draw_rectangle(
            ctx,
            0.0,
            0.0,
            WIDTH,
            HEIGHT,
            self.screen,
        );
    }

}

impl Reset for FlashBattleTransition {
    fn reset(&mut self) {
        self.screen = Self::DEFAULT_COLOR;
        self.waning = false;
        self.index = 0;
        self.fade = 1.0 / 8.0;
        self.zoom = false;
        self.zoom_offset = Self::ZOOM_OFFSET;
    }
}

impl Completable for FlashBattleTransition {
    fn finished(&self) -> bool {
        self.index >= Self::FINAL_INDEX && self.waning
    }
}