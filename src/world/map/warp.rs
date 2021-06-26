use crate::{
    util::{WIDTH, HEIGHT, Entity, Reset, Completable},
    graphics::draw_rectangle,
    tetra::{Context, graphics::Color},
};

pub struct WarpTransition {

    alive: bool,
    color: Color,
    rect_width: f32,
    switch: (bool, bool),

}

impl WarpTransition {

    const RECT_WIDTH: f32 = WIDTH / 2.0;

    pub const fn new() -> Self {
        Self {
            alive: false,
            color: Color::BLACK,
            rect_width: Self::RECT_WIDTH,
            switch: (false, false),
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.switch.0 {
            self.color.a += delta * 2.5;
            if self.color.a >= 1.0 {
                self.color.a = 1.0;
                self.switch.0 = true;
            }
        } else {
            self.color.a -= delta * 3.0;
            self.rect_width -= delta * 180.0;
            if self.rect_width < 0.0 {
                self.rect_width = 0.0;
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.alive {
            draw_rectangle(ctx, 0.0, 0.0, WIDTH, HEIGHT, self.color);
            if self.switch.0 {
                draw_rectangle(ctx, 0.0, 0.0, self.rect_width, HEIGHT, Color::BLACK);
                draw_rectangle(ctx, WIDTH - self.rect_width, 0.0, self.rect_width, HEIGHT, Color::BLACK);
            }
        }        
    }

    pub fn switch(&mut self) -> bool {
        if self.switch.0 != self.switch.1 {
            self.switch.1 = true;
            true
        } else {
            false
        }
    }

}

impl Entity for WarpTransition {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}

impl Reset for WarpTransition {
    fn reset(&mut self) {
        self.color.a = 0.0;
        self.rect_width = Self::RECT_WIDTH;
        self.switch = (false, false);
    }
}

impl Completable for WarpTransition {
    fn finished(&self) -> bool {
        self.rect_width <= 0.0
    }
}