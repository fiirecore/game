pub mod util;

pub mod texture {
	pub mod still_texture_manager;
	pub mod movement_texture;
	pub mod movement_texture_manager;
	pub mod texture_manager;
	pub mod four_way_texture;
	pub mod three_way_texture;
}

pub trait Entity {
	
	fn spawn(&mut self);
	
	fn despawn(&mut self);
	
	fn is_alive(&self) -> bool;
	
}