use macroquad::prelude::info;
use macroquad::prelude::warn;
use crate::entity::texture::still_texture_manager::StillTextureManager;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::world::npc::NPC;

pub fn load_npc_textures(npc_textures: &mut ahash::AHashMap<u8, ThreeWayTexture>) {
    info!("Loading NPC textures...");

    let files = ["idle_up.png", "idle_down.png", "idle_side.png"];

    match crate::io::ASSET_DIR.get_dir("world/textures/npcs") {
        Some(npcs_textures_dir) => {
            for npc_texture_dir in npcs_textures_dir.dirs() {
                match npc_texture_dir.path().file_name().unwrap().to_string_lossy().parse::<u8>() {
                    Ok(index) => {
                        let mut error = false;
                        let mut twt = ThreeWayTexture::new();
                        for file in &files {
                            match npcs_textures_dir.get_file(npcs_textures_dir.path().join(index.to_string()).join(file)) {
                                Some(file) => {
                                    twt.add_texture_manager(Box::new(StillTextureManager::new(crate::util::graphics::texture::byte_texture(file.contents()), false)));
                                }
                                None => {
                                    warn!("Could not get texture {} under NPC texture folder {}", file, npc_texture_dir.path);
                                    error = true;
                                }
                            }
                            
                        }
                        if !error {
                            npc_textures.insert(index, twt);
                        }
                    }
                    Err(err) => warn!("Found an npc texture folder with an unparsable name at {} with error {}", npc_texture_dir.path, err),
                }
            }
        }
        None => {
            macroquad::prelude::error!("Could not find NPC textures folder!");
        }
    }
}

pub fn load_npc_entries(root_path: &include_dir::Dir, map_index: Option<usize>) -> Vec<NPC> {

    let mut npcs = Vec::new();

    match root_path.get_dir(root_path.path().join("npcs")) {
        Some(npc_dir) => {
            match map_index {
                Some(map_index) => {
                    match npc_dir.get_dir(npc_dir.path().join(String::from("map_") + map_index.to_string().as_str())) {
                        Some(npc_dir_mapset) => {
                            get_npc_from_directory(&mut npcs, npc_dir_mapset);
                            //     warn!(
                            //         "Error fetching npc under {:?} with error: {}",
                            //         root_path, err
                            //     );
                            // }
                        }
                        None => {
                            warn!("Problem reading npc map set directory #{} under path {}", map_index, npc_dir.path);
                        }
                    }                    
                },
                None => {
                    get_npc_from_directory(&mut npcs, npc_dir);
                    //     warn!(
                    //         "Error fetching npc under {:?} with error: {}",
                    //         &npc_path, err
                    //     );
                    // }
                }
            }
        }

        None => {
            warn!(
                "Could not read NPC directory for map {}",
                &root_path.path,
            );
        }
    }
    npcs
}

pub fn get_npc_from_directory(npcs: &mut Vec<NPC>, dir: include_dir::Dir) {
    for file in dir.files() {
        if let Some(npc_entry) = load_npc(file) {
            npcs.push(npc_entry);
        }
    }
}

pub fn load_npc(file: &include_dir::File) -> Option<NPC> {

    match file.contents_utf8() {
        Some(data) => {

            let npc_entry: Result<NPC, serde_json::Error> = serde_json::from_str(data);

            match npc_entry {
                Ok(npc) => {
                    return Some(npc);
                },
                Err(err) => {
                    warn!("Could not parse NPC json at {} with error {}", file.path, err);
                    return None;
                }
            }

        },
        None => {
            warn!("Could not get NPC json at {}", file.path);
            return None;
        }
    }


}