use std::path::{Path, PathBuf};
use util::Location;
use worldlib::{
    map::{
        manager::{Maps, WorldMapManager},
        WorldMap,
    },
    serialized::SerializedTextures,
};

use crate::gba_map::get_gba_map;
use crate::world::textures::get_textures;

use super::MapConfig;

pub mod chunk;
pub mod set;

pub fn load_maps<P: AsRef<Path>>(maps: P, textures: P) -> (WorldMapManager, SerializedTextures) {
    let maps_path = maps.as_ref();
    let textures_path = textures.as_ref();

    let mut maps = Maps::default();
    let textures = get_textures(textures_path);
    println!(
        "Loaded {} palettes and {} animated textures",
        textures.palettes.len(),
        textures.animated.len()
    );

    println!("Loading maps...");

    for worlds in std::fs::read_dir(maps_path).unwrap_or_else(|err| {
        panic!(
            "Could not read directory at {:?} with error {}",
            maps_path, err
        )
    }) {
        let worlds = worlds
            .unwrap_or_else(|err| {
                panic!(
                    "Could not get directory entry under {:?} with error {}",
                    maps_path, err
                )
            })
            .path();
        if let Ok(dir) = std::fs::read_dir(&worlds) {
            for entry in dir {
                if let Ok(entry) = entry {
                    let file = entry.path();
                    if let Some(ext) = file.extension() {
                        if ext == std::ffi::OsString::from("ron") {
                            maps.extend(load_map(&worlds, &file));
                        }
                    }
                }
            }
        }
    }

    println!("Finished loading maps!");

    let manager = WorldMapManager {
        maps,
        ..Default::default()
    };

    (manager, textures)
}

fn load_map(root_path: &PathBuf, file: &PathBuf) -> Vec<(Location, WorldMap)> {
    println!("Loading map under: {:?}", root_path);

    let data = std::fs::read_to_string(file).unwrap_or_else(|err| {
        panic!(
            "Could not read map configuration file at {:?} to string with error {}",
            file, err
        )
    });

    match ron::from_str(&data) {
        Ok(serialized_chunk) => {
            let chunk = chunk::new_chunk_map(root_path, serialized_chunk);
            vec![(Location::new(None, chunk.id), chunk)]
        }
        Err(chunk_err) => match ron::from_str(&data) {
            Ok(serialized_map_set) => set::load_map_set(root_path, serialized_map_set),
            Err(set_err) => {
                panic!(
                    "Map config at {:?} does not contain either a jigsaw map or a warp map. 
                        Chunk map error: {}, 
                        Map set error: {}\n",
                    &root_path, chunk_err, set_err
                );
            }
        },
    }
}

pub fn load_map_from_config<P: AsRef<Path>>(root_path: P, config: MapConfig) -> WorldMap {
    let root_path = root_path.as_ref();
    let gba_map = get_gba_map(
        std::fs::read(root_path.join(config.file)).unwrap_or_else(|err| {
            panic!(
                "Could not get map file at {:?} with error {}",
                root_path, err
            )
        }),
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

        chunk: config.chunk,
        // size: (gba_map.borders.len() as f32).sqrt() as u8,
        // },
        warps: super::warp::load_warp_entries(root_path.join("warps")),
        wild: super::wild::load_wild_entries(root_path.join("wild")),
        npcs: super::npc::load_npc_entries(root_path.join("npcs")),
        scripts: super::script::load_script_entries(root_path.join("scripts")),
        // state: WorldMapState::default(),
    }
}
