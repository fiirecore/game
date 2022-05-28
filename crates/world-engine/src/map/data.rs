use firecore_world::{character::player::PlayerCharacter, serialized::SerializedTextures};
use pokengine::engine::{graphics::Graphics, notan::draw::Font};

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
    pub debug_font: Font,
}

impl ClientWorldData {
    pub fn new(
        gfx: &mut Graphics,
        textures: SerializedTextures,
        debug_font: Font,
    ) -> Result<Self, String> {
        Ok(Self {
            tiles: tile::PaletteTextureManager::new(gfx, textures.palettes)?,
            npc: npc::NpcTextures::new(gfx, textures.npcs)?,
            object: object::ObjectTextures::new(gfx, textures.objects)?,
            player: player::PlayerTexture::new(gfx, textures.player)?,
            gui: gui::GuiTextures::new(gfx)?,
            debug_font,
        })
    }

    pub fn update<P, B: Default>(&mut self, delta: f32, player: &mut PlayerCharacter<P, B>) {
        self.tiles.update(delta);
        self.player.update(delta, player);
        self.object.update(delta);
    }
}
