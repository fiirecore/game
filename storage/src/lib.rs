#[cfg(feature = "io")]
extern crate firecore_engine as engine;

#[cfg(feature = "io")]
use {
    engine::{fs::read_to_string, log::warn},
    serde::{de::DeserializeOwned, Serialize},
};

use std::path::PathBuf;

pub use bincode::{deserialize as from_bytes, serialize as to_bytes};
pub use ron::{from_str, to_string};

pub mod error;
pub mod reload;

#[cfg(feature = "io")]
pub trait PersistantData: Serialize + DeserializeOwned + Default {
    fn path() -> &'static str;
}

#[cfg(feature = "io")]
pub async fn try_load<D: PersistantData + Sized>(publisher: Option<&str>, application: &str) -> Result<D, error::DataError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let file_name = D::path();
        let dir = self::directory(false, publisher, application)?;
        let path = dir.join(&file_name);
        let data = match path.exists() {
            true => from_str(&read_to_string(&path).await?)?,
            false => {
                let data = D::default();
                if let Err(err) = save(&data, publisher, application, file_name) {
                    let name = std::any::type_name::<D>();
                    let name = name.split("::").last().unwrap_or(name);
                    warn!("Could not save new {} with error {}", name, err);
                }
                data
            }
        };
        Ok(data)
    }
    #[cfg(target_arch = "wasm32")]
    {
        todo!("Loading on WASM")
    }
}

#[cfg(feature = "io")]
pub fn save<D: Serialize + DeserializeOwned + Default, P: AsRef<std::path::Path>>(
    data: &D,
    publisher: Option<&str>,
    application: &str,
    path: P,
) -> Result<(), error::DataError> {
    let string = ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())?;

    #[cfg(not(target_arch = "wasm32"))]
    {
        let dir = self::directory(false, publisher, application)?;
        let path = dir.join(path.as_ref());

        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }

        std::fs::write(&path, string.as_bytes())?;
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn directory(
    local: bool,
    publisher: Option<&str>,
    application: &str,
) -> Result<PathBuf, error::DataError> {
    use error::DataError;
    use std::io::{Error as IOError, ErrorKind};
    fn get_current() -> Result<PathBuf, error::DataError> {
        let exe = std::env::current_exe()?;
        exe.parent()
            .ok_or_else(|| {
                DataError::IOError(IOError::new(
                    ErrorKind::NotFound,
                    "Could not find parent directory for executable!",
                ))
            })
            .map(Into::into)
    }

    match local {
        true => get_current(),
        false => match dirs_next::data_dir() {
            Some(dir) => {
                let dir = publisher
                    .map(|s| dir.join(s))
                    .unwrap_or(dir)
                    .join(application);

                if !dir.exists() {
                    match std::fs::create_dir_all(&dir) {
                        Ok(..) => directory(local, publisher, application),
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
