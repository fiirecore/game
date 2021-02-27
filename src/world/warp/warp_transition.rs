use crate::util::Entity;
use crate::util::Reset;
use crate::util::{Update, Render};

#[derive(Default)]
pub struct WarpTransition {

    alive: bool,

    alpha: f32,
    pub waning: bool,

}

impl Reset for WarpTransition {
    fn reset(&mut self) {
        self.alpha = 0.0;
        self.waning = false;
    }
}

impl Update for WarpTransition {

    fn update(&mut self, delta: f32) {
        if self.is_alive() {
            if !self.waning {
                self.alpha += delta * 4.0;
            } else {
                self.alpha -= delta * 4.0;
            }
            if self.alpha > 1.0 {
                self.alpha = 1.0;
                self.waning = true;
            } else if self.alpha < 0.0 {
                self.alpha = 0.0;
                self.despawn();
            }
        }
    }

}

impl Render for WarpTransition {
    
    fn render(&self) {
        crate::util::graphics::draw_rect([0.0, 0.0, 0.0, self.alpha], 0.0, 0.0, crate::BASE_WIDTH as u32, crate::BASE_HEIGHT as u32);
    }

}

impl Entity for WarpTransition {

    fn spawn(&mut self) {
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

}