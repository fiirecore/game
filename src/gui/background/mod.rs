pub mod builder;


use crate::util::graphics::Texture;

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
	
	fn render(&self) {
		crate::util::graphics::draw(self.texture, self.x, self.y);
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
    }
    
}

impl crate::entity::Entity for Background {

	
	fn spawn(&mut self) {
		self.alive = true;
	}
	
	fn despawn(&mut self) {
		self.alive = false;
	}
	
	fn is_alive(& self) -> bool {
		self.alive
	}

}