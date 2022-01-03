use hashbrown::HashMap;

pub type NpcGroupId = tinystr::TinyStr16;

pub type NpcGroupOutput = HashMap<NpcGroupId, Vec<u8>>;

#[cfg(feature = "compile")]
pub const NAME: &str = "NPC group";

#[cfg(feature = "compile")]
pub fn get_npc_groups(path: impl AsRef<std::path::Path>) -> NpcGroupOutput {
    std::fs::read_dir(path)
        .unwrap_or_else(|err| panic!("Could not read {} directory with error {}", NAME, err))
        .flatten()
        .map(|d| d.path())
        .filter(|p| p.is_file())
        .map(|path| {
            (
                path.file_stem()
                    .unwrap_or_else(|| panic!("Could not get filename for {} at {:?}", NAME, path))
                    .to_string_lossy()
                    .parse()
                    .unwrap_or_else(|err| {
                        panic!(
                            "Cannot parse file name for {} at {:?} with error {}",
                            NAME, path, err
                        )
                    }),
                std::fs::read(&path).unwrap_or_else(|err| {
                    panic!(
                        "Could not read {} entry at {:?} with error {}",
                        NAME, path, err
                    )
                }),
            )
        })
        .collect()
}
