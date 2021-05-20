use std::path::{Path, PathBuf};
use worldlib::{
    map::{
        MapIdentifier,
        WorldMap,
        // WorldMapState,
        // Border,
        manager::WorldMapManager,
        chunk::{
            WorldChunk,
            map::WorldChunkMap,
        },
        set::{
            WorldMapSet,
            manager::WorldMapSetManager,
        }
    },
    serialized::SerializedTextures
};

use crate::gba_map::get_gba_map;
use crate::world::textures::get_textures;

use super::MapConfig;

pub mod chunk;
pub mod set;

pub fn load_maps<P: AsRef<Path>>(maps: P, textures: P) -> (WorldMapManager, SerializedTextures) {

    let maps = maps.as_ref();
    let textures = textures.as_ref();

    let mut chunk_map = WorldChunkMap::default();
    let mut map_set_manager = WorldMapSetManager::default();
    let textures = get_textures(textures);
    println!("Loaded {} palettes and {} animated textures", textures.palettes.len(), textures.animated.len());

    println!("Loading maps...");

    for worlds in std::fs::read_dir(maps).unwrap_or_else(|err| panic!("Could not read directory at {:?} with error {}", maps, err)) {
        let worlds = worlds.unwrap_or_else(|err| panic!("Could not get directory entry under {:?} with error {}", maps, err)).path();
        if let Ok(dir) = std::fs::read_dir(&worlds) {
            for entry in dir {
                if let Ok(entry) = entry {
                    let file = entry.path();
                    if let Some(ext) = file.extension() {
                        if ext == std::ffi::OsString::from("ron") {
                            let (cm, ms) = load_map(&worlds, &file);
                            if let Some(chunk) = cm {
                                chunk_map.chunks.insert(chunk.map.id, chunk);
                            } else if let Some((index, map_set)) = ms {
                                map_set_manager.map_sets.insert(index, map_set);
                            }
                        }
                    }
                }
            }
        }
        
    }

    println!("Finished loading maps!");

    let manager = WorldMapManager {
        chunk_map,
        map_set_manager,
        ..Default::default()
    };

    (
        manager,
        textures,
    )

}

fn load_map(
    root_path: &PathBuf, 
    file: &PathBuf
) -> (
    Option<WorldChunk>, 
    Option<(MapIdentifier, WorldMapSet)>
) 
    {
    
    println!("Loading map under: {:?}", root_path);
    
    let data = std::fs::read_to_string(file).unwrap_or_else(|err| panic!("Could not read map configuration file at {:?} to string with error {}", file, err));
    
    match ron::from_str(&data) {
        Ok(serialized_chunk) => {
            (
                Some(
                    chunk::new_chunk_map(root_path, serialized_chunk)
                ), 
                None
            )
        }
        Err(chunk_err) => {
            match ron::from_str(&data) {
                Ok(serialized_map_set) => {
                    (
                        None, 
                        Some(
                            set::load_map_set(root_path, serialized_map_set)
                        )
                    )
                }
                Err(set_err) => {
                    panic!(
                        "Map config at {:?} does not contain either a jigsaw map or a warp map. 
                        Chunk map error: {}, 
                        Map set error: {}\n"
                        , &root_path, chunk_err, set_err);
                }
            }
        }
    }
}

pub fn load_map_from_config<P: AsRef<Path>>(root_path: P, config: MapConfig) -> WorldMap {
    let root_path = root_path.as_ref();
    let gba_map = get_gba_map(
        std::fs::read(root_path.join(config.file)).unwrap_or_else(|err| panic!("Could not get map file at {:?} with error {}", root_path, err))
    );

    WorldMap {

        id: config.identifier,

        name: config.name,
        music: gba_map.music,

        width: gba_map.width,
        height: gba_map.height,

        palettes: gba_map.palettes,

        tiles: gba_map.tiles,
        movements: gba_map.movements,
        // border: Border {
        border: gba_map.borders,
            // size: (gba_map.borders.len() as f32).sqrt() as u8,
        // },
        warps: super::warp::load_warp_entries(root_path.join("warps")),
        wild: super::wild::load_wild_entries(root_path.join("wild")),
        npcs: super::npc::load_npc_entries(root_path.join("npcs")),
        scripts: super::script::load_script_entries(root_path.join("scripts")),
        // state: WorldMapState::default(),
    }
}