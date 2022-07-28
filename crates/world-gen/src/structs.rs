use either::Either;
use firecore_world::positions::{BoundingBox, Coordinate, Location, LocationId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
#[serde(transparent)]
pub struct BuilderLocation {
    #[serde(with = "either::serde_untagged")]
    inner: Either<LocationId, (LocationId, LocationId)>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct BuilderArea {
    #[serde(with = "either::serde_untagged")]
    inner: Either<Coordinate, BoundingBox>,
}

impl From<BuilderLocation> for Location {
    fn from(location: BuilderLocation) -> Self {
        match location.inner {
            Either::Left(id) => Location::from(id),
            Either::Right((map, index)) => Location {
                map: Some(map),
                index,
            },
        }
    }
}

impl From<Location> for BuilderLocation {
    fn from(location: Location) -> Self {
        Self {
            inner: match location.map {
                Some(map) => Either::Right((map, location.index)),
                None => Either::Left(location.index),
            },
        }
    }
}

impl From<BuilderArea> for BoundingBox {
    fn from(area: BuilderArea) -> Self {
        match area.inner {
            Either::Left(coords) => BoundingBox::from(coords),
            Either::Right(bb) => bb,
        }
    }
}

impl From<BoundingBox> for BuilderArea {
    fn from(area: BoundingBox) -> Self {
        Self {
            inner: match area.min != area.max {
                true => Either::Right(area),
                false => Either::Left((area.min.x, area.min.y).into()),
            },
        }
    }
}
