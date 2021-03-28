use firecore_util::Direction;

use firecore_input::Control;
use macroquad::prelude::warn;

pub mod graphics;
pub mod text;
pub mod pokemon;

pub const TILE_SIZE: u8 = 16;

pub fn keybind(direction: Direction) -> Control {
	match direction {
		Direction::Up => Control::Up,
		Direction::Down => Control::Down,
		Direction::Left => Control::Left,
		Direction::Right => Control::Right,
	}
}

pub fn play_music(id: firecore_audio::MusicId) {
    if let Err(err) = firecore_audio::play_music_id(id) {
        warn!("Could not play music id {:x} with error {}", id, err);
    }
}

pub fn play_music_named(music: &str) {
    if let Err(err) = firecore_audio::play_music_named(music) {
        warn!("Could not play music \"{}\" with error {}", music, err);
    }
}