use serde::{Deserialize, Serialize};

use super::{PaletteId, Palettes};

pub type TileId = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorldTile {
    Primary(TileId),
    Secondary(TileId),
}

impl WorldTile {
    pub fn id(self) -> TileId {
        match self {
            WorldTile::Primary(t) | WorldTile::Secondary(t) => t,
        }
    }

    pub fn palette<'a>(&self, palettes: &'a Palettes) -> &'a PaletteId {
        match self {
            WorldTile::Primary(..) => &palettes[0],
            WorldTile::Secondary(..) => &palettes[1],
        }
    }
}

pub type Border = [WorldTile; 4];
