use std::ffi::OsString;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;

use firecore_world_lib::serialized::SerializedNPCType;
use firecore_world_lib::serialized::SerializedNPCTypeConfig;

pub fn load_npc_types<P: AsRef<Path>>(npc_types: P) -> Vec<SerializedNPCType> {
    let npc_types = npc_types.as_ref();
    let mut types = Vec::new();

    for entry in read_dir(npc_types)
    .unwrap_or_else(|err| panic!("Could not get warp file at {:?} with error {}", npc_types, err))
        .map(|entry| entry.unwrap_or_else(|err| panic!("Could not directory entry at {:?} with error {}", npc_types, err))) {
        let path = entry.path();
        if path.is_dir() {
            let ron_path = get_npc_type_file(&path);
            let npc_type: SerializedNPCTypeConfig = ron::from_str(
                &std::fs::read_to_string(&ron_path).unwrap_or_else(|err| panic!("Could not get NPC type file at {:?} with error {}", ron_path, err))
            ).unwrap_or_else(|err| panic!("Could not decode NPC type file at {:?} with error {}", ron_path, err));

            let sprite_path = path.join(npc_type.identifier.to_string() + ".png");
            let battle_sprite_path = path.join("battle.png");
            let texture =  std::fs::read(&sprite_path).unwrap_or_else(|err| panic!("Could not get npc sprite at {:?} with error {}", sprite_path, err));
            
            let battle_texture = std::fs::read(battle_sprite_path).ok();

            types.push(
                SerializedNPCType {
                    config: npc_type,
                    texture,
                    battle_texture,
                }
            );
        }
    }

    types
}

fn get_npc_type_file(path: &PathBuf) -> PathBuf {
    for entry in read_dir(path).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == OsString::from("ron") {
                    return path;
                }
            }
        }
    }
    panic!("Could not find NPC type under folder {:?}", path);
}