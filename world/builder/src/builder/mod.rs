use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tinystr::TinyStr16;

use world::{
    map::{chunk::Connection, PaletteId, WorldMapSettings},
    positions::{CoordinateInt, Direction},
};

use self::location::MapLocation;

pub mod map;
pub mod textures;
// pub mod constants;

pub mod npc;
// pub mod script;
pub mod warp;
pub mod wild;

pub mod location;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapConfig {
    // #[deprecated(note = "use full location")]
    pub identifier: MapLocation,
    pub name: String,

    /// Map file path
    pub map: String,
    /// Border file path
    pub border: String,

    pub width: usize,
    pub height: usize,

    pub palettes: [PaletteId; 2],

    pub music: TinyStr16,

    #[serde(default)]
    pub chunk: HashMap<Direction, Vec<MapConnection>>,

    #[serde(default)]
    pub settings: WorldMapSettings,
    // #[serde(default)]
    // pub pokemon_center: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MapConnection(MapLocation, CoordinateInt);

impl From<MapConnection> for Connection {
    fn from(connection: MapConnection) -> Self {
        Self(connection.0.into(), connection.1)
    }
}

impl From<Connection> for MapConnection {
    fn from(connection: Connection) -> Self {
        Self(connection.0.into(), connection.1)
    }
}
