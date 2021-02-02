use macroquad::prelude::warn;
use crate::world::warp::WarpEntry;

pub fn load_warp_entries(root_path: &include_dir::Dir, map_index: Option<usize>) -> Vec<WarpEntry> {
    let mut warps = Vec::new();

    match root_path.get_dir(root_path.path().join("warps")) {
        Some(warp_path) => {
            match map_index {
                Some(map_index) => {
                    match warp_path.get_dir(warp_path.path().join(String::from("map_") + map_index.to_string().as_str())) {
                        Some(warp_dir_mapset) => {
                            /*if let Some(err) = */add_warp_under_directory(&mut warps, warp_dir_mapset);// {
                            //     warn!("Problem reading warp entry at map set {} under {:?} with error: {}", map_index, &warp_path, err);
                            // }
                        }
                        None => {
                            warn!("Problem reading warp map set directory #{} under path {:?}", map_index, &warp_path);
                        }
                    }
                },
                None => {
                    /*if let Some(err) = */add_warp_under_directory(&mut warps, warp_path);// {
                    //     warn!("Problem reading warp entry under {:?} with error: {}", &root_path, err);
                    // }
                }
            }            
        }
        None => {
            warn!(
                "Could not read warps directory under {:?}",
                root_path
            );
        }
    }

    return warps;
}

fn add_warp_under_directory(warps: &mut Vec<WarpEntry>, dir: include_dir::Dir) {
    for file in dir.files() {
        if let Some(warp_entry) = WarpEntry::new(file) {
            warps.push(warp_entry);
        }
    }
}