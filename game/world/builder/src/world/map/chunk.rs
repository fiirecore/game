use std::path::PathBuf;
use worldlib::map::WorldMap;
use crate::world::MapConfig;

pub fn new_chunk_map(root_path: &PathBuf, config: MapConfig) -> WorldMap {
    println!("    Loading chunk map {}", config.name);

    super::load_map_from_config(root_path, config)
    
}
