use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use text::TextColor;

use crate::{
    character::{
        npc::{Npc, NpcId, TrainerType},
        sprite::SpriteIndexType,
    },
    map::{manager::WorldMapManager, PaletteId, TileId},
    TrainerId,
    // positions::Location,
};

// pub type MapGuiLocs = HashMap<crate::map::MapIcon, (String, Location)>;

#[derive(Deserialize, Serialize)]
pub struct SerializedWorld {
    pub manager: WorldMapManager,

    pub npc_types: Vec<SerializedNpcType>,
    // pub map_gui_locs: MapGuiLocs,
    pub textures: SerializedTextures,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedNpc {
    pub id: NpcId,
    pub npc: Npc,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedNpcTypeConfig {
    pub identifier: TrainerId,
    pub text_color: TextColor,
    pub sprite: SpriteIndexType,
    pub trainer: Option<TrainerType>,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedNpcType {
    pub config: SerializedNpcTypeConfig,

    pub texture: Vec<u8>,
}

pub type Palettes = HashMap<PaletteId, Vec<u8>>;
pub type Animated = HashMap<TileId, Vec<u8>>;

pub type Doors = HashMap<Vec<TileId>, Vec<u8>>;

#[derive(Deserialize, Serialize)]
pub struct SerializedTextures {
    pub palettes: Palettes,

    pub animated: Animated,

    pub doors: Doors,
}

#[derive(Deserialize)]
pub struct SerializedDoor {
    pub tiles: Vec<TileId>,
    pub file: String,
}
