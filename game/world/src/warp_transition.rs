use firecore_game::util::{WIDTH, HEIGHT, Entity, Reset, Completable};

use firecore_game::macroquad::prelude::{Color, BLACK, draw_rectangle};

pub struct WarpTransition {

    alive: bool,
    color: Color,
    // alpha: f32,
    rect_width: f32,
    switch: bool,

    pub recognize_switch: bool,

}

impl WarpTransition {

    pub fn new() -> Self {
        Self {
            alive: false,
            color: [0.0, 0.0, 0.0, 0.0].into(),
            // alpha: 0.0,
            rect_width: WIDTH / 2.0,
            switch: false,
            recognize_switch: false,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.switch {
            self.color.a += delta * 2.5;
            if self.color.a >= 1.0 {
                self.color.a = 1.0;
                self.switch = true;
            }
        } else {
            self.color.a -= delta * 3.0;
            self.rect_width -= delta * 180.0;
            if self.rect_width < 0.0 {
                self.rect_width = 0.0;
            }
        }
    }

    pub fn render(&self) {
        if self.alive {

            draw_rectangle(0.0, 0.0, WIDTH, HEIGHT, self.color);
            if self.switch {
                draw_rectangle(0.0, 0.0, self.rect_width, HEIGHT, BLACK);
                draw_rectangle(WIDTH - self.rect_width, 0.0, self.rect_width, HEIGHT, BLACK);
            }
        }        
    }

    pub fn switch(&self) -> bool {
        self.switch
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

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl Reset for WarpTransition {
    fn reset(&mut self) {
        self.color.a = 0.0;
        self.rect_width = WIDTH / 2.0;
        self.switch = false;
        self.recognize_switch = false;
    }
}

impl Completable for WarpTransition {
    fn is_finished(&self) -> bool {
        self.rect_width <= 0.0
    }
}