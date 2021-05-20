extern crate firecore_storage;
pub extern crate firecore_configuration as configuration;
pub extern crate firecore_saves as player;

pub use firecore_storage::*;

use player::{PlayerSaves, PlayerSave};

pub static mut PLAYER_SAVES: Option<PlayerSaves> = None;

pub async fn init() {
    unsafe { PLAYER_SAVES = Some(load::<PlayerSaves>().await); }
}

pub fn data() -> &'static PlayerSave {
    unsafe { PLAYER_SAVES.as_ref().unwrap().get() }
}

pub fn data_mut() -> &'static mut PlayerSave {
    unsafe { PLAYER_SAVES.as_mut().unwrap().get_mut() }
}