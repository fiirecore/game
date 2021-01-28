use macroquad::prelude::draw_texture;
use macroquad::prelude::draw_texture_ex;

use crate::util::texture::Texture;

use super::TILE_SIZE;

pub static TEXTURE_SIZE: u8 = TILE_SIZE;

pub fn draw(texture: Texture, x: f32, y: f32) {
	draw_texture(texture, x, y, macroquad::prelude::WHITE);
}

pub fn draw_o(texture: Option<&Texture>, x: f32, y: f32) {
	if let Some(texture) = texture {
		draw(*texture, x, y);
	}
}

pub fn draw_flip(texture: Texture, x: f32, y: f32, flip: bool) {
	if flip {
		draw_texture_ex(texture, x + texture.width(), y, macroquad::prelude::WHITE, macroquad::prelude::DrawTextureParams {
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

//pub fn render_title_in() {
//	
//}

//pub fn gloss_over() {
//	
//}
//
//pub fn slide_in() {
//	
//}

/*

#[allow(clippy::too_many_arguments, dead_code)]
pub fn fade_out_o(c: &mut Context, g: &mut G2d, tex: &Option<Texture>, x: u32, y: u32, start_time: Instant, offset: u64, duration: u32, red: u8, green: u8, blue: u8) {
	
	g.clear_color([red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0, 1f32]);
	
	let fade = if start_time.elapsed().unwrap().as_millis() > offset as u128 { 1f32 - (start_time.elapsed().unwrap().as_millis() as f32 - offset as f32) / duration as f32 } else { 1f32 };
	
	Image::new_color([1f32, 1f32, 1f32, fade]).draw(
		tex.iter().next().unwrap(),
		&DrawState::default(),
		c.trans(x as f64, y as f64).transform,
		g
	);
	
}

*/

pub fn fade_in_out(texture: Texture, x: f32, y: f32, accumulator: f32, end_time: f32, fade_time: f32) {
	if accumulator < fade_time {
		draw_texture(texture, x, y, [1.0, 1.0, 1.0, accumulator/fade_time].into());
	} else if accumulator < end_time - fade_time {
		draw(texture, x, y);
	} else if accumulator < end_time {
		draw_texture(texture, x, y, [1.0, 1.0, 1.0, (end_time - accumulator) / fade_time].into());
	}
}

