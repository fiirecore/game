use serde::{Deserialize, Serialize};

use hashbrown::HashMap;

use crate::{
    character::{
        npc::{
            group::{NpcGroup, NpcGroupId},
            Npc, NpcId,
        },
        Movement,
    },
    map::{manager::Maps, PaletteId, TileId},
    // positions::Location,
};

// pub type MapGuiLocs = HashMap<crate::map::MapIcon, (String, Location)>;

type Texture = Vec<u8>;

#[derive(Deserialize, Serialize)]
pub struct SerializedWorld {
    pub maps: Maps,

    pub npcs: HashMap<NpcGroupId, SerializedNpcGroup>,
    // pub map_gui_locs: MapGuiLocs,
    pub textures: SerializedTextures,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedNpc {
    pub id: NpcId,
    pub npc: Npc,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedNpcGroup {
    pub group: NpcGroup,
    pub texture: Vec<u8>,
}

pub type SerializedPaletteMap = HashMap<PaletteId, SerializedPalette>;
pub type SerializedPlayerTexture = HashMap<Movement, Texture>;

#[derive(Deserialize, Serialize)]
pub struct SerializedTextures {
    pub palettes: SerializedPaletteMap,
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
