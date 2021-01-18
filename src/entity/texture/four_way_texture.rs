use opengl_graphics::Texture;

use super::movement_texture_manager::MovementTextureManager;
use super::texture_manager::TextureManager;

pub struct FourWayMovingTexture {

    direction: u8,
    textures: Vec<MovementTextureManager>, // 0 = Up, 1 = Down, 2 = Left, 3 = Right

}

impl FourWayMovingTexture {

    pub fn new() -> Self {

        Self {

            direction: 0,
            textures: Vec::new(),

        }

    }

    pub fn add_movement_texture(&mut self, movement_texture: MovementTextureManager) {
        self.textures.push(movement_texture);
    }

    pub fn update_with_direction(&mut self, direction: u8) {
        if self.direction != direction {
            self.direction = direction;
            self.textures[self.direction as usize].reset();
        } else {
            self.update();
        }
    }

}

impl TextureManager for FourWayMovingTexture {

    fn reset(&mut self) {
        self.textures[self.direction as usize].reset();
    }

    fn update(&mut self) {
        self.textures[self.direction as usize].update();
    }

    fn idle(&mut self) {
        self.textures[self.direction as usize].idle();
    }

    fn unidle(&mut self) {
        self.textures[self.direction as usize].unidle();
    }

    fn is_idle(&self) -> bool {
        self.textures[self.direction as usize].is_idle()
    }

    fn texture(&self) -> (&Texture, bool) {
        return self.textures[self.direction as usize].texture();
    }

}