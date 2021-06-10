use deps::hash::HashMap;
use firecore_font::message::TextColor;
use serde::{Deserialize, Serialize};

use crate::character::npc::npc_type::NpcTypeId;
use crate::map::{
    TileId,
    PaletteId, 
    // TileLocation,
};
use crate::character::npc::Npc;
use crate::character::npc::NpcId;
use crate::character::npc::npc_type::TrainerType;
use crate::character::sprite::SpriteIndexType;
use crate::map::manager::WorldMapManager;

pub type MapGuiLocs = HashMap<crate::map::MapIcon, (String, util::Location)>;

#[derive(Deserialize, Serialize)]
pub struct SerializedWorld {

    pub manager: WorldMapManager,

    pub npc_types: Vec<SerializedNpcType>,
    pub map_gui_locs: MapGuiLocs,
    pub textures: SerializedTextures,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedNpc {

    pub id: NpcId,
    pub npc: Npc,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedNpcTypeConfig {

    pub identifier: NpcTypeId,
    pub text_color: TextColor,
    pub sprite: SpriteIndexType,
    pub trainer: Option<TrainerType>,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedNpcType {

    pub config: SerializedNpcTypeConfig,

    pub texture: Vec<u8>,
    pub battle_texture: Option<Vec<u8>>,

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