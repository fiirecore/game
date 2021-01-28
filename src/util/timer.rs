use crate::entity::Entity;

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

    pub fn update(&mut self, delta: f32) {
        if self.is_alive() {
            self.counter += delta;
        }        
    }

    // pub fn halfway(&self) -> bool {
    //     return self.counter << 1 >= self.final_count;
    // }

    pub fn is_finished(&self) -> bool {
        return self.counter >= self.final_time;
    }

    pub fn reset(&mut self) {
        self.counter = 0.0;
    }

}

impl Entity for Timer {

    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.reset();
    }
    
    fn is_alive(&self) -> bool {
        return self.alive;
    }

}