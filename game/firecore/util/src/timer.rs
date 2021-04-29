use super::Entity;

#[derive(Debug, Default, Clone)]
pub struct Timer {

    alive: bool,
    counter: f32,
    final_time: f32,

}

impl Timer {

    pub fn new(seconds: f32) -> Self {
        Self {
            alive: false,
            counter: 0.0,
            final_time: seconds,
        }
    }

    pub fn set_target(&mut self, seconds: f32) {
        self.final_time = seconds;
    }

    pub fn update(&mut self, delta: f32) {
        if self.is_alive() {
            self.counter += delta;
        }        
    }
    
    pub fn is_finished(&self) -> bool {
        return self.counter >= self.final_time;
    }

    pub fn soft_reset(&mut self) {
        if self.final_time > self.counter {
            self.counter -= self.final_time;
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
        return self.alive;
    }

}