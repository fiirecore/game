use audio::music::MusicId;
use deps::vec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        npc::Npcs,
        Movement,
    },
    positions::{Coordinate, Location},
    script::world::WorldScript,
};

use warp::{WarpDestination, Warps};
use wild::WildEntry;

pub mod manager;
pub mod warp;
pub mod wild;

pub type TileId = u16;
pub type MovementId = u8;
pub type MapSize = usize;
pub type PaletteId = u8;

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

    pub border: [TileId; 4], //Border, // border blocks

    pub chunk: Option<WorldChunk>,

    // Map objects
    pub warps: Warps,

    pub wild: Option<WildEntry>,

    pub npcs: Npcs,

    // pub objects: HashMap<u8, MapObject>,
    pub scripts: Vec<WorldScript>,

    #[serde(default)]
    pub settings: WorldMapSettings,
    // pub mart: Option<mart::Pokemart>,

}

pub type ChunkConnections = ArrayVec<[Location; 6]>;

#[derive(Serialize, Deserialize)]
pub struct WorldChunk {
    pub connections: ChunkConnections,
    pub coords: Coordinate,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorldMapSettings {
    #[serde(default)]
    pub fly_position: Option<Coordinate>,
    #[serde(default)]
    pub brightness: Brightness,
}

pub fn can_move(movement: Movement, code: MovementId) -> bool {
    match movement {
        Movement::Swimming => can_swim(code),
        _ => can_walk(code),
    }
}

pub fn can_walk(code: MovementId) -> bool {
    code == 0xC
}

pub fn can_swim(code: MovementId) -> bool {
    code == 0x4
}

impl World for WorldMap {
    fn in_bounds(&self, coords: Coordinate) -> bool {
        !(coords.x.is_negative()
            || coords.x >= self.width as i32
            || coords.y.is_negative()
            || coords.y >= self.height as i32)
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        self.in_bounds(coords)
            .then(|| self.tiles[coords.x as usize + coords.y as usize * self.width])
    }

    fn movement(&self, coords: Coordinate) -> Option<MovementId> {
        if self
            .npcs
            .values()
            .any(|npc| npc.character.position.coords == coords)
        {
            return Some(1);
        }
        self.in_bounds(coords)
            .then(|| self.movements[coords.x as usize + coords.y as usize * self.width])
    }

    fn warp_at(&self, coords: Coordinate) -> Option<&WarpDestination> {
        self.warps
            .values()
            .find(|warp| warp.location.in_bounds(&coords))
            .map(|entry| &entry.destination)
    }
}

#[deprecated(note = "move")]
#[derive(Debug, Deserialize, Serialize)]
pub enum Brightness {
    Day,
    Night,
    // FlashNeeded,
}

// #[deprecated]
// #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
// pub enum MapIcon {
//     City(u8, u8),
//     Route(u8, u8, u8, u8),
// }

impl Default for Brightness {
    fn default() -> Self {
        Self::Day
    }
}
