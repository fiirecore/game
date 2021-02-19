use std::path::PathBuf;
use anyhow::Result;
#[cfg(target_arch = "wasm32")]
use anyhow::Error;
use macroquad::prelude::FileError;
use macroquad::prelude::warn;

pub mod noasync;

#[async_trait::async_trait(?Send)]
pub trait PersistantData {

    async fn load(path: PathBuf) -> Self; // replace with async

    fn save(&self);

    async fn reload(&mut self) {}

}

#[async_trait::async_trait(?Send)]
pub trait PersistantDataLocation: PersistantData {

    async fn load_from_file() -> Self;

}

pub fn save_struct<P: AsRef<std::path::Path>>(path: P, data: &impl serde::Serialize) {

    

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(err) = std::fs::create_dir_all(&path) {
                    warn!("Could not create directory at {:?} with error {}", &path, err);
                }
            }
        }
        
        match std::fs::File::create(&path) {
            Ok(mut file) => {
                match serde_json::to_string_pretty(data) {
                    Ok(string) => {
                        if let Err(err) = std::io::Write::write(&mut file, string.as_bytes()) {
                            warn!("Failed to save data with error: {}", err);
                        }
                    }
                    Err(err) => warn!("Failed to encode save data with error: {}", err),
                }

                
            }
            Err(err) => warn!("Could not create save file at {:?} with error {}", &path, err),
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(fname) = path.as_ref().file_name() {
            match serde_json::to_string(&data) {
                Ok(string) => miniquad_cookie::set_cookie(&fname.to_string_lossy(), &string),
                Err(err) => warn!("Could not encode cookie with error: {}", err),
            }
        } else {
            warn!("Could not save cookie!");
        }
    }

}

pub async fn read_string<P: AsRef<std::path::Path>>(path: P) -> Result<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        return Ok((*String::from_utf8_lossy(&load_file(path).await?)).to_owned());
    }
    #[cfg(target_arch = "wasm32")]
    {
        match path.as_ref().file_name() {
            Some(fname) => return Ok(miniquad_cookie::get_cookie(&fname.to_string_lossy())),
            None => return Err(Error::msg("Could not get filename from path!")),
        }
    }
}

pub async fn load_file<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<u8>, FileError> {
    macroquad::prelude::load_file(path).await
}

pub async fn read_to_string<P: AsRef<std::path::Path>>(path: P) -> Result<String, FileError> {
    return macroquad::prelude::load_string(path).await;
}