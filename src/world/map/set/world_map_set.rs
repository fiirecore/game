use ahash::AHashMap as HashMap;
use crate::util::graphics::Texture;
use crate::world::NpcTextures;
use crate::world::RenderCoords;
use crate::world::World;
use crate::world::map::WorldMap;
use crate::world::player::Player;
use crate::world::warp::WarpEntry;

#[derive(Default)]
pub struct WorldMapSet {

    maps: Vec<WorldMap>,
    current_map_index: usize,

}

impl WorldMapSet {

    pub fn new(maps: Vec<WorldMap>) -> Self {
        Self {
            maps: maps,
            current_map_index: 0,
        }
    }

    pub fn set(&mut self, index: usize) {
        self.current_map_index = index;
    }

    pub fn get(&self) -> &usize {
        &self.current_map_index
    }

 
    pub fn map(&self) -> &WorldMap {
        &self.maps[self.current_map_index]
    }

    pub fn map_mut(&mut self) -> &mut WorldMap {
        &mut self.maps[self.current_map_index]
    }


    pub(crate) fn tiles(&self) -> Vec<u16> {
        let mut set = Vec::new(); 
        for map in &self.maps {
            for tile_id in &map.tile_map {
                if !set.contains(tile_id) {
                    set.push(*tile_id);
                }                
            }
            for tile_id in &map.border_blocks {
                if !set.contains(tile_id) {
                    set.push(*tile_id);
                }
            }
        }
        return set;
    }

}

impl World for WorldMapSet {

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        self.maps[self.current_map_index].in_bounds(x, y)
    }

    fn tile(&self, x: isize, y: isize) -> u16 {
        self.maps[self.current_map_index].tile(x, y)
    }

    fn walkable(&self, x: isize, y: isize) -> u8 {
        if self.in_bounds(x, y) {
            self.maps[self.current_map_index].walkable(x, y)
        } else {
            1
        }
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        self.maps[self.current_map_index].check_warp(x, y)
    }

    fn on_tile(&mut self, player: &mut Player) {
        self.maps[self.current_map_index].on_tile(player)
    }

    fn render(&self, textures: &HashMap<u16, Texture>, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        self.maps[self.current_map_index].render(textures, npc_textures, screen, border)
    }

    fn input(&mut self, delta: f32, player: &Player) {
        self.maps[self.current_map_index].input(delta, player)
    }

}