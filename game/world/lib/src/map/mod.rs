use serde::{Deserialize, Serialize};
use deps::{
    hash::HashMap,
    vec::ArrayVec,
};
use util::{
    Coordinate,
    LocationId,
};
use firecore_audio_lib::music::MusicId;

use crate::MapSize;
use crate::MovementId;
use crate::TileId;

use crate::character::npc::{NPCId, NPC};
use crate::script::world::WorldScript;

use wild::WildEntry;
use warp::{WarpMap, WarpDestination};

pub mod manager;

pub mod warp;
pub mod wild;
// pub mod object;

pub trait World {

    fn in_bounds(&self, coords: Coordinate) -> bool;

    fn tile(&self, coords: Coordinate) -> Option<TileId>;

    fn movement(&self, coords: Coordinate) -> Option<MovementId>;

    fn warp_at(&self, coords: Coordinate) -> Option<&WarpDestination>;

}

#[derive(Serialize, Deserialize)]
pub struct WorldMap {

    pub id: LocationId,

    pub name: String,
    pub music: MusicId,

    pub width: MapSize,
    pub height: MapSize,

    pub palettes: [u8; 2],

    pub tiles: Vec<TileId>,
    pub movements: Vec<MovementId>,

    pub border: [TileId; 4],//Border, // border blocks

    pub chunk: Option<WorldChunk>,

    // Map objects

    pub warps: WarpMap,

    pub wild: Option<WildEntry>,
    
    pub npcs: NPCManager,

    // pub objects: HashMap<u8, MapObject>,

    pub scripts: Vec<WorldScript>,

    // #[serde(skip)]
    // pub state: WorldMapState,

}

pub type Connections = ArrayVec<[LocationId; 6]>;

#[derive(Serialize, Deserialize)]
pub struct WorldChunk {

    pub connections: Connections,

    pub coords: Coordinate,

}

pub type NPCMap = HashMap<NPCId, Option<NPC>>;
pub type ActiveNPC = Option<(NPCId, NPC)>;

#[derive(Default, Serialize, Deserialize)]
pub struct NPCManager {
    pub list: NPCMap,
    pub active: ActiveNPC,
}

impl NPCManager {
    pub fn get(&self, id: &NPCId) -> Option<&NPC> {
        self.list.get(id).map(|npc| npc.as_ref()).unwrap_or(self.active.as_ref().filter(|(active, _)| active == id).map(|(_, npc)| npc))
    }
    pub fn get_mut(&mut self, id: &NPCId) -> Option<&mut NPC> {
        self.list.get_mut(id).map(|npc| npc.as_mut()).unwrap_or(self.active.as_mut().filter(|(active, _)| active == id).map(|(_, npc)| npc))
    }
}

impl Into<NPCManager> for NPCMap {
    fn into(self) -> NPCManager {
        NPCManager {
            list: self,
            active: Default::default(),
        }
    }
}

impl World for WorldMap {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        !(
            coords.x.is_negative() || 
            coords.x >= self.width as i32 || 
            coords.y.is_negative() || 
            coords.y >= self.height as i32
        )
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        self.in_bounds(coords).then(|| self.tiles[coords.x as usize + coords.y as usize * self.width])     
    }

    fn movement(&self, coords: Coordinate) -> Option<MovementId> {
        if self.npcs.list.values().flatten().any(|npc| npc.character.position.coords == coords) {
            return Some(1);
        }
        if let Some((_, npc)) = self.npcs.active.as_ref() {
            if npc.character.position.coords == coords {
                return Some(1);
            }
        }
        self.in_bounds(coords).then(|| self.movements[coords.x as usize + coords.y as usize * self.width])
    }

    fn warp_at(&self, coords: Coordinate) -> Option<&WarpDestination> {
        self.warps.values().find(|warp| warp.location.in_bounds(&coords)).map(|entry| &entry.destination)
    }

}