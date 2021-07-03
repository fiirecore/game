extern crate firecore_storage as storage;
pub extern crate firecore_saves as player;

pub use storage::*;
use storage::error::DataError;
use player::{PlayerSaves, PlayerSave};

static mut PLAYER_SAVES: Option<PlayerSaves> = None;

pub fn init() -> Result<(), DataError> {
    unsafe { PLAYER_SAVES = Some(PlayerSaves::load()?); }
    Ok(())
}

pub fn saves() -> &'static mut PlayerSaves {
    unsafe { PLAYER_SAVES.as_mut().unwrap() }
}

pub fn data() -> &'static PlayerSave {
    unsafe { PLAYER_SAVES.as_ref().unwrap().get() }
}

pub fn data_mut() -> &'static mut PlayerSave {
    unsafe { PLAYER_SAVES.as_mut().unwrap().get_mut() }
}