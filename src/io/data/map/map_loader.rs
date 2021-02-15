use std::path::PathBuf;
use macroquad::prelude::warn;
use crate::world::map::chunk::world_chunk::WorldChunk;

pub fn create_file_test(path: &PathBuf, chunk: &WorldChunk) {
    let path = PathBuf::from("assets").join(path).join("map.json");
    match bincode::serialize(&chunk.map) {
        Ok(contents) => {
            match std::fs::File::create(&path) {
                Ok(mut file) => {
                    if let Err(err) = std::io::Write::write_all(&mut file, &contents) {
                        warn!("Could not write to file at {:?} with error {}", path, err);
                    }
                }
                Err(err) => {
                    warn!("Could not create file at {:?} with error {}", path, err)
                }
            }
        }
        Err(err) => {
            warn!("Could not serialize {} with error {}", &chunk.map.name, err);
        }
    }
}

