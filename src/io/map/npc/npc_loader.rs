use macroquad::prelude::info;
use macroquad::prelude::warn;
use crate::util::image::byte_image;
use crate::world::NpcTextures;
use crate::world::npc::NPC;

pub fn load_npc_textures(npc_textures: &mut NpcTextures) {
    info!("Loading NPC textures...");

    //let files = ["idle_up.png", "idle_down.png", "idle_side.png"];

    match crate::io::ASSET_DIR.get_dir("world/textures/npcs") {
        Some(npcs_textures_dir) => {
            for npc_texture_dir in npcs_textures_dir.dirs() {
                let npc_type =  npc_texture_dir.path().file_name().unwrap().to_string_lossy().into_owned();
                let filepath = npcs_textures_dir.path().join(&npc_type);
                let filepath = filepath.join(npc_type.clone() + ".png");

                match npcs_textures_dir.get_file(&filepath) {
                    Some(file) => {
                        match byte_image(file.contents()) {
                            Ok(image) => match super::npc_sprite::parse_image(image) {
                                Some(twt) => {npc_textures.insert(npc_type, twt);},
                                None => warn!("Could not parse image of three way NPC texture with id {}!", &npc_type),
                            },
                            Err(err) => {
                                warn!("Could not parse NPC spritesheet at {:?} with error {}", filepath, err);
                            }
                        }
                        
                    }
                    None => {
                        warn!("Could not get texture {} under NPC texture file {:?}", npc_type, filepath);
                    },
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
                        }
                        None => {
                            warn!("Problem reading npc map set directory #{} under path {}", map_index, npc_dir.path);
                        }
                    }                    
                },
                None => {
                    get_npc_from_directory(&mut npcs, npc_dir);
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

    return npcs;
}

pub fn get_npc_from_directory(npcs: &mut Vec<NPC>, dir: include_dir::Dir) {
    for file in dir.files() {
        if let Some(npc_entry) = load_npc(file) {
            info!("Loaded NPC {}", &npc_entry.identifier.name);
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