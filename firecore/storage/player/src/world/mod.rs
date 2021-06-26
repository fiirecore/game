use firecore_util::Location;
use firecore_util::Position;
use serde::{Deserialize, Serialize};
use deps::{
    str::{TinyStr8, TinyStr16},
    hash::{HashMap, HashSet},
};

use self::map::MapData;

pub mod map;

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldStatus {

    #[serde(default)]
    pub map: HashMap<TinyStr16, MapData>, // battled trainers, map stops, etc.
    #[serde(default)]
    pub scripts: HashSet<TinyStr16>,
    #[serde(default)]
    pub badges: HashSet<TinyStr16>,
    #[serde(default = "default_heal_loc")]
    pub heal: (Location, Position),

}

impl WorldStatus {

    pub fn get_map(&mut self, id: &Location) -> &mut MapData {
        if !self.map.contains_key(&id.index) {
            self.map.insert(id.index, MapData::default());            
        }
        self.map.get_mut(&id.index).unwrap()
    }

    pub fn has_battled(&mut self, map: &Location, npc: &TinyStr8) -> bool {
		self.get_map(map).battled.contains(npc)
	}

}

impl Default for WorldStatus {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            scripts: HashSet::default(),
            badges: HashSet::default(),
            heal: default_heal_loc(),
        }
    }
}

const fn default_heal_loc() -> (Location, Position) {
    (crate::default_location(), crate::default_position())
}