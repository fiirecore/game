use deps::tetra::Context;
use worldlib::serialized::SerializedTextures;

pub mod tile;
pub mod npc;
pub mod player;
pub mod gui;

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

    pub fn setup(&mut self, ctx: &mut Context, textures: SerializedTextures) {
        self.tiles.setup(ctx, textures);
    }

}