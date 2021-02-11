use std::path::PathBuf;
use crate::world::warp::WarpEntry;

pub fn load_warp_entries(root_path: &PathBuf, map_index: Option<usize>) -> Vec<WarpEntry> {
    let mut warps = Vec::new();

    let warp_path = root_path.join("warps");

    //match root_path.get_dir() {
    //    Some(warp_path) => {
    match map_index {
        Some(map_index) => {
            //match warp_path.get_dir() {
            //    Some(warp_dir_mapset) => {
                    /*if let Some(err) = */add_warp_under_directory(&mut warps, warp_path.join(String::from("map_") + map_index.to_string().as_str()));// {
                    //     warn!("Problem reading warp entry at map set {} under {:?} with error: {}", map_index, &warp_path, err);
                    // }
            //     }
            //     None => {
            //         warn!("Problem reading warp map set directory #{} under path {:?}", map_index, &warp_path);
            //     }
            // }
        },
        None => {
            /*if let Some(err) = */add_warp_under_directory(&mut warps, warp_path);// {
            //     warn!("Problem reading warp entry under {:?} with error: {}", &root_path, err);
            // }
        }
    }            
        //}
        // None => {
        //     warn!(
        //         "Could not read warps directory under {:?}",
        //         root_path
        //     );
        // }
    //}

    return warps;
}

fn add_warp_under_directory(warps: &mut Vec<WarpEntry>, dir: PathBuf) {
    for file in crate::io::get_dir(dir) {
        if let Some(warp_entry) = WarpEntry::new(file) {
            warps.push(warp_entry);
        }
    }
}