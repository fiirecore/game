use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::fs::read_dir;

use worldlib::serialized::{SerializedNpcType, SerializedNpcTypeConfig};

pub fn load_npc_types(root_path: &Path) -> Vec<SerializedNpcType> {
    let npc_types = root_path.join("npcs");
    let mut types = Vec::new();

    for entry in read_dir(&npc_types)
    .unwrap_or_else(|err| panic!("Could not get warp file at {:?} with error {}", npc_types, err))
        .map(|entry| entry.unwrap_or_else(|err| panic!("Could not directory entry at {:?} with error {}", npc_types, err))) {
        let path = entry.path();
        if path.is_dir() {
            let ron_path = get_npc_type_file(&path);
            let npc_type: SerializedNpcTypeConfig = ron::from_str(
                &std::fs::read_to_string(&ron_path).unwrap_or_else(|err| panic!("Could not get Npc type file at {:?} with error {}", ron_path, err))
            ).unwrap_or_else(|err| panic!("Could not decode Npc type file at {:?} with error {}", ron_path, err));

            let sprite_path = path.join(npc_type.identifier.to_string() + ".png");
            let battle_sprite_path = path.join("battle.png");
            let texture =  std::fs::read(&sprite_path).unwrap_or_else(|err| panic!("Could not get npc sprite at {:?} with error {}", sprite_path, err));
            
            let battle_texture = std::fs::read(battle_sprite_path).ok();

            types.push(
                SerializedNpcType {
                    config: npc_type,
                    texture,
                    battle_texture,
                }
            );
        }
    }

    types
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