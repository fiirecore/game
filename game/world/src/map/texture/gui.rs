use game::{
    deps::hash::HashMap,
    macroquad::prelude::Texture2D,
    graphics::byte_texture,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GuiTexture {
    Condition,
}

pub struct GuiTextures {
    textures: HashMap<GuiTexture, Texture2D>,
}

impl GuiTextures {

    pub fn get(&self, texture: &GuiTexture) -> Texture2D {
        *self.textures.get(texture).unwrap_or_else(|| panic!("Could not get texture for GUI texture {:?}", texture))
    }

}

impl Default for GuiTextures {
    fn default() -> Self {
        let mut map = HashMap::with_capacity(1);
        map.insert(GuiTexture::Condition, byte_texture(include_bytes!("../../../assets/gui/world/condition.png")));
        Self {
            textures: map,
        }
    }
}