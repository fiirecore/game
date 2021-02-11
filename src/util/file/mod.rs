use std::path::PathBuf;

use macroquad::prelude::FileError;

//mod noasync;
//pub use self::noasync::*;

//static ASSET_DIR: &str = "assets";

#[async_trait::async_trait(?Send)]
pub trait PersistantData {

    async fn load(path: PathBuf) -> Self; // replace with async

    fn save(&self);

    // async fn reload(&mut self);

}

#[async_trait::async_trait(?Send)]
pub trait PersistantDataLocation: PersistantData {

    async fn load_from_file() -> Self;

}

// pub fn asset_as_pathbuf<P>(path: P) -> PathBuf where P: AsRef<Path> {
// 	PathBuf::from(ASSET_DIR).join(path.as_ref())
// }

pub async fn load_file<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<u8>, FileError> {
    macroquad::prelude::load_file(&path.as_ref().to_string_lossy()).await
}

pub async fn read_to_string<P: AsRef<std::path::Path>>(path: P) -> Result<String, FileError> {
    return macroquad::prelude::load_string(&path.as_ref().to_string_lossy()).await;
}