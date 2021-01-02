use std::ffi::OsString;
use std::fs::ReadDir;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::path::Path;

use crate::entity::util::direction::Direction;
use crate::game::npc::npc::NPC;

use crate::util::file_util::UNKNOWN_FILENAME_ERR;

use super::npc_serializable::JsonNPC;

pub fn get_npcs<P>(root_path: P, map_set_num: Option<usize>) -> Vec<NPC> where P: AsRef<Path> {
    let root_path = root_path.as_ref();
    let npc_path = root_path.join("npcs");

    let mut npcs = Vec::new();

    match read_dir(&npc_path) {
        Ok(dir) => {
            match map_set_num {
                Some(map_set_num) => {
                    let mut map_set = String::from("map_");
                    map_set.push_str(map_set_num.to_string().as_str());
                    match read_dir(&npc_path.join(map_set)) {
                        Ok(dir) => {
                            if let Some(err) = get_npc_from_directory(&mut npcs, dir) {
                                println!(
                                    "Error fetching npc under {:?} with error: {}",
                                    root_path, err
                                );
                            }
                        }
                        Err(err) => {
                            println!("Error reading map set directory #{} under path {:?} with error {}", map_set_num, root_path, err);
                        }
                    }                    
                },
                None => {
                    if let Some(err) = get_npc_from_directory(&mut npcs, dir) {
                        println!(
                            "Error fetching npc under {:?} with error: {}",
                            root_path, err
                        );
                    }
                }
            }
        }

        Err(err) => {
            println!(
                "Could not read NPC directory for map {:?} with error {}",
                root_path
                    .file_name()
                    .unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)),
                err
            );
        }
    }
    npcs
}

pub fn get_npc_from_directory(npcs: &mut Vec<NPC>, dir: ReadDir) -> Option<std::io::Error> {
    for path_result in dir.map(|res| res.map(|e| e.path())) {
        match path_result {
            Ok(path) => {
                if let Some(npc_entry) = load_npc(path) {
                    npcs.push(npc_entry);
                }
            }
            Err(err) => {
                return Some(err);
            }
        }
    }
    return None;
}

pub fn load_npc<P>(path: P) -> Option<NPC> where P: AsRef<Path> {
    let path = path.as_ref();

    let string_result = read_to_string(path);

    match string_result {
        Ok(string) => {

            let npc_entry: Result<JsonNPC, serde_json::Error> = serde_json::from_str(string.as_str());

            match npc_entry {
                Ok(npc) => {
                    let party;
                    if let Some(pkmn_party) = npc.trainer {
                        party = Some(pkmn_party.pokemon);
                    } else {
                        party = None;
                    }
                    return Some(NPC {
                        x: npc.location.x,
                        y: npc.location.y,
                        direction: Direction::from_int(npc.location.direction).unwrap_or(Direction::Down),
                        sprite: npc.identifier.sprite,
                        pokemon: party,
                    });
                },
                Err(err) => {
                    println!("Could not parse NPC json at {:?} with error {}", path.file_name().unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)), err);
                    return None;
                }
            }

        },
        Err(err) => {
            println!("Could not get NPC json at {:?} with error {}", path, err);
            return None;
        }
    }


}