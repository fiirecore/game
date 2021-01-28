use std::path::{Path, PathBuf};
use macroquad::prelude::FileError;
use macroquad::prelude::warn;
use parking_lot::Mutex;

static ASSET_DIR: &str = "assets";

pub trait PersistantData {

    fn load(path: PathBuf) -> Self;

    fn save(&self);

    // async fn reload(&mut self);

}

pub trait PersistantDataLocation: PersistantData {

    fn load_from_file() -> Self;

}

pub fn asset_as_pathbuf<P>(path: P) -> PathBuf where P: AsRef<Path> {
	PathBuf::from(ASSET_DIR).join(path.as_ref())
}



pub async fn load_file<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<u8>, FileError> {
    macroquad::prelude::load_file(path.as_ref().to_str().unwrap()).await
}

pub async fn read_to_string<P: AsRef<std::path::Path>>(path: P) -> Result<String, FileError> {
    return macroquad::prelude::load_string(path.as_ref().to_str().unwrap()).await;
}





lazy_static::lazy_static! {
	static ref FILE: Mutex<Option<Vec<u8>>> = Mutex::new(None); // lol
}

pub fn read_noasync<P: AsRef<std::path::Path>>(path: P) -> Option<Vec<u8>> {
    let path = path.as_ref().to_str().unwrap().to_owned();
    macroquad::miniquad::fs::load_file(&path.clone(), move |bytes| {
        match bytes {
            Ok(bytes) => *FILE.lock() = Some(bytes),
            Err(err) => {
                warn!("Could not read file at {:?} with error {}", &path, err);
            }
        }
    });
    return FILE.lock().take();
}

pub fn read_to_string_noasync<P: AsRef<std::path::Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    match read_noasync(path) {
        Some(bytes) => {
            match std::str::from_utf8(bytes.as_slice()) {
                Ok(str) => return Some(str.to_string()),
                Err(err) => {
                    warn!("Could not read file at {:?} to string with error {}", path, err);
                    //return Err(Box::new(err));
                    return None;
                }
            }
        }
        None => {
            return None;
        }
    }
}