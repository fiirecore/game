#[derive(Default)]
pub struct Flicker {
    pub remaining: u8,
    pub accumulator: f32,
}

impl Flicker {

    pub const HALF: f32 = Self::LENGTH / 2.0;
    pub const LENGTH: f32 = 0.20;
    pub const TIMES: u8 = 4;

    pub fn update(&mut self, delta: f32) {
        if self.remaining != 0 {
            self.accumulator += delta;
            if self.accumulator > Self::LENGTH {
                self.accumulator -= Self::LENGTH;
                self.remaining -= 1;
            }
            if self.remaining == 0 {
                self.accumulator = 0.0;
            }
        }
    }

    pub fn flickering(&self) -> bool {
        self.remaining != 0
    }

}