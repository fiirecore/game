use super::Entity;

#[derive(Debug, Default, Clone)]
pub struct Timer {

    alive: bool,
    counter: f32,
    pub length: f32,

}

impl Timer {

    pub const fn new(alive: bool, length: f32) -> Self {
        Self {
            alive,
            counter: 0.0,
            length,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.is_alive() {
            self.counter += delta;
        }        
    }
    
    pub fn is_finished(&self) -> bool {
        self.counter >= self.length
    }

    pub fn soft_reset(&mut self) {
        if self.length > self.counter {
            self.counter -= self.length;
        } else {
            self.hard_reset();
        }
    }

    pub fn hard_reset(&mut self) {
        self.counter = 0.0;
    }

}

impl Entity for Timer {

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