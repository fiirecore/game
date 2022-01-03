use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use world::{
    map::{chunk::Connection, PaletteId, WorldMapSettings, manager::tile::PaletteTileDatas, wild::WildChances, MusicId},
    positions::{CoordinateInt, Direction, Location, Position},
};

use self::structs::BuilderLocation;

pub mod map;
pub mod textures;
// pub mod constants;

pub mod npc;
// pub mod script;
pub mod warp;
pub mod wild;

pub mod structs;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapConfig {
    pub identifier: BuilderLocation,
    pub name: String,

    /// Map file path
    pub map: String,
    /// Border file path
    pub border: String,

    pub width: usize,
    pub height: usize,

    pub palettes: [PaletteId; 2],

    pub music: MusicId,

    #[serde(default)]
    pub chunk: HashMap<Direction, Vec<BuilderConnection>>,

    #[serde(default)]
    pub settings: WorldMapSettings,
    // #[serde(default)]
    // pub pokemon_center: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BuilderWorldData {
    pub tiles: PaletteTileDatas,
    pub wild: WildChances,
    pub spawn: (Location, Position),
}

#[derive(Serialize, Deserialize)]
pub struct BuilderConnection(BuilderLocation, CoordinateInt);

impl From<BuilderConnection> for Connection {
    fn from(connection: BuilderConnection) -> Self {
        Self(connection.0.into(), connection.1)
    }
}

impl From<Connection> for BuilderConnection {
    fn from(connection: Connection) -> Self {
        Self(connection.0.into(), connection.1)
    }
}
