use std::path::{Path, PathBuf};

static ASSET_DIR: &str = "assets";

pub trait PersistantData {

    fn load<P>(path: P) -> Self where P: AsRef<Path>;

    fn save(&self);

    fn reload(&mut self);

}

pub trait PersistantDataLocation: PersistantData {

    fn load_from_file() -> Self;

}

pub fn asset_as_pathbuf<P>(path: P) -> PathBuf where P: AsRef<Path> {
	PathBuf::from(ASSET_DIR).join(path.as_ref())
}