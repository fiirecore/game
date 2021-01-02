use crate::engine::game_context::GameContext;
use crate::entity::entities::player::Player;
use crate::game::npc::npc::NPC;
use crate::game::{warp::warp_entry::WarpEntry, world::pokemon::wild_pokemon_table::WildPokemonTable};
use crate::util::map_util::GameMap;

pub struct WorldMapPiece {

    pub name: String,
    pub music: u8,

    pub x: isize,
    pub y: isize,

    pub width: u16,
    pub height: u16,

    pub tile_map: Vec<u16>,
    pub border_blocks: [u16; 4],
    pub movement_map: Vec<u8>,

    pub connections: Vec<usize>,
    pub warps: Vec<WarpEntry>,
    pub npcs: Vec<NPC>,

    pub wild_tiles: Option<Vec<u16>>,
    pub wild_pokemon_table: Option<Box<dyn WildPokemonTable>>,

}

impl GameMap for WorldMapPiece {

    fn tile(&self, x: isize, y: isize) -> u16 {
        self.tile_map[x as usize + y as usize * self.width as usize]
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        for warp in &self.warps {
            if warp.x == x {
                if warp.y == y {
                    return Some(warp.clone());
                }
            }
        }
        return None;
    }

    fn input(&mut self, context: &mut GameContext, player: &Player) {
        
    }

}