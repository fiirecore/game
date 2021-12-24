use firecore_world::{
    character::npc::group::NpcGroupId, character::player::PlayerCharacter,
    serialized::SerializedTextures,
};
use hashbrown::HashMap;

use crate::engine::{error::ImageError, Context};

pub mod gui;
pub mod npc;
pub mod player;
pub mod tile;

pub struct ClientWorldData {
    pub tiles: tile::PaletteTextureManager,
    pub npc: npc::NpcTextures,
    pub player: player::PlayerTexture,
    pub gui: gui::GuiTextures,
}

impl ClientWorldData {
    pub fn new(
        ctx: &mut Context,
        textures: SerializedTextures,
        npcs: HashMap<NpcGroupId, Vec<u8>>,
    ) -> Result<Self, ImageError> {
        Ok(Self {
            tiles: tile::PaletteTextureManager::new(ctx, textures.palettes),
            npc: npc::NpcTextures::new(ctx, npcs)?,
            player: player::PlayerTexture::new(ctx, textures.player)?,
            gui: gui::GuiTextures::new(ctx),
        })
    }

    pub fn update(&mut self, delta: f32, player: &mut PlayerCharacter) {
        self.tiles.update(delta);
        self.player.update(delta, player);
    }
}
