use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use worldlib::map::NpcManager;
use worldlib::serialized::SerializedNpc;
use worldlib::map::NpcMap;

pub mod npc_type;

pub fn load_npc_entries(npc_path: PathBuf) -> NpcManager {
    read_dir(npc_path).map(|dir| {
        dir.flatten().map(|entry| entry.path())
            .map(|file| (read_to_string(&file).unwrap_or_else(|err| panic!("Could not get Npc file at {:?} with error {}", file, err)), file))
            .map(|(data, file)| ron::from_str::<SerializedNpc>(&data).unwrap_or_else(|err| panic!("Could not parse Npc at {:?} with error {} at position {}", file, err, err.position)))
            .map(|npc| (npc.id, Some(npc.npc)))
            .collect::<NpcMap>()
    }).unwrap_or_default().into()
}