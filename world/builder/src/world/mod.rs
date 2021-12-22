use std::collections::HashMap;

use either::Either;

use serde::{Deserialize, Serialize};
use tinystr::TinyStr16;
use worldlib::{
    map::{WorldMapSettings, chunk::Connection, PaletteId},
    positions::{LocationId, Direction, Location, CoordinateInt},
};

pub mod map;
pub mod textures;
// pub mod constants;

pub mod npc;
pub mod script;
pub mod warp;
pub mod wild;
// pub mod mart;

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
    pub chunk: HashMap<Direction, MapConnection>,

    #[serde(default)]
    pub settings: WorldMapSettings,
    // #[serde(default)]
    // pub pokemon_center: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MapConnection(MapLocation, CoordinateInt);

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct MapLocation {
    #[serde(with = "either::serde_untagged")]
    inner: Either<LocationId, Location>,
}

impl From<MapLocation> for Location {
    fn from(location: MapLocation) -> Self {
        match location.inner {
            Either::Left(id) => Location::from(id),
            Either::Right(loc) => loc,
        }
    }
}

impl From<Location> for MapLocation {
    fn from(location: Location) -> Self {
        Self {
            inner: match location.map.is_some() {
                true => Either::Right(location),
                false => Either::Left(location.index),
            },
        } 
    }
}

impl From<MapConnection> for Connection {
    fn from(connection: MapConnection) -> Self {
        let location = connection.0.into();
        Self(location, connection.1)
    }
}

impl From<Connection> for MapConnection {
    fn from(connection: Connection) -> Self {
        Self(connection.0.into(), connection.1)
    }
}