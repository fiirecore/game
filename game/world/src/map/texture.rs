use firecore_world_lib::serialized::SerializedNPCType;
use firecore_world_lib::serialized::SerializedTextures;

pub mod tile;
pub mod npc;
pub mod player;
pub mod gui;

#[derive(Default)]
pub struct WorldTextures {

    pub tiles: tile::TileTextureManager,
    pub npcs: npc::NPCTextureManager,
    pub player: player::PlayerTexture,
    pub gui: gui::GuiTextures,

}

impl WorldTextures {

    pub fn setup(&mut self, textures: SerializedTextures, npc_types: &Vec<SerializedNPCType>) {
        self.tiles.setup(textures);
        self.npcs.with_capacity(npc_types.len());
        self.player.load();
    }

}