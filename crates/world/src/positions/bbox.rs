use serde::{Deserialize, Serialize};

use crate::positions::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BoundingBox {
    pub min: Coordinate,
    pub max: Coordinate,
}

impl BoundingBox {

    pub fn centered(coords: Coordinate, range: Coordinate) -> Self {
        Self {
            min: coords - range,
            max: coords + range,
        }
    }

    pub const fn contains(&self, coordinate: &Coordinate) -> bool {
        if coordinate.x >= self.min.x && coordinate.x <= self.max.x {
            coordinate.y >= self.min.y && coordinate.y <= self.max.y
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Coordinate> + '_ {
        (self.min.x..=self.max.x).flat_map(|x| {
            (self.min.y..=self.max.y)
                .into_iter()
                .map(move |y| Coordinate { x, y })
        })
    }
}

impl From<Coordinate> for BoundingBox {
    fn from(coords: Coordinate) -> Self {
        Self {
            min: coords,
            max: coords,
        }
    }
}