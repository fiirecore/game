use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use worldlib::serialized::SerializedNPC;
use worldlib::map::NPCMap;

pub mod npc_type;

pub fn load_npc_entries(npc_path: PathBuf) -> NPCMap {
    read_dir(npc_path).map(|dir| {
        dir.flatten().map(|entry| entry.path())
            .map(|file| (read_to_string(&file).unwrap_or_else(|err| panic!("Could not get NPC file at {:?} with error {}", file, err)), file))
            .map(|(data, file)| ron::from_str::<SerializedNPC>(&data).unwrap_or_else(|err| panic!("Could not parse NPC at {:?} with error {} at position {}", file, err, err.position)))
            .map(|npc| (npc.id, npc.npc))
            .collect::<NPCMap>()
    }).unwrap_or_default()
}