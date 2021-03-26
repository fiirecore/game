use firecore_util::Direction;

use firecore_input::Control;

pub mod graphics;
pub mod text;
pub mod image;
pub mod battle_data;

pub const TILE_SIZE: u8 = 16;

pub fn keybind(direction: Direction) -> Control {
	match direction {
		Direction::Up => Control::Up,
		Direction::Down => Control::Down,
		Direction::Left => Control::Left,
		Direction::Right => Control::Right,
	}
}