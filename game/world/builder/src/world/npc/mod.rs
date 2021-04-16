use std::path::PathBuf;
use ahash::AHashMap as HashMap;

use firecore_world_lib::serialized::SerializedNPC;
use firecore_world_lib::map::npc::NPCManager;

pub mod npc_type;

pub fn load_npc_entries(npc_path: PathBuf) -> NPCManager {
    let mut npcs = HashMap::new();
    if let Ok(dir) = std::fs::read_dir(npc_path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let file = entry.path();
                let data = std::fs::read_to_string(&file).unwrap_or_else(|err| panic!("Could not get NPC file at {:?} with error {}", file, err));
                let npc_result: Result<SerializedNPC, ron::Error> = ron::from_str(&data);
                match npc_result {
                    Ok(npc) => {
                        npcs.insert(npc.index, npc.npc);
                    },
                    Err(err) => {
                        panic!("Could not parse NPC at {:?} with error {} at position {}", file, err, err.position);
                    },
                }
            }
        }
    } 
    NPCManager::new(npcs)
}