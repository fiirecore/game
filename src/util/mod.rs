use firecore_util::Direction;

use firecore_input::Control;

pub mod graphics;
pub mod text;
pub mod image;
pub mod battle_data;

pub static TILE_SIZE: u8 = 16;

pub trait Load {

	fn load(&mut self);
	
	fn on_start(&mut self) {

	}

}

pub trait Quit {

	fn quit(&mut self);

}

pub trait Update {

	fn update(&mut self, delta: f32); 

}

pub trait Render {

	fn render(&self);

}

pub trait Input {

	fn input(&mut self, delta: f32);

}

pub trait Reset {

	fn reset(&mut self);

}
pub trait Completable: Reset {

    fn is_finished(&self) -> bool;

}

pub fn keybind(direction: Direction) -> Control {
	match direction {
		Direction::Up => Control::Up,
		Direction::Down => Control::Down,
		Direction::Left => Control::Left,
		Direction::Right => Control::Right,
	}
}