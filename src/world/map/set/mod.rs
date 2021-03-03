use crate::world::NpcTextures;
use crate::world::RenderCoords;
use crate::world::TileTextures;
use crate::world::World;
use crate::world::map::WorldMap;
use crate::world::player::Player;
use crate::world::warp::WarpEntry;

pub mod manager;

#[derive(Default)]
pub struct WorldMapSet {

    maps: Vec<WorldMap>,
    current_map: usize,

}

impl WorldMapSet {

    pub fn new(maps: Vec<WorldMap>) -> Self {
        Self {
            maps: maps,
            current_map: 0,
        }
    }

    pub fn set(&mut self, index: usize) {
        self.current_map = index;
    }

    pub fn get(&self) -> &usize {
        &self.current_map
    }

 
    pub fn map(&self) -> &WorldMap {
        &self.maps[self.current_map]
    }

    pub fn map_mut(&mut self) -> &mut WorldMap {
        &mut self.maps[self.current_map]
    }


    pub(crate) fn tiles(&self) -> Vec<crate::world::TileId> {
        let mut tiles = Vec::new(); 
        for map in &self.maps {
            for tile_id in &map.tile_map {
                if !tiles.contains(tile_id) {
                    tiles.push(*tile_id);
                }        
            }
            for tile_id in &map.border_blocks {
                if !tiles.contains(tile_id) {
                    tiles.push(*tile_id);
                }
            }
        }
        return tiles;
    }

}

impl World for WorldMapSet {

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        self.maps[self.current_map].in_bounds(x, y)
    }

    fn tile(&self, x: isize, y: isize) -> u16 {
        self.maps[self.current_map].tile(x, y)
    }

    fn walkable(&self, x: isize, y: isize) -> u8 {
        if self.in_bounds(x, y) {
            self.maps[self.current_map].walkable(x, y)
        } else {
            1
        }
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        self.maps[self.current_map].check_warp(x, y)
    }

    fn on_tile(&mut self, player: &mut Player) {
        self.maps[self.current_map].on_tile(player)
    }

    fn update(&mut self, delta: f32, player: &mut Player) {
        self.maps[self.current_map].update(delta, player);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        self.maps[self.current_map].render(tile_textures, npc_textures, screen, border)
    }

    fn input(&mut self, delta: f32, player: &mut Player) {
        self.maps[self.current_map].input(delta, player)
    }

}