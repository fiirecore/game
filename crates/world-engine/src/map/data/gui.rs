use crate::engine::{graphics::{Texture, Graphics}, HashMap};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GuiTexture {
    Condition,
}

pub struct GuiTextures {
    textures: HashMap<GuiTexture, Texture>,
}

impl GuiTextures {
    pub fn get(&self, texture: &GuiTexture) -> &Texture {
        self.textures
            .get(texture)
            .unwrap_or_else(|| panic!("Could not get texture for GUI texture {:?}", texture))
    }
}

impl GuiTextures {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        let mut map = HashMap::with_capacity(1);
        map.insert(
            GuiTexture::Condition,
            gfx.create_texture()
                .from_image(include_bytes!("../../../assets/textures/gui/condition.png"))
                .build()?,
        );
        Ok(Self { textures: map })
    }
}
