use macroquad::prelude::Color;
pub use macroquad::prelude::Texture2D as Texture;
use macroquad::prelude::WHITE;
use macroquad::prelude::draw_texture;

use crate::io::data::text::Message;

use super::text::TextRenderer;

pub mod texture;

lazy_static::lazy_static! {
	static ref TEXT_RENDERER: TextRenderer = TextRenderer::new();
}

pub fn draw(texture: Texture, x: f32, y: f32) {
	draw_texture(texture, x, y, WHITE);
}

pub fn draw_o(texture: Option<&Texture>, x: f32, y: f32) {
	if let Some(texture) = texture {
		draw(*texture, x, y);
	}
}

pub fn draw_flip(texture: Texture, x: f32, y: f32, flip: bool) {
	if flip {
		macroquad::prelude::draw_texture_ex(texture, x + texture.width(), y, WHITE, macroquad::prelude::DrawTextureParams {
			dest_size: Some(macroquad::prelude::vec2(texture.width() * -1.0, texture.height())),
			..Default::default()
		});
	} else {
		draw(texture, x, y);
	}
	
}

pub fn draw_o_bottom(texture: Option<Texture>, x: f32, y: f32) {
	if let Some(texture) = texture {
		draw_bottom(texture, x, y);
	}
}

pub fn draw_bottom(texture: Texture, x: f32, y: f32) {
	draw(texture, x, y - texture.height());
}

pub fn draw_rect<C: Into<macroquad::prelude::Color>>(color: C, x: f32, y: f32, width: u32, height: u32) {
	macroquad::prelude::draw_rectangle(x, y, width as f32, height as f32, color.into());
}

pub fn draw_message(message: Message, x: f32, y: f32) {
	for y_offset in 0..message.message.len() {
		TEXT_RENDERER.render_text_from_left(message.font_id, &message.message[y_offset], message.color.into(), x, y + (y_offset * 15) as f32);
	}
}

pub fn draw_text_left(font_id: usize, text: &str, x: f32, y: f32) {
	TEXT_RENDERER.render_text_from_left(font_id, text, WHITE, x, y);
}

pub fn draw_text_left_color(font_id: usize, text: &str, color: impl Into<Color>, x: f32, y: f32) {
	TEXT_RENDERER.render_text_from_left(font_id, text, color.into(), x, y);
}

//#[deprecated(since = "0.2.1", note = "Use draw_message instead")]
pub fn draw_text_right(font_id: usize, text: &str, x: f32, y: f32) {
	TEXT_RENDERER.render_text_from_right(font_id, text, WHITE, x, y);
}

pub fn draw_cursor(x: f32, y: f32) {
	TEXT_RENDERER.render_cursor(x, y);
}

pub fn draw_button(text: &str, font_id: usize, x: f32, y: f32) {
	TEXT_RENDERER.render_button(text, font_id, x, y)
}

pub fn fade_in_out(texture: Texture, x: f32, y: f32, accumulator: f32, end_time: f32, fade_time: f32) {
	if accumulator < fade_time {
		draw_texture(texture, x, y, [1.0, 1.0, 1.0, accumulator/fade_time].into());
	} else if accumulator < end_time - fade_time {
		draw(texture, x, y);
	} else if accumulator < end_time {
		draw_texture(texture, x, y, [1.0, 1.0, 1.0, (end_time - accumulator) / fade_time].into());
	}
}
