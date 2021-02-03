use crate::util::graphics::Texture;

pub struct MovementTexture {

    textures: Vec<Texture>,
    index_map: Vec<(u8, bool)>,

    idle: (u8, bool),

}

impl MovementTexture {

    pub fn empty(idle: (u8, bool)) -> Self {

        Self {

            textures: Vec::new(),
            index_map: Vec::new(),

            idle: idle,

        }

    }

    pub fn push_texture(&mut self, texture: Texture) {
        self.textures.push(texture);
    }

    pub fn map_to_index(&mut self, texture_id: u8, flip: bool) {
        self.index_map.push((texture_id, flip));
    }

    pub fn len(&self) -> usize {
        return self.index_map.len();
    }

    pub fn texture(&self, index: usize) -> (Texture, bool) {
        let tuple = self.index_map[index];
        return (self.textures[tuple.0 as usize], tuple.1);
    }

    pub fn idle(&self) -> (Texture, bool) {
        return (self.textures[self.idle.0 as usize], self.idle.1);
    }

}