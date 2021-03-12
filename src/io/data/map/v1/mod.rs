use std::convert::TryFrom;
use std::path::PathBuf;
use firecore_world::npc::NPC;
use firecore_world::pokemon::WildEntry;
use firecore_world::pokemon::wild_pokemon_table::WildPokemonTable;
use firecore_world::warp::WarpEntry;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use ahash::AHashMap as HashMap;

use firecore_world::script::WorldScript;
use crate::io::get_dir;
use crate::io::get_file_as_string;
use firecore_world::map::WorldMap;
use firecore_world::map::chunk::WorldChunk;
use firecore_world::map::chunk::world_chunk_map::WorldChunkMap;
use firecore_world::map::set::WorldMapSet;
use firecore_world::map::set::manager::WorldMapSetManager;

pub mod chunk_map_loader;
pub mod map_set_loader;

pub mod map_serializable;
pub mod gba_map;

pub fn load_maps_v1(chunk_map: &mut WorldChunkMap, map_set_manager: &mut WorldMapSetManager, tile_textures: &mut crate::world::TileTextures, npc_textures: &mut crate::world::NpcTextures) {
    let mut bottom_sheets: HashMap<u8, macroquad::prelude::Image> = HashMap::new();
    let palette_sizes = gba_map::fill_palette_map(&mut bottom_sheets);

    info!("Loading maps...");
    let maps: Vec<(Option<(u16, WorldChunk)>, Option<(String, WorldMapSet)>)> = crate::io::get_dir("world/maps/").iter().map(|dir_entry | {
        for file in crate::io::get_dir(dir_entry) {
            if let Some(ext) = file.extension() {
                if ext == std::ffi::OsString::from("toml") {
                    return load_map(&palette_sizes, dir_entry, &file);
                }
            }
        }
        return (None, None);
    }).collect();

    for maps in maps {
        if let Some(world_chunk) = maps.0 {
            // super::map_loader::create_file_test(&dir_entry, &world_chunk.1);
            chunk_map.chunks.insert(world_chunk.0, world_chunk.1);
        } else if let Some(map_set) = maps.1 {
            map_set_manager.map_sets.insert(map_set.0, map_set.1);
        }
    }    

    info!("Finished loading maps!");

    info!("Loading textures...");
    for tile_id in chunk_map.tiles() {
        if !(tile_textures.tile_textures.contains_key(&tile_id) ){// && self.top_textures.contains_key(tile_id)) {
            //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
            tile_textures.tile_textures.insert(tile_id, gba_map::get_texture(&bottom_sheets, &palette_sizes, tile_id));
        }
    }
    for tile_id in map_set_manager.tiles() {
        if !(tile_textures.tile_textures.contains_key(&tile_id) ){// && self.top_textures.contains_key(tile_id)) {
            //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
            tile_textures.tile_textures.insert(tile_id, gba_map::get_texture(&bottom_sheets, &palette_sizes, tile_id));
        }
    }

    super::npc_texture::load_npc_textures(npc_textures);
    info!("Finished loading textures!");

}

pub fn new_world_from_v1(gba_map: gba_map::GbaMap, config: &map_serializable::MapConfig, root_path: &PathBuf, map_index: Option<usize>) -> WorldMap {
    WorldMap {
        name: config.identifier.name.clone(),
        music: firecore_util::music::Music::try_from(gba_map.music).unwrap_or_default(),
        width: gba_map.width,
        height: gba_map.height,
        tile_map: gba_map.tile_map,
        border_blocks: gba_map.border_blocks,
        movement_map: gba_map.movement_map,
        fly_position: config.settings.fly_position,
        wild: load_wild_entry(root_path, config.wild.clone(), map_index),
        warps: load_warp_entries(root_path, map_index),
        npcs: load_npc_entries(root_path, map_index),
        scripts: load_script_entries(root_path, map_index),
        script_npcs: HashMap::new(),
        npc_active: None,
    }
}

fn load_map(palette_sizes: &HashMap<u8, u16>, root_path: &PathBuf, file: &PathBuf) -> (Option<(u16, WorldChunk)>, Option<(String, WorldMapSet)>) {

    match get_file_as_string(file) {
        Ok(data) => {
            match self::map_serializable::MapConfig::from_string(&data) {
                Ok(map_config) => {
                    if map_config.jigsaw_map.is_some() {
                        match self::chunk_map_loader::new_chunk_map(root_path, palette_sizes, map_config) {
                            Some(map) => {
                                return (Some(map), None);
                            }
                            None => {
                                warn!("Error reading jigsaw map at path: {:?}", &root_path);
                                return (None, None);
                            }
                        }
                    } else if map_config.warp_map.is_some() {
                        match self::map_set_loader::new_map_set(root_path, palette_sizes, map_config) {
                            Some(map) => {
                                return (None, Some(map));
                            }
                            None => {
                                warn!("Error reading warp map at path: {:?}", &root_path);
                                return (None, None);
                            }
                        }

                    } else {

                        warn!("Map config at {:?} does not contain either a jigsaw map or a warp map.", &root_path);
                        return (None, None);

                    }
                    
                }
                Err(err) => {
                    warn!(
                        "Toml file at {:?} is {}",
                        &root_path,
                        err
                    );

                    return (None, None);
                }
            }
        }
        Err(err) => {
            warn!("Error reading file at {:?} to string with error {}", &root_path, err);
            return (None, None);
        }
    }
}

