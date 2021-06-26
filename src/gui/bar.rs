pub struct ProgressBar {
    width: f32,
    gap: f32,
}

impl ProgressBar {

	pub const fn new(width: f32) -> Self {
		Self {
			width,
			gap: 0.0,
		}
	}

    pub fn update(&mut self, delta: f32) {
        if self.moving() {
			if self.gap > 0.0 {
				self.gap -= 60.0 * delta;
				if self.gap < 0.0 {
					self.gap = 0.0;
				}
			} else {
				self.gap += 60.0 * delta;
				if self.gap > 0.0 {
					self.gap = 0.0;
				}
			}
		}
    }

    pub fn resize(&mut self, new: f32, reset: bool) {
        self.gap = if reset {
			0.0
		} else {
			self.width - new
		};
		self.width = new;
    }

	pub fn resize_with_gap(&mut self, width: f32, gap: f32) {
		self.width = width;
		self.gap = gap;
	}

    pub fn moving(&self) -> bool {
        self.gap != 0.0
    }

	pub fn width(&self) -> f32 {
		self.width + self.gap
	}

}