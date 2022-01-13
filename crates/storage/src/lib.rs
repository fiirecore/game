#[cfg(feature = "io")]
extern crate firecore_engine as engine;

#[cfg(feature = "io")]
use {
    engine::{fs::read, log::warn},
    serde::{de::DeserializeOwned, Serialize},
};

use std::path::PathBuf;

pub use postcard::{from_bytes as from_bytes, to_allocvec as to_bytes};
pub use ron::{from_str, to_string};

pub mod error;
pub mod reload;

#[cfg(feature = "io")]
pub trait PersistantData: Serialize + DeserializeOwned + Default {
    fn path() -> &'static str;
}

pub trait DataSerializer {
    fn deserialize<D: serde::de::DeserializeOwned>(data: &[u8]) -> Result<D, error::DataError>;

    fn serialize<D: serde::Serialize>(data: &D) -> Result<Vec<u8>, error::DataError>;

    fn extension() -> &'static str;
}

#[cfg(feature = "io")]
pub async fn get<T: DeserializeOwned>(
    path: impl AsRef<std::path::Path>,
) -> Result<T, error::DataError> {
    let bytes = read(path).await?;
    Ok(from_bytes::<T>(&bytes)?)
}

#[cfg(feature = "io")]
pub async fn try_load<S: DataSerializer, D: PersistantData>(
    publisher: Option<&str>,
    application: &str,
) -> Result<D, error::DataError> {
    let path = crate::directory(false, publisher, application)?.join(file::<S, D>());
    #[cfg(not(target_arch = "wasm32"))]
    {
        let data = match path.exists() {
            true => {
                let data = read(&path).await?;
                S::deserialize(&data)?
            }
            false => {
                let data = D::default();
                if let Err(err) = save::<S, D>(&data, publisher, application) {
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
        let data = quad_storage::STORAGE
            .lock()
            .map_err(|_| error::DataError::QuadStorageError)?
            .get(D::path());
        match data {
            Some(data) => Ok(S::deserialize(&base64::decode(data)?)?),
            None => {
                let data = D::default();
                if let Err(err) = save::<S, D>(&data, publisher, application) {
                    let name = std::any::type_name::<D>();
                    let name = name.split("::").last().unwrap_or(name);
                    warn!("Could not save new {} with error {}", name, err);
                }
                Ok(data)
            }
        }
    }
}

#[cfg(feature = "io")]
pub fn file<S: DataSerializer, D: PersistantData>() -> String {
    format!("{}.{}", D::path(), S::extension())
}

#[cfg(feature = "io")]
pub fn save<S: DataSerializer, D: PersistantData>(
    data: &D,
    publisher: Option<&str>,
    application: &str,
) -> Result<(), error::DataError> {
    let data = S::serialize(data)?;

    #[cfg(not(target_arch = "wasm32"))]
    {
        let dir = self::directory(false, publisher, application)?;
        let path = dir.join(format!("{}.{}", D::path(), S::extension()));

        if !path.exists() {
            std::fs::create_dir_all(&dir)?;
        }

        std::fs::write(&path, data)?;
    }

    #[cfg(target_arch = "wasm32")]
    {
        quad_storage::STORAGE
            .lock()
            .map_err(|_| error::DataError::QuadStorageError)?
            .set(D::path(), &base64::encode(data));
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn directory(
    local: bool,
    publisher: Option<&str>,
    application: &str,
) -> Result<PathBuf, error::DataError> {
    Ok(PathBuf::new())
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

#[cfg(feature = "io")]
pub struct RonSerializer;

#[cfg(feature = "io")]
impl DataSerializer for RonSerializer {
    fn deserialize<D: serde::de::DeserializeOwned>(data: &[u8]) -> Result<D, error::DataError> {
        ron::from_str(core::str::from_utf8(data)?).map_err(Into::into)
    }

    fn serialize<D: serde::Serialize>(data: &D) -> Result<Vec<u8>, error::DataError> {
        ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())
            .map(String::into_bytes)
            .map_err(Into::into)
    }

    fn extension() -> &'static str {
        "ron"
    }
}
