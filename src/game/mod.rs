pub mod battle_glue;
pub mod config;
pub mod gui;
pub mod init;
pub mod text;

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[deprecated]
static QUIT: AtomicBool = AtomicBool::new(false);

pub fn quit() {
    QUIT.store(true, Relaxed)
}

#[inline(always)]
pub fn should_quit() -> bool {
    QUIT.load(Relaxed)
}

#[deprecated]
pub static DEBUG: AtomicBool = AtomicBool::new(cfg!(debug_assertions));

pub fn set_debug(debug: bool) {
    DEBUG.store(debug, Relaxed);
}

pub fn is_debug() -> bool {
    DEBUG.load(Relaxed)
}