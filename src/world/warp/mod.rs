pub mod warp_transition;
use include_dir::File;
use macroquad::prelude::warn;
use serde::Deserialize;

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

    pub fn new(file: &File) -> Option<WarpEntry> {
        match file.contents_utf8() {
            Some(data) => {

                let warp_entry: Result<WarpEntry, toml::de::Error> = toml::from_str(data);

                match warp_entry {
                    Ok(warp_entry) => {
                        return Some(warp_entry);
                    }
                    Err(e) => {
                        warn!("Could not parse warp entry at {:?} with error {}", file.path, e);
                        return None;
                    }
                }

            },
            None => {
                warn!("Could not read warp entry toml at {:?} to string", file.path);
                return None;
            }
        }

    }

}