use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::Path;

use log::warn;

use crate::util::file::asset_as_pathbuf;
use crate::world::world_chunk::WorldChunk;
use crate::world::world_chunk_map::WorldChunkMap;
use crate::world::world_map_set::WorldMapSet;
use crate::world::world_map_set_manager::WorldMapSetManager;

use super::chunk_map_loader::new_chunk_map;
use super::map_serializable::MapConfig;
use super::map_set_loader::new_map_set;

pub fn load_maps(palette_sizes: &Vec<u16>, chunk_map: &mut WorldChunkMap, map_sets: &mut WorldMapSetManager) {

    for dir_entry in std::fs::read_dir(asset_as_pathbuf("world").join("maps")).unwrap().map( |res| res.map(|e| e.path())) {
        match dir_entry {
            Ok(dir_entry) => {
                for subdir_entry in dir_entry.read_dir().unwrap().map( |res| res.map(|e| e.path())) {
                    match subdir_entry {
                        Ok(p) => {
                            if let Some(ext) = p.extension() {
                                if ext == OsString::from("toml") {
                                    let maps = crate::io::map::map_loader::map_from_toml(palette_sizes, p);
                                    if let Some(world_chunk) = maps.0 {
                                        chunk_map.insert(world_chunk.0, world_chunk.1);
                                    } else if let Some(map_set) = maps.1 {
                                        map_sets.insert(map_set.0, map_set.1);
                                    }
                                    
                                }
                            }
                        }
                        Err(e) => {
                            warn!("{}", e);
                        }
                    }
                }
                                    
            }
            Err(e) => {
                warn!("{}", e);
            }
        }
    }

}

pub fn map_from_toml<P: AsRef<Path>>(palette_sizes: &Vec<u16>, path: P) -> (
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
                        match new_chunk_map(path.parent().unwrap(), palette_sizes, map_config) {
                            Some(map) => {
                                return (Some(map), None);
                            }
                            None => {
                                warn!("Error reading jigsaw map at path: {:?}", path);
                                return (None, None);
                            }
                        }
                        

                    } else if map_config.warp_map.is_some() {
                        match new_map_set(path.parent().unwrap(), palette_sizes, map_config) {
                            Some(map) => {
                                return (None, Some(map));
                            }
                            None => {
                                warn!("Error reading warp map at path: {:?}", path);
                                return (None, None);
                            }
                        }

                    } else {

                        warn!("Map config at {:?} does not contain either a jigsaw map or a warp map.", &path);
                        return (None, None);

                    }
                    
                }
                Err(err) => {
                    warn!(
                        "Toml file at {:?} is {}",
                        path,
                        err
                    );

                    return (None, None);
                }
            }
        }
        Err(err) => {
            warn!(
                "Error reading file at {:?} to string with error: {}",
                path,
                err
            );
            return (None, None);
        }
    }
}

