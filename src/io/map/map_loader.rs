use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::Path;

use log::warn;

use crate::game::world::world_map::world_map_piece::WorldMapPiece;
use crate::game::world::warp_map::warp_map_set::WarpMapSet;
use crate::util::file_util::UNKNOWN_FILENAME_ERR;

use super::jigsaw_map_loader::new_jigsaw_map;
use super::map_serializable::MapConfig;
use super::warp_map_loader::new_warp_map;

pub static NULL_TILE_ID: u16 = 233;

pub fn map_from_toml<P: AsRef<Path>>(palette_sizes: &Vec<u16>, path: P) -> (
    String,
    Option<(usize, WorldMapPiece)>,
    Option<(String, WarpMapSet)>,
)
{
    let path = path.as_ref();

    match read_to_string(path) {
        Ok(string) => {

            let map_config: Result<MapConfig, toml::de::Error> = toml::from_str(string.as_str());

            match map_config {

                Ok(map_config) => {

                    if map_config.jigsaw_map.is_some() {
                        match new_jigsaw_map(path.parent().unwrap(), palette_sizes, &map_config) {
                            Some(map) => {
                                return (map_config.identifier.world_id, Some(map), None);
                            }
                            None => {
                                warn!("Error reading jigsaw map at path: {:?}", path);
                                return (map_config.identifier.world_id, None, None);
                            }
                        }
                        

                    } else if map_config.warp_map.is_some() {
                        match new_warp_map(path.parent().unwrap(), palette_sizes, &map_config) {
                            Some(map) => {
                                return (map_config.identifier.world_id, None, Some(map));
                            }
                            None => {
                                warn!("Error reading warp map at path: {:?}", path);
                                return (map_config.identifier.world_id, None, None);
                            }
                        }

                    } else {

                        warn!("Map config at {:?} does not contain either a jigsaw map or a warp map.", &path);
                        return (map_config.identifier.world_id, None, None);

                    }
                    
                }
                Err(e) => {
                    warn!(
                        "Toml file at {:?} is {}",
                        path.file_name()
                            .unwrap_or(&OsString::from(&UNKNOWN_FILENAME_ERR)),
                        e
                    );

                    return (String::from("null"), None, None);
                }
            }
        }
        Err(err) => {
            warn!(
                "Error reading file at {:?} to string with error: {}",
                path.file_name()
                    .unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)),
                err
            );
            return (String::from("null"),None, None);
        }
    }
}

