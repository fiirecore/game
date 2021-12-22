use firecore_world::{
    character::player::PlayerCharacter,
    serialized::{Animated, Palettes, Player, SerializedNpcType},
};

use crate::engine::{error::ImageError, Context};

pub mod gui;
pub mod npc;
pub mod player;
pub mod tile;

pub struct ClientWorldData {
    pub tiles: tile::TileTextureManager,
    pub npc: npc::NpcData,
    pub player: player::PlayerTexture,
    pub gui: gui::GuiTextures,
}

impl ClientWorldData {
    pub fn new(
        ctx: &mut Context,
        palettes: Palettes,
        animated: Animated,
        npcs: Vec<SerializedNpcType>,
        player: Player,
    ) -> Result<Self, ImageError> {
        Ok(Self {
            tiles: tile::TileTextureManager::new(ctx, palettes, animated),
            npc: npc::NpcData::new(ctx, npcs)?,
            player: player::PlayerTexture::new(ctx, player)?,
            gui: gui::GuiTextures::new(ctx),
        })
    }

    pub fn update(&mut self, delta: f32, player: &mut PlayerCharacter) {
        self.tiles.update(delta);
        self.player.update(delta, player);
    }
}
