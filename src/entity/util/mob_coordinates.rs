pub struct MobCoordinates {

	pub x: isize,
	pub y: isize,
	pub x_offset: i8,
	pub y_offset: i8,

}

impl Default for MobCoordinates {
	fn default() -> Self {
		Self {
			x: 0,
			y: 0,
			x_offset: 0,
			y_offset: 0,
		}
	}
}

impl MobCoordinates {

	pub fn new(x: isize, y: isize) -> Self {

		Self {
			x: x,
			y: y,
			..Default::default()
		}
		
	}

	pub fn pixel_x(&self) -> isize {
		(self.x << 4) + self.x_offset as isize
	}

	pub fn pixel_y(&self) -> isize {
		(self.y << 4) + self.y_offset as isize
	}

}