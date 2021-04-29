use firecore_util::Location;
use serde::{Deserialize, Serialize};
use deps::{
    hash::{HashMap, HashSet},
    tinystr::{TinyStr8, TinyStr16},
};

use self::map::MapData;

pub mod map;

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldStatus {

    // battled trainers, map stops, etc.
    pub map: HashMap<TinyStr16, MapData>,
    pub scripts: HashSet<TinyStr16>,
    pub badges: HashSet<TinyStr16>,
    pub heal: Location,

}

impl WorldStatus {

    pub fn get_map(&mut self, id: &TinyStr16) -> &mut MapData {
        if !self.map.contains_key(id) {
            self.map.insert(*id, MapData::default());            
        }
        self.map.get_mut(id).unwrap()
    }

    pub fn has_battled(&mut self, map: &TinyStr16, npc: &TinyStr8) -> bool {
		self.get_map(map).battled.contains(npc)
	}

}

impl Default for WorldStatus {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            scripts: HashSet::default(),
            badges: HashSet::default(),
            heal: crate::default_location(),
        }
    }
}