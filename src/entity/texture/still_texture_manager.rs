use crate::util::texture::Texture;

use super::texture_manager::TextureManager;

pub struct StillTextureManager {

    texture: Texture,
    flip: bool,

}

impl StillTextureManager {

    pub fn new(texture: Texture, flip: bool) -> Self {
        Self {
            texture: texture,
            flip: flip,
        }
    }

}

impl TextureManager for StillTextureManager {

    fn reset(&mut self) {}

    fn update(&mut self, _delta: f32) {}

    fn idle(&mut self) {}

    fn unidle(&mut self) {}

    fn is_idle(&self) -> bool {
        true
    }

    fn texture(&self) -> (Texture, bool) {
        (self.texture, self.flip)
    }
}