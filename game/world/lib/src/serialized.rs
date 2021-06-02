use deps::hash::HashMap;
use firecore_font::message::TextColor;
use serde::{Deserialize, Serialize};

use crate::character::npc::npc_type::NPCTypeId;
use crate::map::{
    TileId,
    PaletteId, 
    // TileLocation,
};
use crate::character::npc::NPC;
use crate::character::npc::NPCId;
use crate::character::npc::npc_type::TrainerType;
use crate::character::sprite::SpriteIndexType;
use crate::map::manager::WorldMapManager;

#[derive(Deserialize, Serialize)]
pub struct SerializedWorld {

    pub manager: WorldMapManager,

    pub npc_types: Vec<SerializedNPCType>,
    pub textures: SerializedTextures,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedNPC {

    pub id: NPCId,
    pub npc: NPC,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedNPCTypeConfig {

    pub identifier: NPCTypeId,
    pub text_color: TextColor,
    pub sprite: SpriteIndexType,
    pub trainer: Option<TrainerType>,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedNPCType {

    pub config: SerializedNPCTypeConfig,

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