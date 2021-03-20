use firecore_util::{Entity, Reset, Completable};

use crate::util::graphics::draw_rect;
use crate::{WIDTH_F32, HEIGHT_F32};

pub struct WarpTransition {

    alive: bool,
    alpha: f32,
    rect_width: f32,
    switch: bool,

    pub recognize_switch: bool,

}

impl WarpTransition {

    pub fn new() -> Self {
        Self {
            alive: false,
            alpha: 0.0,
            rect_width: WIDTH_F32 / 2.0,
            switch: false,
            recognize_switch: false,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.switch {
            self.alpha += delta * 2.5;
            if self.alpha >= 1.5 {
                self.alpha = 1.5;
                self.switch = true;
            }
        } else {
            self.alpha -= delta * 3.0;
            self.rect_width -= delta * 180.0;
            if self.rect_width < 0.0 {
                self.rect_width = 0.0;
            }
        }
    }

    pub fn render(&self) {
        if self.alive {
            draw_rect([0.0, 0.0, 0.0, self.alpha], 0.0, 0.0, WIDTH_F32, HEIGHT_F32);
            if self.switch {
                draw_rect(macroquad::prelude::BLACK, 0.0, 0.0, self.rect_width, HEIGHT_F32);
                draw_rect(macroquad::prelude::BLACK, WIDTH_F32 - self.rect_width, 0.0, self.rect_width, HEIGHT_F32);
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
        self.alpha = 0.0;
        self.rect_width = crate::WIDTH_F32 / 2.0;
        self.switch = false;
        self.recognize_switch = false;
    }
}

impl Completable for WarpTransition {
    fn is_finished(&self) -> bool {
        self.rect_width <= 0.0
    }
}