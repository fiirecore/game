use serde::{Deserialize, Serialize};

pub mod configuration;
pub mod player_data;
pub mod game_data;

pub mod pokemon;

pub mod text {
    pub mod message;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, enum_iterator::IntoEnumIterator, Deserialize, Serialize)]
pub enum Direction {
	
	Up,
	Down,
	Left,
	Right,
	
}

impl Direction {

	pub fn inverse(&self) -> Direction {
		match *self {
		    Direction::Up => {
				Direction::Down
			}
		    Direction::Down => {
				Direction::Up
			}
		    Direction::Left => {
				Direction::Right
			}
		    Direction::Right => {
				Direction::Left
			}
		}
	}

	pub fn value(&self) -> u8 {
		match *self {
			Direction::Up => 0,
			Direction::Down => 1,
			Direction::Left => 2,
			Direction::Right => 3,
		}
	}

	// Input

	pub fn keybind(&self) -> usize {
		match *self {
		    Direction::Up => 2,
		    Direction::Down => 3,
		    Direction::Left => 4,
		    Direction::Right => 5,
		}
	}

	pub fn offset(&self) -> (i8, i8) {
		match *self {
		    Direction::Up => (0, -1),
		    Direction::Down => (0, 1),
		    Direction::Left => (-1, 0),
		    Direction::Right => (1, 0),
		}
	}

}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {

	pub map_id: String,
	pub map_index: u16,
	pub position: Position,

}

#[derive(Debug, Default, Clone, Copy, Hash, Deserialize, Serialize)]
pub struct Position {

	pub x: isize,
    pub y: isize,
	pub direction: Direction,
    #[serde(skip)]
    pub x_offset: i8,
    #[serde(skip)]
	pub y_offset: i8,

}

impl Position {

	pub fn offset(x: isize, y: isize) {
		
	}

	pub fn subtract(&self, x: isize, y: isize) -> Position {
		Position {
			x: self.x - x,
			y: self.y - y,
			..*self
		}
    }

}