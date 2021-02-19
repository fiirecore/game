use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use ahash::AHashSet as HashSet;
use macroquad::prelude::warn;

#[cfg(feature = "embed")]
use rust_embed::RustEmbed;

pub mod data;
pub mod input;
pub mod args;

#[cfg(feature = "embed")]
#[derive(RustEmbed)]
#[folder = "assets"]
pub struct AssetFolder;

pub async fn get_file<P: AsRef<Path>>(path: P) -> Option<Cow<'static, [u8]>> {
    #[cfg(feature = "embed")]
    return get_file_noasync(path);
    #[cfg(not(feature = "embed"))]
    if let Ok(bytes) = macroquad::file::load_file(PathBuf::from("assets").join(path.as_ref())).await {
        return Some(Cow::Owned(bytes));
    } else {
        return None;
    }
}

pub fn get_file_noasync<P: AsRef<Path>>(path: P) -> Option<Cow<'static, [u8]>> {
    #[cfg(feature = "embed")]
    return AssetFolder::get(&path.as_ref().to_string_lossy());
    #[cfg(not(feature = "embed"))]
    return crate::util::file::noasync::read_noasync(PathBuf::from("assets").join(path.as_ref())).map(|bytes| Cow::Owned(bytes));
}

// pub async fn get_file_fs<P: AsRef<Path>>(path: P) -> Option<Cow<'static, [u8]>> {
//     let mut file = get_file(&path);
//     if file.is_none() {
//         let path = path.as_ref();
//         match macroquad::file::load_file(&path).await {
//             Ok(bytes) => {
//                 file = Some(Cow::Owned(bytes))
//             }
//             Err(err) => {
//                 warn!("Could not open file at {:?} with error {}", path, err);
//             }
//         }
//     }
//     return file;
// }

pub async fn get_file_as_string<P: AsRef<Path>>(path: P) -> Option<String> {
    match get_file(&path).await {
        Some(file) => match std::str::from_utf8(&file) {
            Ok(str) => return Some(str.to_string()),
            Err(err) => {
                warn!("Could not decode string at path {:?} with error {}!", path.as_ref(), err);
                return None;
            }
        },
        None => {
            warn!("Could not get file at {:?}", path.as_ref());
            return None;
        },
    }
}

pub fn get_dir<P: AsRef<Path>>(path: P) -> HashSet<PathBuf> {

    #[cfg(feature = "embed")]
    {
        let path = path.as_ref();
        let mut paths = HashSet::new();
        for filepath in AssetFolder::iter() {
            let filepath = PathBuf::from(&*filepath);
            if let Some(parent) = filepath.parent() {
                if path.eq(parent) {
                    paths.insert(filepath);
                }
            }
        }
        if paths.is_empty() {
            for filepath in AssetFolder::iter() {
                let filepath = PathBuf::from(&*filepath);
                if let Some(first_parent) = filepath.parent() {
                    if let Some(parent) = first_parent.parent() {
                        if path.eq(parent) {
                            paths.insert(PathBuf::from(first_parent));
                        }
                    }
                }
            }
        }

        return paths;

    }

    #[cfg(not(feature = "embed"))]
    {
        let mut paths = HashSet::new();
        if let Ok(dir) = std::fs::read_dir(PathBuf::from("assets").join(path.as_ref())) {
            for entry in dir {
                if let Ok(entry) = entry {
                    paths.insert(entry.path());
                }
            }
        }
        return paths;
    }

}
    
    