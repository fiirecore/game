use firecore_game::macroquad::prelude::DrawTextureParams;
use firecore_game::macroquad::prelude::Image;
use firecore_game::macroquad::prelude::Rect;
use firecore_game::macroquad::prelude::draw_texture_ex;

use crate::world::TileId;
use crate::util::{TILE_SIZE, graphics::Texture};

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
        draw_texture_ex(self.texture_atlas, x, y, firecore_game::macroquad::prelude::WHITE, DrawTextureParams {
            source: Some(Rect::new(tex_x, tex_y, TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        });        
    }

}