use hashbrown::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    map::{PaletteId, TileId},
    positions::Direction,
};

// #[derive(Serialize, Deserialize)]
// #[serde(transparent)]
// pub struct PaletteTiles(pub HashMap<PaletteId, PaletteTileData>);

// impl PaletteTiles {

//     pub fn get(&self, map: &WorldMap, tile: TileId) -> Option<&PaletteTileData> {

//     }

// }

pub type PaletteTileDatas = HashMap<PaletteId, PaletteTileData>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaletteTileData {
    // pub bushes: HashMap<BushType, TileId>,
    #[serde(default)]
    pub wild: MapWildType,
    #[serde(default)]
    pub cliffs: HashMap<Direction, Vec<TileId>>,
    #[serde(default)]
    pub forwarding: Vec<TileId>,
}

impl PaletteTileData {
    pub fn iter<'a>(
        tiles: &'a PaletteTileDatas,
        palettes: &'a [PaletteId; 2],
    ) -> impl Iterator<Item = &'a PaletteTileData> + 'a {
        palettes.iter().flat_map(|p| tiles.get(p))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapWildType {
    None,
    Some(Vec<TileId>),
    All,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub enum WildLandType {
//     Land,
//     Water,
// }

impl MapWildType {
    pub fn contains(&self, tile: &TileId) -> bool {
        match self {
            MapWildType::None => false,
            MapWildType::Some(tiles) => tiles.contains(tile),
            MapWildType::All => true,
        }
    }
}

impl Default for MapWildType {
    fn default() -> Self {
        Self::None
    }
}

// #[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub enum BushType {
//     Short,
//     Tall,
// }
