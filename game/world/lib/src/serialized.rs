use firecore_font::message::TextColor;
use serde::{Deserialize, Serialize};
use deps::tinystr::TinyStr16;

use crate::character::npc::NPC;
use crate::character::npc::NPCId;
use crate::character::npc::npc_type::TrainerType;
use crate::character::sprite::SpriteIndexType;
use crate::map::manager::WorldMapManager;

#[derive(Deserialize, Serialize)]
pub struct SerializedWorld {

    pub manager: WorldMapManager,

    pub npc_types: Vec<SerializedNPCType>,
    pub palettes: Vec<Palette>,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedNPC {

    pub id: NPCId,
    pub npc: NPC,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedNPCTypeConfig {

    pub identifier: TinyStr16,
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

pub type PaletteId = u8;

#[derive(Deserialize, Serialize)]
pub struct Palette {

    pub id: PaletteId,
    pub bottom: Vec<u8>,

}