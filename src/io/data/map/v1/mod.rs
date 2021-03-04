use std::path::PathBuf;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use ahash::AHashMap as HashMap;
use crate::world::map::chunk::WorldChunk;
use crate::world::map::chunk::world_chunk_map::WorldChunkMap;
use crate::world::npc::manager::MapNpcManager;
use crate::world::map::set::WorldMapSet;
use crate::world::map::set::manager::WorldMapSetManager;
use crate::world::npc::NPC;
use crate::world::pokemon::WildEntry;
// use crate::world::map::script_manager::MapScriptManager;
// use crate::world::script::npc::NPCScript;
use crate::world::warp::WarpEntry;

pub mod chunk_map_loader;
pub mod map_set_loader;

pub mod map_serializable;
pub mod gba_map;

pub fn load_maps_v1(chunk_map: &mut WorldChunkMap, map_sets: &mut WorldMapSetManager, bottom_textures: &mut crate::world::TileTextures, npc_textures: &mut crate::world::NpcTextures) {
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
            chunk_map.insert(world_chunk.0, world_chunk.1);
        } else if let Some(map_set) = maps.1 {
            map_sets.insert(map_set.0, map_set.1);
        }
    }    

    info!("Finished loading maps!");

    info!("Loading textures...");
    for tile_id in chunk_map.tiles() {
        if !(bottom_textures.tile_textures.contains_key(&tile_id) ){// && self.top_textures.contains_key(tile_id)) {
            //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
            bottom_textures.tile_textures.insert(tile_id, gba_map::get_texture(&bottom_sheets, &palette_sizes, tile_id));
        }
    }
    for wmapset in map_sets.values() {
        for tile_id in &wmapset.tiles() {
            if !(bottom_textures.tile_textures.contains_key(tile_id) ){// && self.top_textures.contains_key(tile_id)) {
                //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                bottom_textures.tile_textures.insert(*tile_id, gba_map::get_texture(&bottom_sheets, &palette_sizes, *tile_id));
            }
        }
    }

    super::npc_texture::load_npc_textures(npc_textures);
    info!("Finished loading textures!");

}

fn load_map(palette_sizes: &HashMap<u8, u16>, root_path: &PathBuf, file: &PathBuf) -> (Option<(u16, WorldChunk)>, Option<(String, WorldMapSet)>) {

    match crate::io::get_file_as_string(file) {
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

pub fn load_npc_entries(root_path: &PathBuf, map_index: Option<usize>) -> MapNpcManager {
    let mut npcs = Vec::new();
    let npc_dir = root_path.join("npcs");
    match map_index {
        Some(map_index) => get_npc_from_directory(&mut npcs, npc_dir.join(String::from("map_") + &map_index.to_string())),
        None => get_npc_from_directory(&mut npcs, npc_dir),
    }
    return MapNpcManager {
        npcs,
        npc_active: None,
    };
}

fn get_npc_from_directory(npcs: &mut Vec<NPC>, dir: PathBuf) {
    for filepath in crate::io::get_dir(dir) {
        match crate::io::get_file_as_string(&filepath) {
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
    for file in crate::io::get_dir(dir) {
        if let Some(warp_entry) = WarpEntry::new(file) {
            warps.push(warp_entry);
        }
    }
}

pub fn load_wild_entry(root_path: &PathBuf, wild: Option<map_serializable::SerializedWildEntry>, map_index: Option<usize>) -> Option<WildEntry> {
    wild.map(|toml_wild_entry| {
        let mut wild_path = root_path.join("wild");

        if let Some(map_index) = map_index {
            wild_path = wild_path.join(String::from("map_") + &map_index.to_string());
        }

        WildEntry {
            tiles: toml_wild_entry.wild_tiles,
            table: crate::world::pokemon::wild_pokemon_table::get_table(toml_wild_entry.encounter_type.as_str(), wild_path.join("grass.toml")),
        }

    })
}

/*

pub fn load_script_entries(root_path: &PathBuf, map_index: Option<usize>) -> MapScriptManager {
    let mut npc_scripts = Vec::new();
    let script_root = root_path.join("scripts");


    let mut npc_script_path = script_root.join("npcs");

    if let Some(map_index) = map_index {
        npc_script_path = npc_script_path.join(String::from("map_") + &map_index.to_string());
    }

    for file in crate::io::get_dir(npc_script_path) {
        if let Some(npc_script) = NPCScript::new(file) {
            npc_scripts.push(npc_script);
        }
    }

    MapScriptManager::new(npc_scripts)
}

*/