use crate::util::Direction;
use crate::util::graphics::Texture;

use super::texture_manager::TextureManager;

pub struct ThreeWayTexture {

    pub direction: u8,
    textures: Vec<Box<dyn TextureManager>>, // 0 = Up, 1 = Down, 2 = Left, 3 = Right

}

impl ThreeWayTexture {

    pub fn new() -> Self {

        Self {

            direction: 0,
            textures: Vec::new(),

        }

    }

    pub fn add_texture_manager(&mut self, texture_manager: Box<dyn TextureManager>) {
        self.textures.push(texture_manager);
    }

    pub fn update_with_direction(&mut self, delta: f32, direction: Direction) {
        let direction = direction.value();
        if self.direction != direction {
            self.direction = direction;
            self.reset();       
        } else {
            self.update(delta);          
        }
    }

    pub fn of_direction(&self, direction: Direction) -> (Texture, bool) {
        let direction = direction.value();
        if direction == 3 {
            let tuple = self.textures[2].texture();
            return (tuple.0, !tuple.1);
        } else {
            return self.textures[direction as usize].texture();
        }
    }

}

impl TextureManager for ThreeWayTexture {

    fn reset(&mut self) {
        if self.direction != 3 {
            self.textures[self.direction as usize].reset();
        } else {
            self.textures[2].reset();
        }        
    }

    fn update(&mut self, delta: f32) {
        if self.direction != 3 {
            self.textures[self.direction as usize].update(delta);
        } else {
            self.textures[2].update(delta);
        } 
    }

    fn idle(&mut self) {
        if self.direction != 3 {
            self.textures[self.direction as usize].idle();
        } else {
            self.textures[2].idle();
        } 
    }

    fn unidle(&mut self) {
        if self.direction != 3 {
            self.textures[self.direction as usize].unidle();
        } else {
            self.textures[2].unidle();
        } 
    }

    fn is_idle(&self) -> bool {
        if self.direction != 3 {
            self.textures[self.direction as usize].is_idle()
        } else {
            self.textures[2].is_idle()
        }
    }

    fn texture(&self) -> (Texture, bool) {
        if self.direction != 3 {
            return self.textures[self.direction as usize].texture();
        } else {
            let mut tuple = self.textures[2].texture();
            tuple.1 = !tuple.1;
            return tuple;
        }
    }

}