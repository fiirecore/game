use either::Either;
use serde::{Serialize, Deserialize};
use world::positions::{LocationId, Location};

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