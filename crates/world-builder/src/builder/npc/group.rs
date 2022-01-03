use std::ffi::OsString;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use hashbrown::HashMap;

use world::{
    character::npc::group::{NpcGroup, NpcGroupId},
    serialized::SerializedNpcGroupTextures,
};

pub fn load_npc_types(root_path: &Path) -> (HashMap<NpcGroupId, NpcGroup>, SerializedNpcGroupTextures)  {
    let npc_types = root_path.join("npcs");
    let mut types = HashMap::new();
    let mut textures = HashMap::new();

    for entry in read_dir(&npc_types)
        .unwrap_or_else(|err| {
            panic!(
                "Could not get warp file at {:?} with error {}",
                npc_types, err
            )
        })
        .map(|entry| {
            entry.unwrap_or_else(|err| {
                panic!(
                    "Could not directory entry at {:?} with error {}",
                    npc_types, err
                )
            })
        })
    {
        let path = entry.path();
        if path.is_dir() {
            let ron_path = get_npc_type_file(&path);

            let id = crate::filename(&ron_path);

            let group: NpcGroup =
                ron::from_str(&std::fs::read_to_string(&ron_path).unwrap_or_else(|err| {
                    panic!(
                        "Could not get Npc type file at {:?} with error {}",
                        ron_path, err
                    )
                }))
                .unwrap_or_else(|err| {
                    panic!(
                        "Could not decode Npc type file at {:?} with error {}",
                        ron_path, err
                    )
                });

            let id1 = id.parse().unwrap_or_else(|err| {
                panic!(
                    "Cannot parse npc group file name {} into id with error {}",
                    id, err
                )
            });

            let sprite_path = path.join(id + ".png");
            let texture = std::fs::read(&sprite_path).unwrap_or_else(|err| {
                panic!(
                    "Could not get npc sprite at {:?} with error {}",
                    sprite_path, err
                )
            });

            types.insert(id1, group);
            textures.insert(id1, texture);
        }
    }

    (types, textures)
}

fn get_npc_type_file(path: &Path) -> PathBuf {
    for entry in read_dir(path).unwrap().flatten() {
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == OsString::from("ron") {
                return path;
            }
        }
    }
    panic!("Could not find Npc type under folder {:?}", path);
}
