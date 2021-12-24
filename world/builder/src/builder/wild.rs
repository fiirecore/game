use std::path::PathBuf;
use world::map::wild::WildEntries;

pub fn load_wild_entries(path: PathBuf) -> Option<WildEntries> {
    // let grass = wild_path.join("grass.toml");
    std::fs::read_to_string(&path).ok().map(|data| {
        ron::from_str(&data).unwrap_or_else(|err| {
            panic!(
                "Could not parse wild entries at {:?} with error {}",
                path, err
            )
        })
    })
}
