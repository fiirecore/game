use std::{path::Path, fmt::Display, collections::HashMap};

use serde::Deserialize;
use worldlib::{
    map::{
        chunk::{ChunkConnections, WorldChunk},
        manager::{Maps, WorldMapManager},
        WorldMap,
    },
    serialized::SerializedTextures, positions::Location,
};

use crate::gba_map::{GbaMap, GbaMapError};
use crate::world::textures::get_textures;

use super::MapConfig;

pub mod list;

pub fn load_world(root_path: &Path) -> (WorldMapManager, SerializedTextures) {
    let maps_path = root_path.join("maps");
    let textures_path = root_path.join("textures");

    // let constants = get_constants(root_path);

    let textures = get_textures(textures_path);
    println!(
        "Loaded {} palettes and {} animated textures",
        textures.palettes.len(),
        textures.animated.len()
    );

    let mut world_maps = Maps::default();
    // let mut map_gui_locs = worldlib::serialized::MapGuiLocs::new();

    println!("Loading maps...");

    for worlds in std::fs::read_dir(&maps_path).unwrap_or_else(|err| {
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

        recurse_dir(&worlds, &mut world_maps);
        
        fn recurse_dir(worlds: &Path, world_maps: &mut HashMap<Location, WorldMap>) {
            if let Ok(dir) = std::fs::read_dir(&worlds) {
                let mut count = 0;
                for entry in dir {
                    if let Ok(entry) = entry {
                        let file = entry.path();
                        if file.is_file() {
                            count += 1;
                        }
                        if let Some(ext) = file.extension() {
                            if ext == std::ffi::OsString::from("ron") {
                                match load_maps(&worlds, &file) {
                                    Ok(map) => {
                                        world_maps.insert(map.id, map);
                                    }
                                    Err(err) => eprintln!("Error loading map under {:?}: {}", worlds, err),
                                }
    
                                // if let Some(map_gui_loc) = map_gui_loc {
                                //     map_gui_locs.insert(map_gui_loc.0, (map_gui_loc.1, map_gui_loc.2));
                                // }
                            }
                        }
                    }
                }
                if count == 0 {
                    for entry in std::fs::read_dir(&worlds).unwrap().flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            recurse_dir(&path, world_maps)
                        }
                    }
                }
            }
        }

    }

    println!("Finished loading maps!");

    (world_maps.into(), textures)
}

#[derive(Deserialize)]
#[serde(transparent)]
struct SerializedMap {
    // #[serde(with = "either::serde_untagged")]
    // inner: Either<MapConfig, SerializedMapList>,
    inner: MapConfig,
}

// pub(crate) type MapGuiPos = Option<(worldlib::map::MapIcon, String, Location)>;

fn load_maps(root_path: &Path, file: &Path) -> Result<WorldMap, LoadMapError> {
    println!("Loading map under: {:?}", root_path);

    let extension = file
        .extension()
        .map(|str| {
            str.to_str()
                .unwrap_or_else(|| panic!("Could not read file extension of file at {:?}", file))
        })
        .unwrap_or_else(|| panic!("Error: could not get file extension for file at {:?}", file));

    let data = std::fs::read_to_string(file).unwrap_or_else(|err| {
        panic!(
            "Could not read map configuration file at {:?} to string with error {}",
            file, err
        )
    });

    fn load<'de, E: Into<LoadMapError>>(
        func: impl FnOnce(&'de str) -> Result<MapConfig, E>,
        data: &'de str,
        root_path: &Path,
    ) -> Result<WorldMap, LoadMapError> {
        match (func)(data).map_err(Into::into) {
            Ok(config) => Ok(load_map_from_config(root_path, config)?),
            // Ok(config) => Ok(vec![load_map_from_config(root_path, config.inner, None)?]),
            Err(err) => Err(err),
        }
    }

    match extension {
        "ron" => load(ron::from_str, &data, root_path),
        "toml" => load(toml::from_str, &data, root_path),
        unknown => panic!(
            "Could not read unknown map config/map list with extension {}. File at {:?}",
            unknown, file
        ),
    }
}

pub fn load_map_from_config<P: AsRef<Path>>(
    root_path: P,
    config: MapConfig,
) -> Result<WorldMap, LoadMapError> {
    println!("    Loading map named {}", config.name);

    let root_path = root_path.as_ref();

    let id = config.identifier.into();

    // let map_gui_pos = config.chunk.as_ref().map(|chunk| chunk.map_icon.map(|i| (i, config.name.clone(), loc))).flatten();

    let gba_map = GbaMap::load(
        std::fs::read(root_path.join(config.file)).unwrap_or_else(|err| {
            panic!(
                "Could not get map file at {:?} with error {}",
                root_path, err
            )
        }),
    );

    let chunk: ChunkConnections = config
        .chunk
        .into_iter()
        .map(|(d, c)| (d, c.into()))
        .collect();

    let chunk = (!chunk.is_empty()).then(|| WorldChunk { connections: chunk });

    Ok(WorldMap {
        id,

        name: config.name,
        music: GbaMap::map_music(gba_map.music)?,

        width: gba_map.width as _,
        height: gba_map.height as _,

        palettes: gba_map.palettes,

        tiles: gba_map.tiles,
        movements: gba_map.movements,

        border: gba_map.borders,

        chunk,

        warps: super::warp::load_warp_entries(root_path.join("warps")),
        wild: super::wild::load_wild_entries(root_path.join("wild")),
        npcs: super::npc::load_npc_entries(root_path.join("npcs")),
        scripts: super::script::load_script_entries(root_path.join("scripts")),

        settings: config.settings,
    })
}

#[derive(Debug)]
pub enum LoadMapError {
    GbaMap(GbaMapError),
    Io(std::io::Error),
    Ron(ron::Error),
    Toml(toml::de::Error),
}

impl Display for LoadMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadMapError::GbaMap(err) => Display::fmt(err, f),
            LoadMapError::Io(err) => Display::fmt(err, f),
            LoadMapError::Ron(err) => Display::fmt(err, f),
            LoadMapError::Toml(err) => Display::fmt(err, f),
            // panic!(
            //     "Map config at {:?} does not contain either a MapConfig or a map list.
            //         MapConfig error: {},
            //         Map list error: {}\n",
            //     &root_path, chunk_err, set_err
            // );
        }
    }
}

impl From<GbaMapError> for LoadMapError {
    fn from(err: GbaMapError) -> Self {
        Self::GbaMap(err)
    }
}

impl From<std::io::Error> for LoadMapError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ron::Error> for LoadMapError {
    fn from(err: ron::Error) -> Self {
        Self::Ron(err)
    }
}

impl From<toml::de::Error> for LoadMapError {
    fn from(err: toml::de::Error) -> Self {
        Self::Toml(err)
    }
}
