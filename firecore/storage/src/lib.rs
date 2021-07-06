use log::warn;
use error::DataError;
use serde::{de::DeserializeOwned, Serialize};
use std::{path::{Path, PathBuf}, sync::atomic::{AtomicBool, Ordering::Relaxed}};

// pub use macroquad::prelude::collections::storage::{try_get as get, try_get_mut as get_mut, store};
use std::fs::read_to_string;

pub use ron::{from_str as deserialize, to_string as serialize};

pub mod error;
pub mod reload;

const DIR1: &str = "rhysholloway"; // To - do: Custom specifiers for directories
const DIR2: &str = "pokemon-game";
const EXTENSION: &str = "ron";

// #[cfg(not(target_arch = "wasm32"))]
pub static SAVE_IN_LOCAL_DIRECTORY: AtomicBool = AtomicBool::new(false);

pub trait PersistantData: Serialize + DeserializeOwned + Default {

    fn path() -> &'static str;

    fn save(&self) -> Result<(), DataError> {
        save(self, crate::directory()?.join(file_name(Self::path())))
    }

}

pub trait Reloadable: PersistantData {
    fn on_reload(&self);
}

pub fn try_load<D: PersistantData + Sized>() -> Result<D, DataError> {
    let path = D::path();
    let dir =  crate::directory()?;
    let path = dir.join(file_name(path));
    let data = match path.exists() {
        true => deserialize(&read_to_string(&*path.to_string_lossy())?)?,
        false => {
            let data = D::default();
            if let Err(err) = save(&data, D::path()) {
                let name = std::any::type_name::<D>();
                let name = name.split("::").last().unwrap_or(name);
                warn!("Could not save new {} with error {}", name, err);
            }
            data
        },
    };
    Ok(data)
}

pub fn save<D: Serialize + DeserializeOwned + Default, P: AsRef<Path>>(data: &D, path: P) -> Result<(), DataError> {
    let dir = crate::directory()?;
    let path = dir.join(path.as_ref());

    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    let string = ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())?;

    std::fs::write(&path, string.as_bytes())?;

    Ok(())
}

pub fn reload<D: Reloadable + Sized>(data: &mut D) -> Result<(), DataError> {
    *data = try_load::<D>()?;
    data.on_reload();
    Ok(())
}

pub fn directory() -> Result<PathBuf, DataError> {
    match SAVE_IN_LOCAL_DIRECTORY.load(Relaxed) {
        true => std::env::current_dir().map_err(DataError::IOError),
        false => match dirs_next::data_dir() {
            Some(data_dir) => {
                let dir = data_dir.join(DIR1).join(DIR2);
                if let Ok(metadata) = std::fs::metadata(&dir) {
                    if !metadata.permissions().readonly() {
                        Ok(dir)
                    } else {
                        Err(DataError::ReadOnly)
                    }
                } else if !dir.exists() {
                    if let Ok(()) = std::fs::create_dir_all(&dir) {
                        directory()
                    } else {
                        Ok(dir)
                    }
                } else {
                    Ok(dir)
                }
            }
            None => std::env::current_dir().map_err(DataError::IOError),
        },
    }
}

pub fn file_name(filename: &str) -> String {
    filename.to_owned() + "." + EXTENSION
}
