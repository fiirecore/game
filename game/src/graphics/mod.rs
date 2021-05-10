use macroquad::prelude::Image;
use macroquad::prelude::{Texture2D, FilterMode::Nearest, draw_texture, Color, color_u8, WHITE as MQWHITE};

use firecore_text::{TEXT_RENDERER, IntoMQColor};

pub const WHITE: Color = color_u8!(248, 248, 248, 255);

pub const DRAW_COLOR: Color = MQWHITE;

pub fn byte_texture(bytes: &[u8]) -> Texture2D { // To - do: consider moving this, image_texture, and filter to a different module
	filter(Texture2D::from_file_with_format(bytes, None))
}

pub fn image_texture(image: &Image) -> Texture2D {
	filter(Texture2D::from_image(image))
}

pub fn filter(texture: Texture2D) -> Texture2D {
	texture.set_filter(Nearest);
	texture
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

// #[deprecated]
// pub fn draw_message(message: &Message, x: f32, y: f32) {
// 	if let Some(renderer) = unsafe{TEXT_RENDERER.as_ref()} {
// 		_draw_message(renderer, message.font, &message.message_set[0], message.color.into_color(), x, y)
// 	}	
// }

// fn _draw_message(renderer: &firecore_text::TextRenderer, font_id: u8, message: &MessagePage, color: Color, x: f32, y: f32) {
// 	message.lines.iter().enumerate().for_each(|(index, line)| renderer.render_text_from_left(font_id, line, color, x, y + (index << 4) as f32));
// }

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

pub fn draw_text_center(font_id: u8, text: &str, color: impl IntoMQColor, x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.render_text_from_center(font_id, text, color.into_color(), x, y);
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
		draw_texture(texture, x, y, Color::new(1.0, 1.0, 1.0, accumulator / fade_time));
	} else if accumulator < end_time - fade_time {
		draw(texture, x, y);
	} else if accumulator < end_time {
		draw_texture(texture, x, y, Color::new(1.0, 1.0, 1.0, (end_time - accumulator) / fade_time));
	}
}

pub fn fade_in(texture: Texture2D, x: f32, y: f32, accumulator: f32, fade_time: f32) {
	if accumulator < fade_time {
		draw_texture(texture, x, y, Color::new(1.0, 1.0, 1.0, accumulator / fade_time));
	} else {
		draw(texture, x, y);
	}
}