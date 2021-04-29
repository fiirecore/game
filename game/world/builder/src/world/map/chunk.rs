use std::path::PathBuf;
use worldlib::map::chunk::WorldChunk;
use crate::world::SerializedChunkMap;
use super::PaletteSizes;

pub fn new_chunk_map(root_path: &PathBuf, palette_sizes: &PaletteSizes, serialized_chunk: SerializedChunkMap) -> WorldChunk {
    println!("    Loading chunk map {}", serialized_chunk.config.name);

    WorldChunk {
        map: super::load_map_from_config(root_path, palette_sizes, serialized_chunk.config),
        coords: serialized_chunk.coords,
        connections: serialized_chunk.connections,
    }
    
}
