use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use ahash::AHashSet as HashSet;
use rust_embed::RustEmbed;

pub mod data;
// pub mod input;
pub mod args;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct AssetFolder;

pub fn get_file<P: AsRef<Path>>(path: P) -> Option<Cow<'static, [u8]>> {
    return AssetFolder::get(&path.as_ref().to_string_lossy());
}

pub fn get_file_as_string<P: AsRef<Path>>(path: P) -> Result<String, StringIOError> {
    match get_file(&path) {
        Some(file) => return String::from_utf8(file.to_vec()).map_err(|err| StringIOError::FromUtf8(err) ),
        None => return Err(StringIOError::Missing(format!("Could not get file at {:?}", path.as_ref()))),
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

#[derive(Debug)]
pub enum StringIOError {
    FromUtf8(std::string::FromUtf8Error),
    Missing(String),
}
    
impl std::error::Error for StringIOError {}

impl core::fmt::Display for StringIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringIOError::FromUtf8(err) => {
                write!(f, "{}", err)
            }
            StringIOError::Missing(string) => {
                write!(f, "{}", string)
            }
        }
    }
}