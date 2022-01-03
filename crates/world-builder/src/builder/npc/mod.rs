use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use world::{character::npc::Npcs, serialized::SerializedNpc};

pub mod group;

pub fn load_npc_entries(npc_path: PathBuf) -> Npcs {
    read_dir(npc_path)
        .map(|dir| {
            dir.flatten()
                .map(|entry| entry.path())
                .map(|file| {
                    (
                        read_to_string(&file).unwrap_or_else(|err| {
                            panic!("Could not get Npc file at {:?} with error {}", file, err)
                        }),
                        file,
                    )
                })
                .map(|(data, file)| {
                    ron::from_str::<SerializedNpc>(&data).unwrap_or_else(|err| {
                        panic!(
                            "Could not parse Npc at {:?} with error {} at position {}",
                            file, err, err.position
                        )
                    })
                })
                .map(|npc| (npc.id, npc.npc))
                .collect::<Npcs>()
        })
        .unwrap_or_default()
}
