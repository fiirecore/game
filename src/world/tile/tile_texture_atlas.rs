use macroquad::prelude::DrawTextureParams;
use macroquad::prelude::Image;
use macroquad::prelude::Rect;
use macroquad::prelude::draw_texture_ex;

use crate::world::TileId;
use crate::util::graphics::Texture;

const TILE_SIZE: f32 = crate::util::TILE_SIZE as f32;

pub struct TileTextureAtlas {

    offset: TileId,
    texture_atlas: Texture,

    width: u16,

}

impl TileTextureAtlas {

    pub fn new(image: Image, palette: u8, palette_sizes: &HashMap<u8, u16>) {

    }

    pub fn render_tile(&self, index: TileId, x: f32, y: f32) {
        let index = index - self.offset;
        let tex_x = (index % self.width) as f32;
        let tex_y = (index / self.width) as f32;
        draw_texture_ex(self.texture_atlas, x, y, macroquad::prelude::WHITE, DrawTextureParams {
            source: Some(Rect::new(tex_x, tex_y, TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        });        
    }

}