use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::{
    ffi::OsString,
    fs::{read, read_to_string},
};

use hashbrown::HashMap;

use world::{
    character::npc::group::NpcGroup, map::manager::WorldNpcData,
    serialized::SerializedNpcGroupTextures,
};

pub fn load_npc_groups(root_path: &Path) -> (WorldNpcData, SerializedNpcGroupTextures) {
    let npc_dir = root_path.join("npcs");
    let trainer_dir = root_path.join("trainers");
    let mut npcs = HashMap::new();
    let mut trainers = HashMap::new();
    let mut textures = HashMap::new();

    for entry in read_dir(&npc_dir)
        .unwrap_or_else(|err| {
            panic!(
                "Could not get NPC group directory at {:?} with error {}",
                npc_dir, err
            )
        })
        .map(|entry| {
            entry.unwrap_or_else(|err| {
                panic!(
                    "Could not directory entry under {:?} with error {}",
                    npc_dir, err
                )
            })
        })
    {
        let path = entry.path();
        if path.is_dir() {
            let ron_path = get_npc_type_file(&path);

            let id = crate::filename(&ron_path);

            let group: NpcGroup = ron::from_str(&read_to_string(&ron_path).unwrap_or_else(|err| {
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
            let texture = read(&sprite_path).unwrap_or_else(|err| {
                panic!(
                    "Could not get npc sprite at {:?} with error {}",
                    sprite_path, err
                )
            });

            npcs.insert(id1, group);
            textures.insert(id1, texture);
        }
    }

    for path in read_dir(&trainer_dir)
        .unwrap_or_else(|err| {
            panic!(
                "Could not get trainer group directory at {:?} with error {}",
                trainer_dir, err
            )
        })
        .map(|entry| {
            entry.unwrap_or_else(|err| {
                panic!(
                    "Could not directory entry under {:?} with error {}",
                    trainer_dir, err
                )
            })
        })
        .map(|d| d.path())
    {
        let id = crate::filename(&path).parse().unwrap_or_else(|err| {
            panic!(
                "Could not parse file name at {:?} into trainer group id with error {}",
                path, err
            )
        });

        let trainer = ron::from_str(&read_to_string(&path).unwrap_or_else(|err| {
            panic!(
                "Could not read trainer group entry at {:?} with error {}",
                path, err
            )
        }))
        .unwrap_or_else(|err| {
            panic!(
                "Could not deserialize trainer group at {:?} with error {}",
                path, err
            )
        });

        trainers.insert(id, trainer);
    }

    (
        WorldNpcData {
            groups: npcs,
            trainers,
        },
        textures,
    )
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
