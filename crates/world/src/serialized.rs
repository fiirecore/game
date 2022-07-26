use enum_map::EnumMap;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

pub extern crate enum_map;

use crate::{
    character::{
        npc::{Npc, NpcId},
        Activity, CharacterGroupId,
    },
    map::{object::ObjectType, PaletteId, TileId},
    // positions::Location,
};

// pub type MapGuiLocs = HashMap<crate::map::MapIcon, (String, Location)>;

type Texture = Vec<u8>;

// #[derive(Deserialize, Serialize)]
// pub struct SerializedWorld {
//     // pub data: WorldMapData,
//     // pub scripts: WorldScriptData,
//     // pub map_gui_locs: MapGuiLocs,
//     // pub textures: SerializedTextures,
// }

#[derive(Deserialize, Serialize)]
pub struct SerializedNpc {
    pub id: NpcId,
    pub npc: Npc,
}

pub type SerializedPaletteMap = HashMap<PaletteId, SerializedPalette>;
pub type SerializedPlayerTexture = EnumMap<Activity, Texture>;
pub type SerializedCharacterGroupTextures = HashMap<CharacterGroupId, Texture>;
pub type SerializedObjectTextures = HashMap<ObjectType, Texture>;

#[derive(Deserialize, Serialize)]
pub struct SerializedTextures {
    pub palettes: SerializedPaletteMap,
    pub npcs: SerializedCharacterGroupTextures,
    pub objects: SerializedObjectTextures,
    pub player: SerializedPlayerTexture,
}

pub type SerializedAnimatedTexture = HashMap<TileId, Texture>;
pub type SerializedDoors = HashMap<TileId, Texture>;

#[derive(Deserialize, Serialize)]
pub struct SerializedPalette {
    pub texture: Texture,
    pub animated: SerializedAnimatedTexture,
    pub doors: SerializedDoors,
}
