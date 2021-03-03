use ahash::AHashMap as HashMap;
use crate::world::NpcTextures;
use crate::world::RenderCoords;
use crate::world::TileTextures;
use crate::world::World;
use crate::world::player::Player;
use crate::world::warp::WarpEntry;

use super::WorldMapSet;

#[derive(Default)]
pub struct WorldMapSetManager {

    map_sets: HashMap<String, WorldMapSet>,
    current_map_set: String,

}

impl WorldMapSetManager {


    pub fn set(&mut self, set: &String) {
        if self.map_sets.contains_key(set) {
            self.current_map_set = set.clone();
        }
    }

    pub fn set_index(&mut self, index: usize) {
        self.map_set_mut().set(index)
    }

    pub fn get(&self) -> &String {
        &self.current_map_set
    }

    pub fn get_index(&self) -> &usize {
        self.map_set().get()
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


    pub fn values(&self) -> std::collections::hash_map::Values<'_, String, WorldMapSet> {
        self.map_sets.values()
    }

}

impl World for WorldMapSetManager {

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        self.map_set().in_bounds(x, y)
    }

    fn tile(&self, x: isize, y: isize) -> u16 {
        self.map_set().tile(x, y)
    }

    fn walkable(&self, x: isize, y: isize) -> u8 {
        self.map_set().walkable(x, y)
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        self.map_set().check_warp(x, y)
    }

    fn on_tile(&mut self, player: &mut Player) {
        self.map_set_mut().on_tile(player)
    }

    fn update(&mut self, delta: f32, player: &mut Player) {
        self.map_set_mut().update(delta, player);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        self.map_set().render(tile_textures, npc_textures, screen, border)
    }

    fn input(&mut self, delta: f32, player: &mut Player) {
        self.map_set_mut().input(delta, player)
    }
}