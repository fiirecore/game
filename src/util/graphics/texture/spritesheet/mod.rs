use macroquad::prelude::Color;
use macroquad::prelude::DrawTextureParams;
use macroquad::prelude::Rect;
use macroquad::prelude::Texture2D;

pub struct SpriteSheet {

    pub texture: Texture2D,
    pub indexes: Vec<SpriteIndex>,
    pub width: u16,
    pub height: u16,

}

pub struct SpriteIndex {

}

pub fn draw_sheet(spritesheet: SpriteSheet, index: usize, x: f32, y: f32, color: Color) {
    macroquad::prelude::draw_texture_ex(spritesheet.texture, x, y, color, DrawTextureParams {
        source: Some(Rect::new(spritesheet.width, spritesheet.height)),
        flip_x: spritesheet.indexes[index].flip
        ..Default::default()
    })
}
