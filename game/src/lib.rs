pub use macroquad;

pub extern crate firecore_util as util;
pub extern crate firecore_storage as data;
pub extern crate firecore_input as input;
pub extern crate firecore_pokedex as pokedex;
pub extern crate firecore_audio as audio;

pub mod npc;
pub mod battle;
pub mod gui;
pub mod text;
pub mod state;
pub mod init;
pub mod graphics;
pub mod textures;

// pub extern crate naia_client_socket as client;
// pub mod network;

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

pub fn is_debug() -> bool {
    DEBUG.load(Relaxed)
}

pub fn play_music(id: audio::MusicId) {
    if let Err(err) = audio::play_music_id(id) {
        warn!("Could not play music id {:x} with error {}", id, err);
    }
}

pub fn play_music_named(music: &str) {
    if let Err(err) = audio::play_music_named(music) {
        warn!("Could not play music \"{}\" with error {}", music, err);
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