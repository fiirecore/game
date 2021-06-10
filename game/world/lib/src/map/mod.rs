use serde::{Deserialize, Serialize};
use util::{
    Coordinate,
    Location,
};
use firecore_audio_lib::music::MusicId;

use crate::script::world::WorldScript;

use wild::WildEntry;
use warp::{WarpMap, WarpDestination};

pub mod manager;

pub mod warp;
pub mod wild;

// pub mod mart;

mod chunk;
pub use chunk::*;

mod npc;
pub use npc::*;

// pub mod object;

pub type TileId = u16;
pub type MovementId = u8;
pub type MapSize = usize;

pub type PaletteId = u8;

// pub type TileLocation = (PaletteId, TileId);

pub trait World {

    fn in_bounds(&self, coords: Coordinate) -> bool;

    fn tile(&self, coords: Coordinate) -> Option<TileId>;

    fn movement(&self, coords: Coordinate) -> Option<MovementId>;

    fn warp_at(&self, coords: Coordinate) -> Option<&WarpDestination>;

}

#[derive(Serialize, Deserialize)]
pub struct WorldMap {

    pub id: Location,

    pub name: String,
    pub music: MusicId,

    pub width: MapSize,
    pub height: MapSize,

    pub palettes: [PaletteId; 2],

    pub tiles: Vec<TileId>,
    pub movements: Vec<MovementId>,

    pub border: [TileId; 4],//Border, // border blocks

    pub chunk: Option<WorldChunk>,

    // Map objects

    pub warps: WarpMap,

    pub wild: Option<WildEntry>,
    
    pub npcs: NpcManager,

    // pub objects: HashMap<u8, MapObject>,

    pub scripts: Vec<WorldScript>,

    #[serde(default)]
    pub fly_position: Option<Coordinate>,

    #[serde(default)]
    pub time: WorldTime,

    // pub mart: Option<mart::Pokemart>,

    // #[serde(skip)]
    // pub state: WorldMapState,

}

#[derive(Debug, Deserialize, Serialize)]
pub enum WorldTime {
    Day,
    Night,
}



#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum MapIcon {
    City(u8, u8),
    Route(u8, u8, u8, u8),
}

impl Default for WorldTime {
    fn default() -> Self {
        Self::Day
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

impl WorldMap {

    pub fn tenth_walkable_coord(&self) -> Option<Coordinate> {
        let mut count: u8 = 0;
        for (i, m) in self.movements.iter().copied().enumerate() {
            if manager::can_walk(m) {
                count += 1;
                if count == 10 {
                    return Some(Coordinate::new((i % self.width) as _, (i / self.width) as _));
                }
            }
        }
        None
    }

}