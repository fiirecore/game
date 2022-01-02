use firecore_world::{character::player::PlayerCharacter, serialized::SerializedTextures};

use crate::engine::{error::ImageError, Context};

pub mod gui;
pub mod npc;
pub mod object;
pub mod player;
pub mod tile;

pub struct ClientWorldData {
    pub tiles: tile::PaletteTextureManager,
    pub npc: npc::NpcTextures,
    pub object: object::ObjectTextures,
    pub player: player::PlayerTexture,
    pub gui: gui::GuiTextures,
}

impl ClientWorldData {
    pub fn new(ctx: &mut Context, textures: SerializedTextures) -> Result<Self, ImageError> {
        Ok(Self {
            tiles: tile::PaletteTextureManager::new(ctx, textures.palettes),
            npc: npc::NpcTextures::new(ctx, textures.npcs)?,
            object: object::ObjectTextures::new(ctx, textures.objects)?,
            player: player::PlayerTexture::new(ctx, textures.player)?,
            gui: gui::GuiTextures::new(ctx)?,
        })
    }

    pub fn update(&mut self, delta: f32, player: &mut PlayerCharacter) {
        self.tiles.update(delta);
        self.player.update(delta, player);
        self.object.update(delta);
    }
}
