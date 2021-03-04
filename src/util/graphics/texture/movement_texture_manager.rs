use crate::util::graphics::Texture;

use crate::util::Entity;
use crate::util::timer::Timer;

use super::movement_texture::MovementTexture;
use super::texture_manager::TextureManager;

pub struct MovementTextureManager {

    timer: Timer,

    textures: MovementTexture,
    index: u8,

    idle: bool,

}

impl MovementTextureManager {

    pub fn new(movement_texture: MovementTexture, seconds: f32) -> Self {

        let mut timer =  Timer::new(seconds);
        timer.spawn();

        Self {

            timer: timer,
            textures: movement_texture,
            index: 0,

            idle: false,

        }

    }

}

impl TextureManager for MovementTextureManager {

    fn reset(&mut self) {
        self.index = 0;
        self.timer.reset();
    }

    fn update(&mut self, delta: f32) {
        self.timer.update(delta);
        if self.timer.is_finished() {
            self.timer.reset();
            self.index += 1;
            if self.index == self.textures.len() as u8 {
                self.index = 0;
            }
        }     
    }

    fn idle(&mut self) {
        self.idle = true;
    }

    fn unidle(&mut self) {
        self.idle = false;
    }

    fn is_idle(&self) -> bool {
        self.idle
    }

    fn texture(&self) -> (Texture, bool) {
        if self.idle {
            return self.textures.idle();
        } else {
            return self.textures.texture(self.index as usize);
        }
        
    }

}