pub fn load_npc_entries(root_path: &PathBuf, map_index: Option<usize>) -> Vec<NPC> {
    let mut npcs = Vec::new();
    let npc_dir = root_path.join("npcs");
    match map_index {
        Some(map_index) => get_npc_from_directory(&mut npcs, npc_dir.join(String::from("map_") + &map_index.to_string())),
        None => get_npc_from_directory(&mut npcs, npc_dir),
    }
    return npcs;
}

fn get_npc_from_directory(npcs: &mut Vec<NPC>, dir: PathBuf) {
    for filepath in get_dir(dir) {
        match get_file_as_string(&filepath) {
            Ok(data) => {
                let npc_result: Result<NPC, ron::Error> = ron::from_str(&data);
                match npc_result {
                    Ok(npc) => {
                        macroquad::prelude::debug!("Loaded NPC {}", &npc.identifier.name);
                        npcs.push(npc);
                    },
                    Err(err) => {
                        warn!("Could not parse NPC .ron at {:?} with error {}", &filepath, err);
                    },
                }
            },
            Err(err) => {
                warn!("Could not get NPC json at {:?} with error {}", &filepath, err);
            },
        }
    }
}

pub fn load_warp_entries(root_path: &PathBuf, map_index: Option<usize>) -> Vec<WarpEntry> {
    let mut warps = Vec::new();
    let warp_path = root_path.join("warps");
    match map_index {
        Some(map_index) => add_warp_under_directory(&mut warps, warp_path.join(String::from("map_") + &map_index.to_string())),
        None => add_warp_under_directory(&mut warps, warp_path),
    }
    return warps;
}

fn add_warp_under_directory(warps: &mut Vec<WarpEntry>, dir: PathBuf) {
    for file in get_dir(dir) {
        match crate::io::get_file_as_string(&file) {
            Ok(data) => {
                match toml::from_str(&data) {
                    Ok(warp_entry) => {
                        warps.push(warp_entry);
                    }
                    Err(err) => {
                        warn!("Could not parse warp entry at {:?} with error {}", &file, err);
                    }
                }

            },
            Err(err) => {
                warn!("Could not read warp entry toml at {:?} to string with error {}", &file, err);
            }
        }
    }
}

pub fn load_wild_entry(root_path: &PathBuf, wild: Option<map_serializable::SerializedWildEntry>, map_index: Option<usize>) -> Option<WildEntry> {
    wild.map(|toml_wild_entry| {
        let mut wild_path = root_path.join("wild");

        if let Some(map_index) = map_index {
            wild_path = wild_path.join(String::from("map_") + &map_index.to_string());
        }

        let file = wild_path.join("grass.toml");

        let table = match toml_wild_entry.encounter_type.as_str() {
            "original" => {
                match crate::io::get_file_as_string(&file) {
                    Ok(content) => {
                        match toml::from_str(&content) {
                            Ok(table) => table,
                            Err(err) => {
                                warn!("Could not parse wild pokemon table at {:?} with error {}, using random table instead!", &file, err);
                                WildPokemonTable::default()
                            }
                        }
                    }
                    Err(err) => {
                        warn!("Could not find wild toml file at {:?} with error {}!", file, err);
                        WildPokemonTable::default()
                    }
                }
            }
            _ => {
                WildPokemonTable::default()
            }
        };

        WildEntry {
            tiles: toml_wild_entry.wild_tiles,
            table: table,
        }

    })
}

pub fn load_script_entries(root_path: &PathBuf, map_index: Option<usize>) -> Vec<WorldScript> {
    let mut scripts = Vec::new();
    let mut script_dir = root_path.join("scripts");
    if let Some(index) = map_index {
        script_dir = script_dir.join(format!("map_{}", index));
    }
    for file in get_dir(script_dir) {
        match crate::io::get_file_as_string(&file) {
            Ok(content) => {
                let script: Result<WorldScript, ron::Error> = ron::from_str(&content);
                match script {
                    Ok(script) => scripts.push(script),
                    Err(err) => {
                        warn!("Could not parse script at {:?} with error {}", file, err);
                    }
                }
            },
            Err(err) => {
                warn!("Could not get script entry at {:?} as string with error {}", file, err);
            }
        }
    }
    scripts
}