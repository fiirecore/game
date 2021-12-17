use error::DataError;
use firecore_battle_gui::pokedex::engine::fs::read_to_string;
use serde::{de::DeserializeOwned, Serialize};
use std::io::{Error as IOError, ErrorKind};
use std::path::{Path, PathBuf};

pub use ron::{from_str as deserialize, to_string as serialize};

pub use crate::engine::log::{info, warn};
pub use crate::engine::inner::miniquad::date::now as time;

pub mod error;
pub mod reload;

const DIR1: &str = "rhysholloway"; // To - do: Custom specifiers for directories
const DIR2: &str = "pokemon-game";
const EXTENSION: &str = "ron";

pub trait PersistantData: Serialize + DeserializeOwned + Default {
    fn path() -> &'static str;

    fn save(&self, local: bool) -> Result<(), DataError> {
        save(
            self,
            local,
            self::directory(local)?.join(file_name(Self::path())),
        )
    }
}

pub async fn try_load<D: PersistantData + Sized>(local: bool) -> Result<D, DataError> {
    #[cfg(not(target_arch = "wasm32"))] {
        let file_name = file_name(D::path());
        let dir = self::directory(local)?;
        let path = dir.join(&file_name);
        let data = match path.exists() {
            true => deserialize(&read_to_string(&*path.to_string_lossy()).await?)?,
            false => {
                let data = D::default();
                if let Err(err) = save(&data, local, file_name) {
                    let name = std::any::type_name::<D>();
                    let name = name.split("::").last().unwrap_or(name);
                    warn!("Could not save new {} with error {}", name, err);
                }
                data
            }
        };
        Ok(data)
    }
    #[cfg(target_arch = "wasm32")] {
        let data = Default::default();
        Ok(data)
    }
}

pub fn save<D: Serialize + DeserializeOwned + Default, P: AsRef<Path>>(
    data: &D,
    local: bool,
    path: P,
) -> Result<(), DataError> {

    let string = ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())?;

    #[cfg(not(target_arch = "wasm32"))] {
        let dir = self::directory(local)?;
        let path = dir.join(path.as_ref());
    
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
    
        std::fs::write(&path, string.as_bytes())?;
    }

    Ok(())
}

pub fn directory(local: bool) -> Result<PathBuf, DataError> {
    fn get_current() -> Result<PathBuf, DataError> {
        let exe = std::env::current_exe()?;
        exe.parent().ok_or_else(|| {
            DataError::IOError(IOError::new(
                ErrorKind::NotFound,
                "Could not find parent directory for executable!",
            ))
        }).map(Into::into)
    }

    match local {
        true => get_current(),
        false => match dirs_next::data_dir() {
            Some(data_dir) => {
                let dir = data_dir.join(DIR1).join(DIR2);
                if !dir.exists() {
                    match std::fs::create_dir_all(&dir) {
                        Ok(..) => directory(local),
                        Err(e) => Err(DataError::IOError(e)),
                    }
                } else {
                    Ok(dir)
                }
            }
            None => get_current(),
        },
    }
}

pub fn file_name(filename: &str) -> String {
    filename.to_owned() + "." + EXTENSION
}
