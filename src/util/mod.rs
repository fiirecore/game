use serde::{Deserialize, Serialize};

use crate::io::input::Control;

pub mod graphics;
pub mod text;
pub mod timer;
pub mod image;
pub mod file;
pub mod battle_data;

pub static TILE_SIZE: u8 = 16;

pub trait Entity {
	
	fn spawn(&mut self);
	
	fn despawn(&mut self);
	
	fn is_alive(&self) -> bool;
	
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, enum_iterator::IntoEnumIterator)]
pub enum Direction { // move to util
	
	Up,
	Down,
	Left,
	Right,
	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {

	pub map_id: String,
	pub map_index: u16,
	pub position: GlobalPosition,

}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct GlobalPosition {

	pub local: Position,
	pub offset: Coordinate,

}

impl GlobalPosition {

	pub fn get_x(&self) -> isize {
		self.offset.x + self.local.coords.x
	}

	pub fn get_y(&self) -> isize {
		self.offset.y + self.local.coords.y
	}
 
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct Position {

	pub coords: Coordinate,
	pub direction: Direction,
    #[serde(skip)]
    pub x_offset: f32,
    #[serde(skip)]
	pub y_offset: f32,

}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Coordinate {

	pub x: isize,
	pub y: isize,

}

impl Direction {

	pub fn inverse(&self) -> Direction {
		match self {
		    Direction::Up => Direction::Down,
		    Direction::Down => Direction::Up,
		    Direction::Left => Direction::Right,
		    Direction::Right => Direction::Left,
		}
	}

	pub fn value(&self) -> u8 {
		match self {
			Direction::Down => 0,
			Direction::Up => 1,
			Direction::Left => 2,
			Direction::Right => 3,
		}
	}

	// Input

	pub fn keybind(&self) -> Control {
		match self {
		    Direction::Up => Control::Up,
		    Direction::Down => Control::Down,
		    Direction::Left => Control::Left,
		    Direction::Right => Control::Right,
		}
	}

	pub fn offset(&self) -> (f32, f32) {
		match self {
		    Direction::Up => (0.0, -1.0),
		    Direction::Down => (0.0, 1.0),
		    Direction::Left => (-1.0, 0.0),
		    Direction::Right => (1.0, 0.0),
		}
	}

}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}

impl Coordinate {

	pub fn subtract(&self, x: isize, y: isize) -> Coordinate {
		Coordinate {
			x: self.x - x,
			y: self.y - y,
			..*self
		}
    }

	pub fn towards(&self, destination: &Coordinate) -> Direction {
		if (self.x - destination.x).abs() <= (self.y - destination.y).abs() {
			if self.x > destination.x {
				Direction::Left
			} else {
				Direction::Right
			}
		} else {
			if self.y > destination.y {
				Direction::Up
			} else {
				Direction::Down
			}
		}
	}

}