use crate::util::texture::Texture;

pub trait TextureManager {

    fn reset(&mut self);

    fn update(&mut self, delta: f32);

    fn idle(&mut self);

    fn unidle(&mut self);

    fn is_idle(&self) -> bool;

    fn texture(&self) -> (Texture, bool);

}