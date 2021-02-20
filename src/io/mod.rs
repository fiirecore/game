use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use ahash::AHashSet as HashSet;
use rust_embed::RustEmbed;

pub mod data;
pub mod input;
pub mod args;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct AssetFolder;

pub fn get_file<P: AsRef<Path>>(path: P) -> Option<Cow<'static, [u8]>> {
    return AssetFolder::get(&path.as_ref().to_string_lossy());
}

pub fn get_file_as_string<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    match get_file(&path) {
        Some(file) => return String::from_utf8(file.to_vec()).map_err(|err| anyhow::Error::from(err) ),
        None => return Err(anyhow::format_err!("Could not get file at {:?}", path.as_ref())),
    }
}

pub fn get_dir<P: AsRef<Path>>(path: P) -> HashSet<PathBuf> {
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
    
    