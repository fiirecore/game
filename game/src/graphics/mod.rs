use firecore_util::text::Message;
use macroquad::prelude::{Texture2D, Image, load_texture_from_image, set_texture_filter, FilterMode::Nearest, draw_texture, Color, WHITE};

use text::IntoMQColor;
use text::TEXT_RENDERER;

pub mod text;

pub const DRAW_COLOR: Color = WHITE;

pub fn byte_texture(bytes: &[u8]) -> Texture2D {
	image_texture(&Image::from_file_with_format(bytes, None))
}

pub fn image_texture(image: &Image) -> Texture2D {
	let texture = load_texture_from_image(image);
	set_texture_filter(texture, Nearest);
	texture
}

pub fn debug_texture() -> Texture2D {
	byte_texture(include_bytes!("../../assets/missing_texture.png"))
}

pub fn draw(texture: Texture2D, x: f32, y: f32) {
	draw_texture(texture, x, y, DRAW_COLOR);
}

pub fn draw_bottom(texture: Texture2D, x: f32, y: f32) {
	draw(texture, x, y - texture.height());
}

pub fn draw_o(texture: Option<Texture2D>, x: f32, y: f32) {
	if let Some(texture) = texture {
		draw(texture, x, y);
	}
}

pub fn draw_o_bottom(texture: Option<Texture2D>, x: f32, y: f32) {
	if let Some(texture) = texture {
		draw_bottom(texture, x, y);
	}
}

pub fn draw_message(message: Message, x: f32, y: f32) {
	if let Some(renderer) = unsafe{TEXT_RENDERER.as_ref()} {
		for y_offset in 0..message.lines.len() {
			renderer.render_text_from_left(message.font_id, &message.lines[y_offset], message.color.into_color(), x, y + (y_offset * 15) as f32);
		}
	}	
}

pub fn draw_text_left(font_id: u8, text: &str, color: impl IntoMQColor, x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.render_text_from_left(font_id, text, color.into_color(), x, y);
	}
}

pub fn draw_text_right(font_id: u8, text: &str, color: impl IntoMQColor, x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.render_text_from_right(font_id, text, color.into_color(), x, y);
	}
}

pub fn draw_cursor(x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.render_cursor(x, y);
	}
}

pub fn draw_button(text: &str, font_id: u8, x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.render_button(text, font_id, x, y)
	}
}

pub fn fade_in_out(texture: Texture2D, x: f32, y: f32, accumulator: f32, end_time: f32, fade_time: f32) {
	if accumulator < fade_time {
		draw_texture(texture, x, y, [1.0, 1.0, 1.0, accumulator/fade_time].into());
	} else if accumulator < end_time - fade_time {
		draw(texture, x, y);
	} else if accumulator < end_time {
		draw_texture(texture, x, y, [1.0, 1.0, 1.0, (end_time - accumulator) / fade_time].into());
	}
}

pub fn fade_in(texture: Texture2D, x: f32, y: f32, accumulator: f32, fade_time: f32) {
	if accumulator < fade_time {
		draw_texture(texture, x, y, [1.0, 1.0, 1.0, accumulator/fade_time].into());
	} else {
		draw(texture, x, y);
	}
}