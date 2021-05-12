pub extern crate macroquad;

pub extern crate firecore_dependencies as deps;
pub extern crate firecore_util as util;
pub extern crate pokemon_firered_clone_storage as storage; // rename storage or something
pub extern crate firecore_input as input;
pub extern crate firecore_pokedex_game as pokedex;
pub extern crate firecore_audio as audio;

pub mod battle;
pub mod gui;
pub mod text;
pub mod state;
pub mod init;
pub mod graphics;

use deps::tinystr::TinyStr8;
use macroquad::prelude::warn;
use util::Direction;
use input::Control;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

static QUIT: AtomicBool = AtomicBool::new(false);

pub fn quit() {
    QUIT.store(true, Relaxed)
}

#[inline(always)]
pub fn should_quit() -> bool {
    QUIT.load(Relaxed)
}

pub static DEBUG: AtomicBool = AtomicBool::new(cfg!(debug_assertions));

pub fn set_debug(debug: bool) {
    DEBUG.store(debug, Relaxed);
}

pub fn is_debug() -> bool {
    DEBUG.load(Relaxed)
}

pub const CRY_ID: TinyStr8 = unsafe { TinyStr8::new_unchecked(7959107) };

pub fn play_music(id: audio::MusicId) {
    if let Err(err) = audio::play_music_id(id) {
        // match err {
            // audio::error::PlayAudioError::Uninitialized => (),
            warn!("Could not play music id {:x} with error {}", id, err);
        // }
    }
}

pub fn play_music_named(music: &str) {
    if let Err(err) = audio::play_music_named(music) {
        // match err {
            // audio::error::PlayAudioError::Uninitialized => (),
            warn!("Could not play music named \"{}\" with error {}", music, err);
        // }
    }
}

pub fn play_sound(sound: &audio::Sound) {
    if let Err(err) = audio::play_sound(&sound) {
        warn!("Could not play sound {} with error {}", sound, err);
    }
}

pub fn keybind(direction: Direction) -> Control {
	match direction {
		Direction::Up => Control::Up,
		Direction::Down => Control::Down,
		Direction::Left => Control::Left,
		Direction::Right => Control::Right,
	}
}