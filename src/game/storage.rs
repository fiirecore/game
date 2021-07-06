pub use firecore_storage::*;
pub extern crate firecore_saves as player;

use error::DataError;
use player::{PlayerSave, PlayerSaves};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

static SAVE_IN_LOCAL_DIRECTORY: AtomicBool = AtomicBool::new(false);

pub fn should_save_locally(should: bool) {
    SAVE_IN_LOCAL_DIRECTORY.store(should, Relaxed)
}

pub fn save_locally() -> bool {
    SAVE_IN_LOCAL_DIRECTORY.load(Relaxed)
}

static mut PLAYER_SAVES: Option<PlayerSaves> = None;

pub fn init() -> Result<(), DataError> {
    unsafe {
        PLAYER_SAVES = Some(PlayerSaves::load(save_locally())?);
    }
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
