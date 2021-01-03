use std::fs::ReadDir;
use std::fs::read_dir;
use std::path::Path;

use log::warn;

use crate::game::warp::warp_entry::WarpEntry;

pub fn get_warps<P: AsRef<Path>>(root_path: P, map_set_num: Option<usize>) -> Vec<WarpEntry> {

    let root_path = root_path.as_ref();
    let warp_path = root_path.join("warps");

    let mut warps = Vec::new();

    match read_dir(&warp_path) {
        Ok(dir) => {
            match map_set_num {
                Some(map_set_num) => {
                    let mut map_set = String::from("map_");
                    map_set.push_str(map_set_num.to_string().as_str());
                    match read_dir(&warp_path.join(map_set)) {
                        Ok(dir) => {
                            if let Some(err) = add_warp_under_directory(&mut warps, dir) {
                                warn!(
                                    "Error reading warp entry at map set {} under {:?} with error: {}",
                                    map_set_num, root_path, err
                                );
                            }
                        }
                        Err(err) => {
                            warn!("Error reading map set directory #{} under path {:?} with error {}", map_set_num, root_path, err);
                        }
                    }
                },
                None => {
                    if let Some(err) = add_warp_under_directory(&mut warps, dir) {
                        warn!(
                            "Error reading warp entry under {:?} with error: {}",
                            root_path, err
                        );
                    }
                }
            }            
        }
        Err(err) => {
            warn!(
                "Could not read warps directory under {:?} with error {}",
                root_path, err
            );
        }
    }

    return warps;
}

fn add_warp_under_directory(warps: &mut Vec<WarpEntry>, dir: ReadDir) -> Option<std::io::Error> {
    for path_result in dir.map(|res| res.map(|e| e.path())) {
        match path_result {
            Ok(path) => {
                if let Some(warp_entry) = WarpEntry::from_path(path) {
                    warps.push(warp_entry);
                }
            }
            Err(err) => {
                return Some(err);
            }
        }
    }
    return None;
}