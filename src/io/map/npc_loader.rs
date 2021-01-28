use std::fs::ReadDir;
use std::fs::read_dir;
use crate::util::file::read_to_string;
use std::path::Path;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use crate::entity::texture::still_texture_manager::StillTextureManager;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::world::npc::NPC;

pub async fn load_npc_textures(npc_textures: &mut ahash::AHashMap<u8, ThreeWayTexture>) {
    info!("Loading NPC textures...");

    let mut archive = crate::io::map::WORLD_ARCHIVE.lock();

    let mut npctex_paths: ahash::AHashSet<String> = ahash::AHashSet::new();

    for entry in archive.file_names() {
        if entry.starts_with("textures/npcs") && !entry.ends_with(".png") && entry.len() > 14 {
            npctex_paths.insert(entry.to_string());
        }
    }

    for root_path in npctex_paths {

        let mut up = root_path.clone();
        up.push_str("idle_up.png");
        let mut down = root_path.clone();
        down.push_str("idle_down.png");
        let mut side = root_path.clone();
        side.push_str("idle_side.png");

        let files = [up, down, side];

        match root_path[14..root_path.len() - 1].parse::<u8>() {
            Ok(id) => {
                let mut twt = ThreeWayTexture::new();
                for file in &files {
                    let mut buf: Vec<u8> = Vec::new();
                    match std::io::Read::read_to_end(&mut archive.by_name(file).expect(&format!("Could not find idle_up file for NPC {}", id)), &mut buf) {
                        Ok(_) => {
                            twt.add_texture_manager(Box::new(StillTextureManager::new(crate::util::texture::byte_texture(buf.as_slice()), false)));
                        }
                        Err(err) => {
                            warn!("Could not read image in world archive {} with error {}", &root_path, err);
                        }
                    }
                }
                npc_textures.insert(id, twt);
            }
            Err(err) => warn!("Found an npc texture folder with an unparsable name at {:?} with error {}", &root_path, err),
        }
    }
}

pub async fn load_npc_entries<P>(root_path: P, map_index: Option<usize>) -> Vec<NPC> where P: AsRef<Path> {
    let root_path = root_path.as_ref();
    let npc_path = root_path.join("npcs");

    let mut npcs = Vec::new();

    match read_dir(&npc_path) {
        Ok(dir) => {
            match map_index {
                Some(map_index) => {
                    let mut map_set = String::from("map_");
                    map_set.push_str(map_index.to_string().as_str());
                    match read_dir(&npc_path.join(map_set)) {
                        Ok(dir) => {
                            if let Some(err) = get_npc_from_directory(&mut npcs, dir).await {
                                warn!(
                                    "Error fetching npc under {:?} with error: {}",
                                    root_path, err
                                );
                            }
                        }
                        Err(err) => {
                            warn!("Problem reading npc map set directory #{} under path {:?} with error {}", map_index, &npc_path, err);
                        }
                    }                    
                },
                None => {
                    if let Some(err) = get_npc_from_directory(&mut npcs, dir).await {
                        warn!(
                            "Error fetching npc under {:?} with error: {}",
                            &npc_path, err
                        );
                    }
                }
            }
        }

        Err(err) => {
            warn!(
                "Could not read NPC directory for map {:?} with error {}",
                &root_path,
                err
            );
        }
    }
    npcs
}

pub async fn get_npc_from_directory(npcs: &mut Vec<NPC>, dir: ReadDir) -> Option<std::io::Error> {
    for path_result in dir.map(|res| res.map(|e| e.path())) {
        match path_result {
            Ok(path) => {
                if let Some(npc_entry) = load_npc(path).await {
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

pub async fn load_npc<P>(path: P) -> Option<NPC> where P: AsRef<Path> {
    let path = path.as_ref();

    match read_to_string(path).await {
        Ok(string) => {

            let npc_entry: Result<NPC, serde_json::Error> = serde_json::from_str(string.as_str());

            match npc_entry {
                Ok(npc) => {
                    return Some(npc);
                },
                Err(err) => {
                    warn!("Could not parse NPC json at {:?} with error {}", path, err);
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