use opengl_graphics::Texture;

use crate::entity::entity::Entity;
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

    pub fn new(movement_texture: MovementTexture, counter: usize) -> Self {

        let mut timer =  Timer::new(counter);
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

    fn update(&mut self) {
        self.timer.update();
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

    fn texture(&self) -> (&Texture, bool) {
        if self.idle {
            return self.textures.idle();
        } else {
            return self.textures.texture(self.index as usize);
        }
        
    }

}