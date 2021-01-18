use opengl_graphics::Texture;

pub trait TextureManager {

    fn reset(&mut self);

    fn update(&mut self);

    fn idle(&mut self);

    fn unidle(&mut self);

    fn is_idle(&self) -> bool;

    fn texture(&self) -> (&Texture, bool);

}