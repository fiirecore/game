use deps::tetra::Context;
use worldlib::serialized::{SerializedTextures, SerializedNPCType};

pub mod tile;
pub mod npc;
pub mod player;
pub mod gui;

pub struct WorldTextures {

    pub tiles: tile::TileTextureManager,
    pub npcs: npc::NPCTextureManager,
    pub player: player::PlayerTexture,
    pub gui: gui::GuiTextures,

}

impl WorldTextures {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            tiles: tile::TileTextureManager::default(),
            npcs: npc::NPCTextureManager::default(),
            player: player::PlayerTexture::new(ctx),
            gui: gui::GuiTextures::new(ctx),
        }
    }

    pub fn setup(&mut self, ctx: &mut Context, textures: SerializedTextures, npc_types: &Vec<SerializedNPCType>) {
        self.tiles.setup(ctx, textures);
        self.npcs.with_capacity(npc_types.len());
    }

}