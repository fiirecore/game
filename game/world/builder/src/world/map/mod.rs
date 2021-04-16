use std::path::{Path, PathBuf};

use ahash::AHashMap as HashMap;

use firecore_world_lib::{
    serialized::Palette,
    map::{
        WorldMap,
        MapIdentifier,
        Border,
        manager::WorldMapManager,
        chunk::{
            WorldChunk,
            map::WorldChunkMap,
        },
        set::{
            WorldMapSet,
            manager::WorldMapSetManager,
        }
    }
};

use crate::gba_map::{get_gba_map, fix_tiles, fill_palette_map};

use super::MapConfig;

pub mod chunk;
pub mod set;

pub fn load_maps<P: AsRef<Path>>(maps: P, tile_textures: P) -> (WorldMapManager, Vec<Palette>) {

    let maps = maps.as_ref();
    let tile_textures = tile_textures.as_ref();

    let mut chunk_map = WorldChunkMap::default();
    let mut map_set_manager = WorldMapSetManager::default();
    let (palette_sizes, palettes) = fill_palette_map(tile_textures);
    println!("Loaded {} palettes", palette_sizes.len());

    println!("Loading maps...");

    for worlds in std::fs::read_dir(maps).unwrap_or_else(|err| panic!("Could not read directory at {:?} with error {}", maps, err)) {
        let worlds = worlds.unwrap_or_else(|err| panic!("Could not get directory entry under {:?} with error {}", maps, err)).path();
        if let Ok(dir) = std::fs::read_dir(&worlds) {
            for entry in dir {
                if let Ok(entry) = entry {
                    let file = entry.path();
                    if let Some(ext) = file.extension() {
                        if ext == std::ffi::OsString::from("ron") {
                            let (cm, ms) = load_map(&palette_sizes, &worlds, &file);
                            if let Some((index, chunk)) = cm {
                                chunk_map.chunks.insert(index, chunk);
                            } else if let Some((index, map_set)) = ms {
                                map_set_manager.map_sets.insert(index, map_set);
                            }
                        }
                    }
                }
            }
        }
        
    }

    let palettes = palettes.into_iter().map(
        |(id, bottom)|
        Palette {
            id,
            bottom,
        }
    ).collect();

    println!("Finished loading maps!");

    let manager = WorldMapManager {
        chunk_map,
        map_set_manager,
        ..Default::default()
    };

    (
        manager,
        palettes
    )

}

fn load_map(
    palette_sizes: &HashMap<u8, u16>, 
    root_path: &PathBuf, 
    file: &PathBuf
) -> (
    Option<(MapIdentifier, WorldChunk)>, 
    Option<(MapIdentifier, WorldMapSet)>
) 
    {
    
    println!("Loading map under: {:?}", root_path);
    
    let data = std::fs::read_to_string(file).unwrap_or_else(|err| panic!("Could not read map configuration file at {:?} to string with error {}", file, err));
    
    match ron::from_str(&data) {
        Ok(serialized_chunk) => {
            (
                Some(
                    chunk::new_chunk_map(root_path, palette_sizes, serialized_chunk)
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
                            set::load_map_set(root_path, palette_sizes, serialized_map_set)
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

pub fn load_map_from_config<P: AsRef<Path>>(root_path: P, palette_sizes: &HashMap<u8, u16>, config: MapConfig) -> (MapIdentifier, WorldMap) {
    let root_path = root_path.as_ref();
    // println!("Loading map: \"{}\"", map_config.name);
    let mut gba_map = get_gba_map(
        std::fs::read(root_path.join(config.file)).unwrap_or_else(|err| panic!("Could not get map file at {:?} with error {}", root_path, err))
    );
    fix_tiles(&mut gba_map, palette_sizes);

    (
        config.identifier,
        WorldMap {
            name: config.name,
            music: gba_map.music,
            width: gba_map.width,
            height: gba_map.height,
            tiles: gba_map.tiles,
            movements: gba_map.movements,
            border: Border {
                tiles: gba_map.borders.into(),
                size: (gba_map.borders.len() as f32).sqrt() as u8,
            },
            warps: super::warp::load_warp_entries(root_path.join("warps")),
            wild: super::wild::load_wild_entry(config.wild, root_path.join("wild")),
            npc_manager: super::npc::load_npc_entries(root_path.join("npcs")),
            scripts: super::script::load_script_entries(root_path.join("scripts")),
        }
    )
}