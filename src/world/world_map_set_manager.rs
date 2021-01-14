use std::collections::HashMap;
use std::collections::hash_map::Values;

use crate::entity::entity::Entity;

use super::world_map_set::WorldMapSet;

#[derive(Default)]
pub struct WorldMapSetManager {

    alive: bool,

    map_sets: HashMap<String, WorldMapSet>,
    current_map_set: String,

}

impl WorldMapSetManager {


    pub fn set(&mut self, set: String) {
        if self.map_sets.contains_key(&set) {
            self.current_map_set = set;
        }
    }

    pub fn get(&self) -> &String {
        &self.current_map_set
    }


    pub fn insert(&mut self, id: String, map_set: WorldMapSet) {
        self.map_sets.insert(id, map_set);
    }


    pub fn map_set(&self) -> &WorldMapSet {
        self.map_sets.get(&self.current_map_set).expect("Could not get current map set")
    }

    pub fn map_set_mut(&mut self) -> &mut WorldMapSet {
        self.map_sets.get_mut(&self.current_map_set).expect("Could not get current map set")
    }


    pub fn values(&self) -> Values<'_, String, WorldMapSet> {
        self.map_sets.values()
    }

}

impl Entity for WorldMapSetManager {
    fn spawn(&mut self) {
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}