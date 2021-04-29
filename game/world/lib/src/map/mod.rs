use serde::{Deserialize, Serialize};
use deps::tinystr::TinyStr16;
use util::{
    Coordinate,
};
use firecore_audio_lib::music::MusicId;

use crate::MapSize;
use crate::MovementId;
use crate::TileId;

use crate::script::world::WorldScript;

use wild::WildEntry;
use warp::{WarpEntry, WarpDestination};

use self::npc::NPCManager;

pub mod set;
pub mod chunk;
pub mod manager;

pub mod warp;
pub mod wild;
pub mod npc;
// pub mod object;

pub type MapIdentifier = TinyStr16;

pub trait World {

    // fn len(&self) -> usize;

    fn in_bounds(&self, coords: Coordinate) -> bool;

    fn tile(&self, coords: Coordinate) -> Option<TileId>;

    fn walkable(&self, coords: Coordinate) -> MovementId; // not an option because can return 1

    fn check_warp(&self, coords: Coordinate) -> Option<warp::WarpDestination>;

}

#[derive(Serialize, Deserialize)]
pub struct WorldMap {

    pub id: MapIdentifier,

    pub name: String,
    pub music: MusicId,

    pub width: MapSize,
    pub height: MapSize,

    pub tiles: Vec<TileId>,
    pub movements: Vec<MovementId>,

    pub border: Border, // border blocks

    // Map objects

    pub warps: Vec<WarpEntry>,

    pub wild: Option<WildEntry>,
    
    pub npc_manager: NPCManager,

    // pub objects: HashMap<u8, MapObject>,

    pub scripts: Vec<WorldScript>,

}

impl WorldMap {

    pub fn tile_or_panic(&self, x: usize, y: usize) -> TileId {
        self.tiles[x + y * self.width]
    }

}

impl World for WorldMap {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        return !(coords.x < 0 || coords.x >= self.width as isize || coords.y < 0 || coords.y >= self.height as isize);
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        if self.in_bounds(coords) {
            Some(self.tiles[coords.x as usize + coords.y as usize * self.width])
        } else {
            None
        }        
    }

    fn walkable(&self, coords: Coordinate) -> MovementId {
        for npc in self.npc_manager.npcs.values() {
            if /*npc.is_alive() &&*/ npc.character.position.coords == coords {
                return 1;
            }
        }
        self.movements[coords.x as usize + coords.y as usize * self.width]
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        for warp in &self.warps {
            if warp.location.in_bounds(&coords) {
                return Some(warp.destination.clone());
            }
        }
        None
    }

}

#[derive(Default, Serialize, Deserialize)]
pub struct Border {

    pub tiles: Vec<TileId>,
    pub size: u8, // length or width (border is a square)

}