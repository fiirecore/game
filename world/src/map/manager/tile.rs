use hashbrown::HashMap;

use serde::{Deserialize, Serialize};

use crate::{map::{TileId, PaletteId}, positions::Direction};

// #[derive(Serialize, Deserialize)]
// #[serde(transparent)]
// pub struct PaletteTiles(pub HashMap<PaletteId, PaletteTileData>);

// impl PaletteTiles {

//     pub fn get(&self, map: &WorldMap, tile: TileId) -> Option<&PaletteTileData> {

//     }

// }

pub type PaletteTileDatas = HashMap<PaletteId, PaletteTileData>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteTileData {
    // pub bushes: HashMap<BushType, TileId>,
    pub wild: MapWildType,
    pub cliffs: HashMap<Direction, Vec<TileId>>,
    pub forwarding: Vec<TileId>,
}

impl PaletteTileData {
    #[deprecated]
    pub fn temp_new(palette: PaletteId) -> Self {


        let forwarding = match palette {
            14 => vec![0x298, 0x2A5],
            _ => Default::default(),
        };

        let cliffs = match palette {
            0 => {
                let mut cliffs = HashMap::with_capacity(1);

                cliffs.insert(Direction::Left, vec![133]);
                cliffs.insert(Direction::Right, vec![134]);
                cliffs.insert(
                    Direction::Down,
                    vec![135, 176, 177, 143, 151, 184, 185, 192, 193, 1234],
                );

                cliffs
            },
            _ => Default::default(),
        };

        let wild = match palette {
            0 => MapWildType::Some(vec![0x0D, 290, 291, 292, 298, 299, 300, 305]),
            2 => MapWildType::Some(vec![421, 423, 429, 430, 431]),
            15 | 40 | 43 | 59 | 60 => MapWildType::All,
            _ => MapWildType::None,
        };

        Self {
            // bushes: Default::default(),
            wild,
            cliffs,
            forwarding,
        }
    }

    
    pub fn iter<'a>(tiles: &'a PaletteTileDatas, palettes: &'a [PaletteId; 2]) -> impl Iterator<Item = &'a PaletteTileData> + 'a {
        palettes.iter().flat_map(|p| tiles.get(p))
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapWildType {
    None,
    Some(Vec<TileId>),
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WildLandType {
    Land,
    Water,
}

impl MapWildType {

    pub fn contains(&self, tile: &TileId) -> bool {
        match self {
            MapWildType::None => false,
            MapWildType::Some(tiles) => tiles.contains(tile),
            MapWildType::All => true,
        }
    }

}

// #[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub enum BushType {
//     Short,
//     Tall,
// }
