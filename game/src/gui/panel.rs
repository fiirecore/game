use macroquad::prelude::Rect;
use macroquad::prelude::Vec2;
use macroquad::prelude::draw_rectangle;
use macroquad::prelude::{Texture2D, draw_texture_ex, WHITE, DrawTextureParams};

use crate::graphics::byte_texture;

static mut PANEL: Option<Texture2D> = None;

pub struct Panel {

    corner: Texture2D,
    // bottom_right_color: Texture2D,
    // size: Vec2,

}

impl Panel {

    pub fn new() -> Self {
        Self {
            corner: unsafe { *PANEL.get_or_insert(byte_texture(include_bytes!("../../assets/gui/panel.png"))) },
        }
    }

    pub fn render(&self, x: f32, y: f32, w: f32, h: f32) {

        draw_texture_ex(self.corner, x, y, WHITE, DrawTextureParams::default());
        let x1 = x + w - 7.0;
        draw_texture_ex(self.corner, x1, y, WHITE, DrawTextureParams {
            flip_x: true,
            ..Default::default()
        });

        let y1 = y + h - 7.0;

        draw_texture_ex(self.corner, x, y1, WHITE, DrawTextureParams {
            flip_y: true,
            ..Default::default()
        });

        draw_texture_ex(self.corner, x1, y1, WHITE, DrawTextureParams {
            flip_x: true,
            flip_y: true,
            ..Default::default()
        });

        let w = w - 14.0;
        let h = h - 14.0;

        draw_rectangle(x + 7.0, y + 7.0, w, h, crate::graphics::WHITE);

        draw_texture_ex(self.corner, x + 7.0, y, WHITE, DrawTextureParams {
            source: Some(Rect::new(6.0, 0.0, 1.0, 7.0)),
            dest_size: Some(Vec2::new(w, 7.0)),
            ..Default::default()
        });

        draw_texture_ex(self.corner, x, y + 7.0, WHITE, DrawTextureParams {
            source: Some(Rect::new(0.0, 6.0, 7.0, 1.0)),
            dest_size: Some(Vec2::new(7.0, h)),
            ..Default::default()
        });

        draw_texture_ex(self.corner, x1, y + 7.0, WHITE, DrawTextureParams {
            source: Some(Rect::new(0.0, 6.0, 7.0, 1.0)),
            dest_size: Some(Vec2::new(7.0, h)),
            flip_x: true,
            ..Default::default()
        });

        draw_texture_ex(self.corner, x + 7.0, y1, WHITE, DrawTextureParams {
            source: Some(Rect::new(6.0, 0.0, 1.0, 7.0)),
            dest_size: Some(Vec2::new(w, 7.0)),
            flip_y: true,
            ..Default::default()
        });

    }

    pub fn render_text(&self, x: f32, y: f32, w: f32, text: &[&str], cursor: usize) {
        self.render(x, y, w, 22.0 + (text.len() << 4) as f32);
        for (index, text) in text.iter().enumerate() {
            crate::graphics::draw_text_left(1, text, crate::text::TextColor::Black, x + 15.0, y + 11.0 + (index << 4) as f32);
        }
        crate::graphics::draw_cursor(x + 8.0, y + 13.0 + (cursor << 4) as f32);
    }

}