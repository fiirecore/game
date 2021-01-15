use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::Path;
use std::path::PathBuf;

use log::warn;

use crate::util::file_util::asset_as_pathbuf;
use crate::world::world_chunk::WorldChunk;
use crate::world::world_chunk_map::WorldChunkMap;
use crate::world::world_map_set::WorldMapSet;
use crate::world::world_map_set_manager::WorldMapSetManager;

use super::jigsaw_map_loader::new_jigsaw_map;
use super::map_serializable::MapConfig;
use super::warp_map_loader::new_warp_map;

pub fn load_maps(world_id: &String, palette_sizes: &Vec<u16>, chunk_map: &mut WorldChunkMap, map_sets: &mut WorldMapSetManager) {

    let mut dir_pb = PathBuf::from("worlds/");
    dir_pb.push(world_id);
    dir_pb.push("maps");

    //println!("{:?}", dir_pb.clone());

    let entries = std::fs::read_dir(asset_as_pathbuf(dir_pb)).unwrap().map( |res| res.map(|e| e.path()));

    for dir_entry in entries {
        match dir_entry {
            Ok(dir_entry) => {
                let dir = dir_entry.read_dir().unwrap().map( |res| res.map(|e| e.path()));
                for subdir_entry in dir {
                    match subdir_entry {
                        Ok(p) => {
                            if let Some(ext) = p.extension() {
                                if ext == OsString::from("toml") {
                                    let maps = crate::io::map::map_loader::map_from_toml(palette_sizes, p);
                                    if let Some(map_data) = maps.1 {
                                        //if map_data.0.eq(&self.player_data.current_world_id) {
                                            chunk_map.insert(map_data.0, map_data.1);
                                        //}
                                    } else if let Some(warp_map_set) = maps.2 {
                                        //if warp_map_set.0.eq(&self.player_data.current_world_id) {
                                            map_sets.insert(warp_map_set.0, warp_map_set.1);
                                        //}
                                    }
                                    
                                }
                            }
                        }
                        Err(e) => {
                            //println!("Error parsing directory: {:?}", subdir_entry);
                            warn!("{}", e);
                        }
                    }
                }
                                    
            }
            Err(e) => {
                //println!("error getting map set files under {:?}", dir_entry);
                warn!("{}", e);
            }
        }
    }

}

pub fn map_from_toml<P: AsRef<Path>>(palette_sizes: &Vec<u16>, path: P) -> (
    String,
    Option<(u16, WorldChunk)>,
    Option<(String, WorldMapSet)>,
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
                Err(err) => {
                    warn!(
                        "Toml file at {:?} is {}",
                        path,
                        err
                    );

                    return (String::from("null"), None, None);
                }
            }
        }
        Err(err) => {
            warn!(
                "Error reading file at {:?} to string with error: {}",
                path,
                err
            );
            return (String::from("null"),None, None);
        }
    }
}

