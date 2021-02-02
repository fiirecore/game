pub mod builder;


use crate::util::texture::Texture;

pub struct Background {
	
	alive: bool,
	
	pub x: f32,
	pub y: f32,
	
	texture: Texture,
	
}

impl Background {
	
	pub fn new(texture: Texture, x: f32, y: f32) -> Self {
		
		Self {
			
			alive: false,
			
			x: x,
			y: y,
			
			texture: texture,
			
		}
		
	}
	
}

impl super::GuiComponent for Background {
	
	fn load(&mut self) {
	}
	
	fn enable(&mut self) {
		self.alive = true;
	}
	
	fn disable(&mut self) {
		self.alive = false;
	}
	
	fn is_active(& self) -> bool {
		self.alive
	}
	
	fn render(&self) {
		crate::util::render::draw(self.texture, self.x, self.y);
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
    }
    
}