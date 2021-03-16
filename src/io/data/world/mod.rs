use serde::{Deserialize, Serialize};
use ahash::{AHashMap as HashMap, AHashSet as HashSet};

use self::map_data::MapData;

pub mod map_data;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WorldStatus {

    // battled trainers, map stops, etc.
    pub map_data: HashMap<String, MapData>,
    pub completed_events: HashSet<String>,

}

impl WorldStatus {

    pub fn get_or_create_map_data(&mut self, name: &String) -> &mut MapData {
        if self.map_data.contains_key(name) {
            return self.map_data.get_mut(name).unwrap();
        } else {
            self.map_data.insert(name.clone(), MapData::default());
            return self.get_or_create_map_data(name);
        }
    }

}