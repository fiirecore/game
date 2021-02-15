use std::path::PathBuf;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use ahash::AHashMap as HashMap;
use crate::world::map::chunk::world_chunk::WorldChunk;
use crate::world::map::chunk::world_chunk_map::WorldChunkMap;
use crate::world::map::set::world_map_set::WorldMapSet;
use crate::world::map::set::world_map_set_manager::WorldMapSetManager;
use crate::world::npc::NPC;
use crate::world::pokemon::WildEntry;
use crate::world::warp::WarpEntry;

pub mod chunk_map_loader;
pub mod map_set_loader;

pub mod map_serializable;
pub mod gba_map;

pub fn load_maps_v1(chunk_map: &mut WorldChunkMap, map_sets: &mut WorldMapSetManager, bottom_textures: &mut HashMap<u16, crate::util::graphics::Texture>, npc_textures: &mut crate::world::NpcTextures) {
    let mut bottom_sheets: HashMap<u8, macroquad::prelude::Image> = HashMap::new();
    let palette_sizes = gba_map::fill_palette_map(&mut bottom_sheets);

    info!("Loading maps...");
    for dir_entry in crate::io::get_dir("world/maps/") {
        for file in crate::io::get_dir(&dir_entry) {
            if let Some(ext) = file.extension() {
                if ext == std::ffi::OsString::from("toml") {
                    let maps = load_map(&palette_sizes, &dir_entry, &file);
                    if let Some(world_chunk) = maps.0 {
                        // super::map_loader::create_file_test(&dir_entry, &world_chunk.1);
                        chunk_map.insert(world_chunk.0, world_chunk.1);
                    } else if let Some(map_set) = maps.1 {
                        map_sets.insert(map_set.0, map_set.1);
                    }
                }
            }
        }
    }
    info!("Finished loading maps!");

    info!("Loading textures...");
    for wmap in chunk_map.chunks.values() {
        for tile_id in &wmap.map.tile_map {
            if !(bottom_textures.contains_key(tile_id) ){// && self.top_textures.contains_key(tile_id)) {
                //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                bottom_textures.insert(*tile_id, gba_map::get_texture(&bottom_sheets, &palette_sizes, *tile_id));
            }
        }
        for tile_id in &wmap.map.border_blocks {
            if !(bottom_textures.contains_key(tile_id) ){// && self.top_textures.contains_key(tile_id)) {
                bottom_textures.insert(*tile_id, gba_map::get_texture(&bottom_sheets, &palette_sizes, *tile_id));
                //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
            }
        }
    }
    for wmapset in map_sets.values() {
        for tile_id in &wmapset.tiles() {
            if !(bottom_textures.contains_key(tile_id) ){// && self.top_textures.contains_key(tile_id)) {
                //self.top_textures.insert(*tile_id, get_texture(&top_sheets, &palette_sizes, *tile_id));
                bottom_textures.insert(*tile_id, gba_map::get_texture(&bottom_sheets, &palette_sizes, *tile_id));
            }
        }
    }

    super::npc_texture::load_npc_textures(npc_textures);
    info!("Finished loading textures!");

}

fn load_map(palette_sizes: &HashMap<u8, u16>, root_path: &PathBuf, file: &PathBuf) -> (Option<(u16, WorldChunk)>, Option<(String, WorldMapSet)>) {

    match crate::io::get_file_as_string(file) {
        Some(data) => {
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
        None => {
            warn!("Error reading file at {:?} to string with error", &root_path);
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
    for filepath in crate::io::get_dir(dir) {
        match crate::io::get_file_as_string(&filepath) {
            Some(data) => {
                let npc_result: Result<NPC, serde_json::Error> = serde_json::from_str(&data);
                match npc_result {
                    Ok(npc) => {
                        macroquad::prelude::debug!("Loaded NPC {}", &npc.identifier.name);
                        npcs.push(npc);
                    },
                    Err(err) => warn!("Could not parse NPC json at {:?} with error {}", &filepath, err),
                }
            },
            None => warn!("Could not get NPC json at {:?}", &filepath),
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

pub fn load_wild_entry(root_path: &PathBuf, wild: Option<map_serializable::SerializedWildEntry>, map_set_index: Option<usize>) -> Option<WildEntry> {
    if let Some(toml_wild_entry) = wild {
        let mut wild_path = root_path.join("wild");

        if let Some(map_set_index) = map_set_index {
            wild_path = wild_path.join(String::from("map_") + &map_set_index.to_string());
        }

        Some(WildEntry {
            tiles: toml_wild_entry.wild_tiles,
            table: crate::world::pokemon::wild_pokemon_table::get(toml_wild_entry.encounter_type.as_str(), wild_path.join("grass.toml")),
        })

    } else {
        return None;
    }   
}