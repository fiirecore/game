use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::ReadDir;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::path::Path;
use std::path::PathBuf;

use log::warn;

use crate::entity::texture::still_texture_manager::StillTextureManager;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::entity::util::direction::Direction;
use crate::game::npc::npc::NPCInstance;

use crate::io::data::npc::npc::JsonNPC;
use crate::util::file_util::UNKNOWN_FILENAME_ERR;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::texture_util::texture_from_path;

pub fn load_npc_textures(world_id: &String, npc_textures: &mut HashMap<u8, ThreeWayTexture>) {
    let mut dir_pb = PathBuf::from("worlds/");
    dir_pb.push(world_id);
    dir_pb.push("textures");
    dir_pb.push("npcs");

    //println!("{:?}", dir_pb.clone());

    let entries_result = std::fs::read_dir(asset_as_pathbuf(dir_pb));
    match entries_result {
        Ok(readdir) => {
            let paths: Vec<Result<PathBuf, std::io::Error>> = readdir.map( |res| res.map(|e| e.path())).collect();
            let size = paths.len();
            for path in paths {
                match path {
                    Ok(path) => {
                        if path.is_dir() {
                            let mut twt = ThreeWayTexture::new();
                            if size > 3 {
                                warn!("Moving NPC textures found, not supported yet.");
                            } else {
                                twt.add_texture_manager(Box::new(StillTextureManager::new(texture_from_path(&path.join("idle_up.png")), false)));
                                twt.add_texture_manager(Box::new(StillTextureManager::new(texture_from_path(&path.join("idle_down.png")), false)));
                                twt.add_texture_manager(Box::new(StillTextureManager::new(texture_from_path(&path.join("idle_side.png")), false)));
                            }                                    
                            npc_textures.insert(path.file_name().unwrap().to_str().unwrap().parse::<u8>().expect("Found a folder with a non-number name"), twt); // fix
                        }
                    },
                    Err(err) => {
                        warn!("{}", err);
                    }
                }
            }
        },
        Err(err) => {
            warn!("Error reading NPC textures directory for map {} with error: {}", world_id, err);
        },
    }
}

pub fn get_npcs<P>(root_path: P, map_set_num: Option<usize>) -> Vec<NPCInstance> where P: AsRef<Path> {
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
                                warn!(
                                    "Error fetching npc under {:?} with error: {}",
                                    root_path, err
                                );
                            }
                        }
                        Err(err) => {
                            warn!("Error reading map set directory #{} under path {:?} with error {}", map_set_num, root_path, err);
                        }
                    }                    
                },
                None => {
                    if let Some(err) = get_npc_from_directory(&mut npcs, dir) {
                        warn!(
                            "Error fetching npc under {:?} with error: {}",
                            root_path, err
                        );
                    }
                }
            }
        }

        Err(err) => {
            warn!(
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

pub fn get_npc_from_directory(npcs: &mut Vec<NPCInstance>, dir: ReadDir) -> Option<std::io::Error> {
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

pub fn load_npc<P>(path: P) -> Option<NPCInstance> where P: AsRef<Path> {
    let path = path.as_ref();

    let string_result = read_to_string(path);

    match string_result {
        Ok(string) => {

            let npc_entry: Result<JsonNPC, serde_json::Error> = serde_json::from_str(string.as_str());

            match npc_entry {
                Ok(npc) => {
                    return Some(NPCInstance {

                        x: npc.location.x,
                        y: npc.location.y,

                        direction: Direction::from_int(npc.location.direction).unwrap_or(Direction::Down),

                        name: npc.identifier.name,
                        sprite: npc.identifier.sprite,

                        trainer: npc.trainer,
                    });
                },
                Err(err) => {
                    warn!("Could not parse NPC json at {:?} with error {}", path.file_name().unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)), err);
                    return None;
                }
            }

        },
        Err(err) => {
            warn!("Could not get NPC json at {:?} with error {}", path, err);
            return None;
        }
    }


}