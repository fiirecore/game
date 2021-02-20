use std::path::PathBuf;

//pub mod warp_transition;
use macroquad::prelude::warn;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct WarpEntry {
    
    pub x: isize,
    pub y: isize,

    pub destination: WarpDestination, // world_id, map_set_id OR "world" for overworld map

}
#[derive(Clone, Serialize, Deserialize)]
pub struct WarpDestination {

    // pub world_id: String,
    
    pub map_id: String,
    pub map_index: u16,

    pub x: isize,
    pub y: isize,

}

impl WarpEntry {

    pub fn new(file: PathBuf) -> Option<WarpEntry> {
        match crate::io::get_file_as_string(&file) {
            Ok(data) => {

                let warp_entry: Result<WarpEntry, toml::de::Error> = toml::from_str(&data);

                match warp_entry {
                    Ok(warp_entry) => {
                        return Some(warp_entry);
                    }
                    Err(err) => {
                        warn!("Could not parse warp entry at {:?} with error {}", &file, err);
                        return None;
                    }
                }

            },
            Err(err) => {
                warn!("Could not read warp entry toml at {:?} to string with error {}", &file, err);
                return None;
            }
        }

    }

}