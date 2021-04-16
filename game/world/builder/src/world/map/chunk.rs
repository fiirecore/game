use std::path::PathBuf;

use ahash::AHashMap as HashMap;
use firecore_world_lib::map::MapIdentifier;
use crate::world::SerializedChunkMap;

use firecore_world_lib::map::chunk::WorldChunk;

pub fn new_chunk_map(root_path: &PathBuf, palette_sizes: &HashMap<u8, u16>, serialized_chunk: SerializedChunkMap) -> (MapIdentifier, WorldChunk) {
    println!("    Loading chunk map {}", serialized_chunk.config.name);

    let (identifier, map) = super::load_map_from_config(root_path, palette_sizes, serialized_chunk.config);
    (
        identifier,
        WorldChunk {
            // index: serialized_chunk.piece_index,
            map,
            coords: serialized_chunk.coords,
            connections: serialized_chunk.connections,
        }
    )
    
}
