use std::path::PathBuf;
use worldlib::map::chunk::WorldChunk;
use crate::world::SerializedChunkMap;

pub fn new_chunk_map(root_path: &PathBuf, serialized_chunk: SerializedChunkMap) -> WorldChunk {
    println!("    Loading chunk map {}", serialized_chunk.config.name);

    WorldChunk {
        map: super::load_map_from_config(root_path, serialized_chunk.config),
        coords: serialized_chunk.coords,
        connections: serialized_chunk.connections,
    }
    
}
