use std::{ffi::OsString, path::Path};
use serde_derive::Deserialize;

use std::fs::read_to_string;

use crate::util::file_util::UNKNOWN_FILENAME_ERR;

#[derive(Clone)]
pub struct WarpEntry {
    
    pub x: isize,
    pub y: isize,

    pub destination: WarpDestination, // world_id, map_set_id OR "world" for overworld map

}
#[derive(Clone)]
pub struct WarpDestination {

    pub world_id: String,
    pub map_id: String,
    pub map_set_num: usize,
    pub dest_x: isize,
    pub dest_y: isize,

}

impl WarpEntry {

    pub fn from_path<P>(path: P) -> Option<WarpEntry> where P: AsRef<Path> {
        let path = path.as_ref();

        let string_result = read_to_string(path);

        match string_result {
            Ok(string) => {

                let warp_entry: Result<TomlWarpEntry, toml::de::Error> = toml::from_str(string.as_str());

                match warp_entry {
                    Ok(warp_entry) => {
                        return Some(
                            WarpEntry {
        
                                x: warp_entry.x,
                                y: warp_entry.y,
                
                                destination: WarpDestination {
                                    world_id: warp_entry.destination.world_id, 
                                    map_id: warp_entry.destination.map_id,
                                    map_set_num: warp_entry.destination.map_set_num.unwrap_or(0),
                                    dest_x: warp_entry.destination.dest_x,
                                    dest_y: warp_entry.destination.dest_y,
                                },
                
                            }
                        );
                    }
                    Err(e) => {
        
                        println!("Could not parse warp entry at {:?} with error {}", path.file_name().unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)), e);
                        return None;
                    }
                }

            },
            Err(err) => {

                println!("Could not read warp entry toml at {:?} to string with error {}", path, err);
                return None;

            }
        }

    }

    /*

    pub fn new_manual(world_id: String, map_set: String, x: isize, y: isize, dest_x: isize, dest_y: isize) -> WarpEntry {
        WarpEntry {
            x: x,
            y: y,
            destination: (world_id, map_set),
            dest_x: dest_x,
            dest_y: dest_y,
        }
    }

    */

}

#[derive(Debug, Deserialize)]
struct TomlWarpEntry {

    x: isize,
    y: isize,

    destination: Destination,

}

#[derive(Debug, Deserialize)]
struct Destination {

    world_id: String,
    map_id: String,
    map_set_num: Option<usize>,
    dest_x: isize,
    dest_y: isize,

}