use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use worldlib::map::warp::WarpEntry;
use worldlib::map::warp::Warps;

pub fn load_warp_entries(warp_path: PathBuf) -> Warps {
    read_dir(warp_path).map(|dir| {
        dir.flatten()
            .map(|dir| (dir.file_name(), dir.path()))
            .map(|(filename, path)| (filename, read_to_string(&path).unwrap_or_else(|err| panic!("Could not get warp file at {:?} with error {}", path, err)), path))
            .map(|(filename, data, path)| (filename.to_string_lossy().split('.').next().unwrap().parse().unwrap_or_else(|err| panic!("Warp file name \"{:?}\" could not be parsed into ASCII with error {}", filename, err)), ron::from_str::<WarpEntry>(&data).unwrap_or_else(|err| panic!("Could not parse warp entry at {:?} with error {}", path, err))))
            .collect()
    }).unwrap_or_default()
}