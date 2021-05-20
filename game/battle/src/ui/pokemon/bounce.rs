pub struct PlayerBounce {
    pub offset: f32,
    invert: bool,
}

impl PlayerBounce {

    const MAX: f32 = 1.0;
    const MIN: f32 = 0.0;

    pub const fn new() -> Self {
        Self {
			invert: true,
            offset: Self::MAX,
        }
    }

    pub fn reset(&mut self) {
        self.offset = Self::MIN;
    }

    pub fn update(&mut self, delta: f32) {
        if self.invert {
            self.offset += 3.0 * delta;
            if self.offset >= Self::MAX {
                self.offset = Self::MAX;
                self.invert = false;
            }
        } else {
            self.offset -= 3.0 * delta;
            if self.offset <= Self::MIN {
                self.offset = Self::MIN;
                self.invert = true;
            }
        }
    }

}