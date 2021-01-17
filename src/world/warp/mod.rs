pub mod warp_transition;

use std::path::Path;
use log::warn;
use serde::Deserialize;

use std::fs::read_to_string;

#[derive(Clone, Deserialize)]
pub struct WarpEntry {
    
    pub x: isize,
    pub y: isize,

    pub destination: WarpDestination, // world_id, map_set_id OR "world" for overworld map

}
#[derive(Clone, Deserialize)]
pub struct WarpDestination {

    // pub world_id: String,
    
    pub map_id: String,
    pub map_index: u16,

    pub x: isize,
    pub y: isize,

}

impl WarpEntry {

    pub fn new<P>(path: P) -> Option<WarpEntry> where P: AsRef<Path> {
        let path = path.as_ref();

        let string_result = read_to_string(path);

        match string_result {
            Ok(string) => {

                let warp_entry: Result<WarpEntry, toml::de::Error> = toml::from_str(string.as_str());

                match warp_entry {
                    Ok(warp_entry) => {
                        return Some(warp_entry);
                    }
                    Err(e) => {
                        warn!("Could not parse warp entry at {:?} with error {}", path, e);
                        return None;
                    }
                }

            },
            Err(err) => {

                warn!("Could not read warp entry toml at {:?} to string with error {}", path, err);
                return None;

            }
        }

    }

}