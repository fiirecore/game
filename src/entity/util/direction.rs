#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

    pub fn value(&self) -> &str {
		match *self {
			Direction::Up => "Up",
			Direction::Down => "Down",
			Direction::Left => "Left",
			Direction::Right => "Right",
		}
	}

	pub fn int_value(&self) -> u8 {
		match *self {
			Direction::Up => 0,
			Direction::Down => 1,
			Direction::Left => 2,
			Direction::Right => 3,
		}
	}
	
	pub fn from_string(string: &str) -> Option<Direction> {
		match string {
			"Up" => Some(Direction::Up),
			"Down" => Some(Direction::Down),
			"Left" => Some(Direction::Left),
			"Right" => Some(Direction::Right),
			&_ => {
				println!("could not match direction");
				None
			},
		}
	}

	pub fn from_int(int: u8) -> Option<Direction> {
		match int {
			0 => Some(Direction::Up),
			1 => Some(Direction::Down),
			2 => Some(Direction::Left),
			3 => Some(Direction::Right),
			_ => {
				println!("could not match direction");
				None
			},
		}
	}

}