use engine::{
    tetra::{
        Context,
        graphics::Texture,
    },
    graphics::byte_texture,
};

use 
deps::hash::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GuiTexture {
    Condition,
}

pub struct GuiTextures {
    textures: HashMap<GuiTexture, Texture>,
}

impl GuiTextures {

    pub fn get(&self, texture: &GuiTexture) -> &Texture {
        self.textures.get(texture).unwrap_or_else(|| panic!("Could not get texture for GUI texture {:?}", texture))
    }

}

impl GuiTextures {
    pub fn new(ctx: &mut Context) -> Self {
        let mut map = HashMap::with_capacity(1);
        map.insert(GuiTexture::Condition, byte_texture(ctx, include_bytes!("../../../../assets/world/gui/world/condition.png")));
        Self {
            textures: map,
        }
    }
}