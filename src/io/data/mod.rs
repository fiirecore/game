use std::path::PathBuf;
use anyhow::{Error, Result};

pub mod configuration;
pub mod map;
pub mod player;
pub mod text;
pub mod world;

lazy_static::lazy_static! {
	pub static ref DATA_DIR: Option<directories_next::ProjectDirs> = directories_next::ProjectDirs::from("net", "rhysholloway", crate::DEBUG_NAME);
}

pub fn get_save_dir() -> Result<PathBuf> {
    let path = DATA_DIR.as_ref().map(|dir| PathBuf::from(dir.data_dir()));
    if let Some(real_path) = path.as_ref() {
        if let Ok(metadata) = std::fs::metadata(real_path) {
            if !metadata.permissions().readonly() {
                return path.ok_or(Error::msg("Read-only directory"));
            }
        } else {
            if !real_path.exists() {
                if let Ok(()) = std::fs::create_dir_all(real_path) {
                    return get_save_dir();
                }
            }
        }
    }
    std::env::current_dir().map_err(|err| Error::from(err))
}