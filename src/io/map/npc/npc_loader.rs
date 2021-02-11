use std::path::PathBuf;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use crate::world::npc::NPC;

pub fn load_npc_entries(root_path: &PathBuf, map_index: Option<usize>) -> Vec<NPC> {

    let mut npcs = Vec::new();

    let npc_dir = root_path.join("npcs");

    match map_index {
        Some(map_index) => {
            get_npc_from_directory(&mut npcs, npc_dir.join(String::from("map_") + &map_index.to_string()));                  
        },
        None => {
            get_npc_from_directory(&mut npcs, npc_dir);
        }
    }

    return npcs;
}

pub fn get_npc_from_directory(npcs: &mut Vec<NPC>, dir: PathBuf) {
    for filepath in crate::io::get_dir(dir) {
        if let Some(npc_entry) = load_npc(filepath) {
            info!("Loaded NPC {}", &npc_entry.identifier.name);
            npcs.push(npc_entry);
        }
    }
}

pub fn load_npc(file: PathBuf) -> Option<NPC> {

    match crate::io::get_file_as_string(&file) {
        Some(data) => {

            let npc_entry: Result<NPC, serde_json::Error> = serde_json::from_str(&data);

            match npc_entry {
                Ok(npc) => {
                    return Some(npc);
                },
                Err(err) => {
                    warn!("Could not parse NPC json at {:?} with error {}", &file, err);
                    return None;
                }
            }

        },
        None => {
            warn!("Could not get NPC json at {:?}", &file);
            return None;
        }
    }


}