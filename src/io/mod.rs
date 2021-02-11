//pub mod embed;
pub mod data;
pub mod map;
pub mod args;

use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;

use ahash::AHashSet as HashSet;

use macroquad::prelude::warn;
use rust_embed::RustEmbed;

//pub static ASSET_DIR: include_dir::Dir = include_dir::include_dir!("assets");

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct AssetFolder;

pub fn get_file<P: AsRef<Path>>(path: P) -> Option<Cow<'static, [u8]>> {
    AssetFolder::get(&path.as_ref().to_string_lossy())
}

pub fn get_file_as_string<P: AsRef<Path>>(path: P) -> Option<String> {
    match get_file(&path) {
        Some(file) => match std::str::from_utf8(&file) {
            Ok(str) => return Some(str.to_string()),
            Err(err) => {
                warn!("Could not decode string at path {:?} with error {}!", path.as_ref(), err);
                return None;
            }
        },
        None => {
            warn!("Could not get file at {:?}", path.as_ref());
            return None
        },
    }
}

pub fn get_dir<P: AsRef<Path>>(path: P) -> HashSet<PathBuf> {
    let path = path.as_ref();
    let mut paths = HashSet::new();
    for filepath in AssetFolder::iter() {
        let path2 = PathBuf::from(filepath.to_string());
        //macroquad::prelude::info!("Dir: {:?}, file in dir: {:?}", &path, &path2);
        if let Some(parent) = path2.parent() {
            if path.eq(parent) {
                paths.insert(path2);
            }
        }
    }
    if paths.is_empty() {
        for filepath in AssetFolder::iter() {
            let path2 = PathBuf::from(filepath.to_string());
            //macroquad::prelude::info!("Dir: {:?}, file in dir: {:?}", &path, &path2);
            if let Some(parent1) = path2.parent() {
                if let Some(parent) = parent1.parent() {
                    if path.eq(parent) {
                        paths.insert(PathBuf::from(parent1));
                    }
                }
            }
        }
    }
    return paths;
}

// pub fn exists<P: AsRef<Path>>(path: P) -> bool {
//     let path = path.as_ref();
//     for file in AssetFolder::iter() {
//         if PathBuf::from(&*file).eq(path) {
//             return true;
//         }
//     }
//     return false;
// }