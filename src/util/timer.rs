use crate::entity::entity::Entity;

pub struct Timer {

    alive: bool,
    counter: usize,
    final_count: usize,

}

impl Timer {

    pub fn new(final_count: usize) -> Self {

        Self {

            alive: false,
            counter: 0,
            final_count: final_count,

        }

    }

    pub fn update(&mut self) {
        if self.is_alive() {
            self.counter+=1;
        }        
    }

    pub fn halfway(&self) -> bool {
        return self.counter << 1 >= self.final_count;
    }

    pub fn is_finished(&self) -> bool {
        return self.counter >= self.final_count;
    }

    pub fn reset(&mut self) {
        self.counter = 0;
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