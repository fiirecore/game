use std::borrow::Cow;
use std::path::Path;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "build"]
pub struct AssetFolder;

// lazy_static::lazy_static! {
//     static ref ROOT_DIR: Directory = Directory {
//         path: "",
//         files: &'static [],
//         dirs: &'static[]
//     };
// }

pub fn create_root_dir() {
    let files: Vec<Cow<'static, str>> = AssetFolder::iter().collect();
    let dirs = Vec::new();
    for file in files {
        println!("file: {}", file);
    }
    Directory {
        path: "",
        files: &[],
        dirs: dirs.as_slice(),
    };
}

// pub fn get_file<P: AsRef<Path>>(path: P) -> Option<&'static File> {
//     None
// }

// pub fn get_dir<P: AsRef<Path>>(path: P) -> Option<&'static Directory> {
//     None
// }


#[derive(Debug, PartialEq)]
pub struct Directory<'a> {

    pub path: &'a str,
    files: &'a [File<'a>],
    dirs: &'a [Directory<'a>],

}

impl<'a> Directory<'a> {

    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    pub fn files(&self) -> &'a [File<'a>] {
        self.files
    }

    pub fn dirs(&self) -> &'a [Directory<'a>] {
        self.dirs
    }

}

#[derive(Debug, PartialEq)]
pub struct File<'a> {

    pub path: &'a str,

}