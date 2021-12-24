use std::path::PathBuf;

use worldlib::map::mart::Pokemart;

pub fn load_mart(path: PathBuf) -> Option<Pokemart> {
    std::fs::read_to_string(&path).ok().map(|data| ron::from_str(&data).unwrap_or_else(|err| panic!("Could not read pokemart data at {:?} with error {}", path, err)))
}