use std::path::PathBuf;

use firecore_pokedex::item::Item;
use firecore_pokedex::serialize::{SerializedItem};

pub fn get_items<P: AsRef<std::path::Path>>(item_dir: P) -> Vec<SerializedItem> {
    let item_dir = item_dir.as_ref();
    std::fs::read_dir(item_dir)
        .unwrap_or_else(|err| panic!("Could not read item directory at {:?} with error {}", item_dir, err))
            .map(|entry| 
                entry.map(|entry| get_item_config(entry.path()))
            ).flatten().flatten().collect()
}

fn get_item_config(dir: PathBuf) -> Option<SerializedItem> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(&dir).unwrap_or_else(|err| panic!("Could not read item entry directory at {:?} with error {}", dir, err)) {
            match entry.map(|entry| entry.path()) {
                Ok(path) => {
                    if path.extension() == Some(&std::ffi::OsString::from("ron")) {
                        let data = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("Could not read item entry at {:?} to string with error {}", path, err));
                        let item: Item = ron::from_str(&data).unwrap_or_else(|err| panic!("Could not deserialize item entry at {:?} with error {}", path, err));
                        let texture = std::fs::read(dir.join(item.id.to_string() + ".png")).unwrap_or_else(|err| panic!("Could not get texture for item id {:?} with error {}", item.id, err));
                        return Some(SerializedItem {
                            item,
                            texture,
                        })
                    }                    
                }
                Err(err) => {
                    eprintln!("Could not read directory item entry with error {}", err);
                },
            }
        }
        None
    } else {
        None
    }
}