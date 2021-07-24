use engine::tetra::Context;
use worldlib::serialized::SerializedTextures;

use super::warp::WarpTransition;

pub mod gui;
pub mod npc;
pub mod player;
pub mod tile;

pub struct WorldTextures {
    pub tiles: tile::TileTextureManager,
    pub npcs: npc::NpcTextureManager,
    pub player: player::PlayerTexture,
    pub gui: gui::GuiTextures,
}

impl WorldTextures {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            tiles: tile::TileTextureManager::default(),
            npcs: npc::NpcTextureManager::default(),
            player: player::PlayerTexture::new(ctx),
            gui: gui::GuiTextures::new(ctx),
        }
    }

    pub fn setup(
        &mut self,
        ctx: &mut Context,
        warper: &mut WarpTransition,
        textures: SerializedTextures,
    ) {
        self.tiles.setup(ctx, warper, textures);
    }
}
