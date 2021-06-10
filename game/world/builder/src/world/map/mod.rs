use std::path::Path;
use util::{Location, LocationId};
use worldlib::{
    map::{
        manager::{Maps, WorldMapManager, WorldMapManagerData},
        WorldMap,
    },
    serialized::SerializedTextures,
};

use crate::gba_map::get_gba_map;
use crate::world::{
    // constants::get_constants, 
    textures::get_textures
};

use super::MapConfig;

pub mod list;

pub fn load_maps(root_path: &Path) -> (WorldMapManager, SerializedTextures, worldlib::serialized::MapGuiLocs) {
    let maps_path = root_path.join("maps");
    let textures_path = root_path.join("textures");

    // let constants = get_constants(root_path);

    let textures = get_textures(textures_path);
    println!(
        "Loaded {} palettes and {} animated textures",
        textures.palettes.len(),
        textures.animated.len()
    );

    let mut maps = Maps::default();
    let mut map_gui_locs = worldlib::serialized::MapGuiLocs::new();

    println!("Loading maps...");

    for worlds in std::fs::read_dir(&maps_path).unwrap_or_else(|err| panic!("Could not read directory at {:?} with error {}", maps_path, err)) {
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
                            let (map, map_gui_loc) = load_map(&worlds, &file);
                            let map = map.into_iter().map(|map| (map.id, map));
                            maps.extend(map);
                            if let Some(map_gui_loc) = map_gui_loc {
                                map_gui_locs.insert(map_gui_loc.0, (map_gui_loc.1, map_gui_loc.2));
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Finished loading maps!");

    let manager = WorldMapManager {
        maps,
        data: WorldMapManagerData::default()
    };

    (manager, textures, map_gui_locs)
}

pub(crate) type MapGuiPos = Option<(firecore_dependencies::tetra::math::Vec2<u8>, String, Location)>;

fn load_map(root_path: &Path, file: &Path) -> (Vec<WorldMap>, MapGuiPos) {
    println!("Loading map under: {:?}", root_path);

    let data = std::fs::read_to_string(file).unwrap_or_else(|err| {
        panic!(
            "Could not read map configuration file at {:?} to string with error {}",
            file, err
        )
    });

    match ron::from_str(&data) {
        Ok(config) => load_map_from_config(root_path, config, None),
        Err(chunk_err) => match ron::from_str(&data) {
            Ok(list) => list::load_map_list(root_path, list),
            Err(set_err) => {
                panic!(
                    "Map config at {:?} does not contain either a MapConfig or a map list. 
                        MapConfig error: {}, 
                        Map list error: {}\n",
                    &root_path, chunk_err, set_err
                );
            }
        },
    }
}

pub fn load_map_from_config<P: AsRef<Path>>(root_path: P, config: MapConfig, map: Option<LocationId>) -> (Vec<WorldMap>, MapGuiPos) {
    println!("    Loading map named {}", config.name);

    let root_path = root_path.as_ref();

    let loc = Location::new(map, config.identifier);

    let map_gui_pos = config.chunk.as_ref().map(|chunk| chunk.map_position.map(|pos| (pos, config.name.clone(), loc))).flatten();

    let gba_map = get_gba_map(
        std::fs::read(root_path.join(config.file)).unwrap_or_else(|err| {
            panic!(
                "Could not get map file at {:?} with error {}",
                root_path, err
            )
        }),
    );

    (
        vec![WorldMap {
            id: loc,

            name: config.name,
            music: gba_map.music,

            width: gba_map.width,
            height: gba_map.height,

            palettes: gba_map.palettes,

            tiles: gba_map.tiles,
            movements: gba_map.movements,

            border: gba_map.borders,

            chunk: config.chunk.map(|chunk| chunk.into()),

            warps: super::warp::load_warp_entries(root_path.join("warps")),
            wild: super::wild::load_wild_entries(root_path.join("wild")),
            npcs: super::npc::load_npc_entries(root_path.join("npcs")),
            scripts: super::script::load_script_entries(root_path.join("scripts")),

            fly_position: config.settings.fly_position,

            time: config.settings.time,

            // mart: None,//super::mart::load_mart(root_path.join("mart.ron")),

        }],
        map_gui_pos
    )
}
