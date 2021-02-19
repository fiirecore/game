use serde::{Deserialize, Serialize};

use crate::io::input::Control;

pub mod configuration;
pub mod map;
pub mod player;
pub mod text;
pub mod world;

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
	pub x_offset: isize,
	pub y_offset: isize,

}

impl GlobalPosition {

	pub fn get_x(&self) -> isize {
		self.x_offset + self.local.x
	}

	pub fn get_y(&self) -> isize {
		self.y_offset + self.local.y
	}
 
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct Position {

	pub x: isize,
    pub y: isize,
	pub direction: Direction,
    #[serde(skip)]
    pub x_offset: f32,
    #[serde(skip)]
	pub y_offset: f32,

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

impl Position {

	pub fn subtract(&self, x: isize, y: isize) -> Position {
		Position {
			x: self.x - x,
			y: self.y - y,
			..*self
		}
    }

}