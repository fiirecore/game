use std::collections::HashMap;

use either::Either;

use serde::Deserialize;
use worldlib::{
    map::{WorldMapSettings, chunk::Connection},
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapConfig {
    // #[deprecated(note = "use full location")]
    pub identifier: MapLocation,
    pub name: String,
    pub file: String,

    #[serde(default)]
    pub chunk: HashMap<Direction, MapConnection>,

    #[serde(default)]
    pub settings: WorldMapSettings,
    // #[serde(default)]
    // pub pokemon_center: bool,
}

#[derive(Deserialize)]
pub struct MapConnection(MapLocation, CoordinateInt);

#[derive(Deserialize)]
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

impl From<MapConnection> for Connection {
    fn from(connection: MapConnection) -> Self {
        let location = connection.0.into();
        Self(location, connection.1)
    }
}