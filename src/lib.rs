pub extern crate firecore_dependencies as deps;

pub use deps::{log, tetra};

pub extern crate firecore_util as util;
pub extern crate pokemon_firered_clone_storage as storage;
pub mod input;
pub extern crate firecore_pokedex_game as pokedex;

pub extern crate simple_logger as logger;

pub mod audio;
#[cfg(any(feature = "battle", feature = "world"))]
pub mod battle_glue;
pub mod config;
pub mod game;
pub mod graphics;
pub mod gui;
pub mod init;
pub mod text;

#[cfg(feature = "world")]
extern crate firecore_world as worldlib;

#[cfg(feature = "world")]
pub mod world;

use deps::str::TinyStr8;
use input::Control;
use log::warn;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use tetra::Context;
use util::Direction;

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

pub fn play_music(ctx: &Context, id: audio::music::MusicId) {
    if let Err(err) = audio::music::play_music_id(ctx, id) {
        warn!("Could not play music id {:x} with error {}", id, err);
    }
}

pub fn play_music_named(ctx: &Context, music: &str) {
    if let Err(err) = audio::music::play_music_named(ctx, music) {
        warn!(
            "Could not play music named \"{}\" with error {}",
            music, err
        );
    }
}

pub fn play_sound(ctx: &Context, sound: &audio::sound::Sound) {
    if let Err(err) = audio::sound::play_sound(ctx, &sound) {
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
