use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{map::TileId, positions::Direction};

// #[derive(Serialize, Deserialize)]
// #[serde(transparent)]
// pub struct PaletteTiles(pub HashMap<PaletteId, PaletteTileData>);

// impl PaletteTiles {

//     pub fn get(&self, map: &WorldMap, tile: TileId) -> Option<&PaletteTileData> {

//     }

// }

#[derive(Serialize, Deserialize)]
pub struct PaletteTileData {
    // pub bushes: HashMap<BushType, TileId>,
    pub cliffs: HashMap<Direction, Vec<TileId>>,
    pub forwarding: Vec<TileId>,
}

impl Default for PaletteTileData {
    fn default() -> Self {
        let forwarding = vec![0x298, 0x2A5];

        let mut cliffs = HashMap::with_capacity(1);

        cliffs.insert(Direction::Left, vec![133]);
        cliffs.insert(Direction::Right, vec![134]);
        cliffs.insert(
            Direction::Down,
            vec![135, 176, 177, 143, 151, 184, 185, 192, 193, 1234],
        );

        Self {
            // bushes: Default::default(),
            cliffs,
            forwarding,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WildType {
    Bush(BushType),
    Water,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum BushType {
    Short,
    Tall,
}
