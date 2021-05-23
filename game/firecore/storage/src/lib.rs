pub extern crate macroquad;

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use error::DataError;
use macroquad::prelude::{
    warn, info,
};
use serde::{Serialize, de::DeserializeOwned};

pub use macroquad::prelude::collections::storage::{try_get as get, try_get_mut as get_mut, store};

pub mod error;
pub mod reload;

// To - do: miniquad-cookie produces cookies that dont throw samesite errors

const DIR1: &str = "rhysholloway"; // To - do: Custom specifiers for directories
const DIR2: &str = "pokemon-firered-clone";
const EXTENSION: &str = "ron";

// #[cfg(not(target_arch = "wasm32"))]
pub static SAVE_IN_LOCAL_DIRECTORY: AtomicBool = AtomicBool::new(false);

pub trait PersistantData: Serialize + DeserializeOwned + Default {

    fn file_name() -> &'static str;

}

pub trait Reloadable: PersistantData {

    fn on_reload(&self);

}

pub async fn load<D: PersistantData + Sized + 'static>() -> D {
    try_load::<D>().await.unwrap_or_else(|err| {
        let name = std::any::type_name::<D>();
        let name = name.split("::").last().unwrap_or(name);
        warn!("Could not load {} with error {}", name, err);
        info!("Saving a new {} file!", name);
        let data = D::default();
        if let Err(err) = save(&data) {
            warn!("Could not save new {} with error {}", name, err);
        }
        data
    })
}

pub async fn try_load<D: PersistantData + Sized>() -> Result<D, DataError> {
    let filename = D::file_name();
    #[cfg(not(target_arch = "wasm32"))]
    let string = {
        match crate::directory() {
            Ok(dir) => Ok(
                macroquad::prelude::load_string(
                    &*dir.join(file_name(filename)).to_string_lossy()
                ).await?
            ),
            Err(err) => Err(err),
        }      
    }?;
    #[cfg(target_arch = "wasm32")]
    let string = miniquad_cookie::get_cookie(filename);
    let data: D = ron::from_str(&string).map_err(|error| DataError::Deserialize(filename, error))?;
    Ok(data)
}

pub fn save<D: PersistantData>(data: &D) -> Result<(), DataError> {
    let filename = D::file_name();
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(dir) = crate::directory() {

            let path = dir.join(file_name(filename));

            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(&parent)?;
                }
            }

            let mut file = std::fs::File::create(&path)?;

            let string = ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())?;

            std::io::Write::write_all(&mut file, string.as_bytes())?;

            Ok(())
        } else {
            Err(DataError::NoDirectory)
        }

    }

    #[cfg(target_arch = "wasm32")]
    {
        match ron::to_string(&data) {
            Ok(string) => {
                miniquad_cookie::set_cookie(filename, &string);
                Ok(())
            },
            Err(err) => Err(DataError::Serialize(err)),
        }
    }
}

pub async fn reload<D: Reloadable + Sized>(data: &mut D) -> Result<(), DataError> {
    *data = try_load::<D>().await?;
    data.on_reload();
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn directory() -> Result<std::path::PathBuf, DataError> {
    match SAVE_IN_LOCAL_DIRECTORY.load(Relaxed) {
        true => std::env::current_dir().map_err(|err| DataError::IOError(err)),
        false => match dirs_next::data_dir() {
            Some(data_dir) => {
                let dir = data_dir.join(DIR1).join(DIR2);
                if let Ok(metadata) = std::fs::metadata(&dir) {
                    if !metadata.permissions().readonly() {
                        Ok(dir)
                    } else {
                        Err(DataError::ReadOnly)
                    }
                } else {
                    if !dir.exists() {
                        if let Ok(()) = std::fs::create_dir_all(&dir) {
                            directory()
                        } else {
                            Ok(dir)
                        }
                    } else {
                        Ok(dir)
                    }
                }
            }
            None => {
                std::env::current_dir().map_err(|err| DataError::IOError(err))
            }
        }
    }
}

pub(crate) fn file_name(filename: &str) -> String {
    filename.to_owned() + "." + EXTENSION
}