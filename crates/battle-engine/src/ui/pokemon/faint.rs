#[derive(Default, Clone)]
pub struct Faint {
    pub fainting: bool,
    pub remaining: f32,
}

impl Faint {
    pub fn update(&mut self, delta: f32) {
        if self.fainting {
            self.remaining -= delta * 128.0;
            if self.remaining < 0.0 {
                self.remaining = 0.0;
            }
        }
    }

    pub fn fainting(&self) -> bool {
        self.fainting && self.remaining != 0.0
    }
}
