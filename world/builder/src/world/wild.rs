use std::path::PathBuf;
use worldlib::map::wild::WildEntry;

pub fn load_wild_entries(wild_path: PathBuf) -> Option<WildEntry> {
    let grass = wild_path.join("grass.toml");
    std::fs::read_to_string(&grass).ok()
        .map(|data| toml::from_str::<WildEntry>(&data).unwrap_or_else(|err| panic!("Could not parse wild pokemon table at {:?} with error {}", grass, err)))
}