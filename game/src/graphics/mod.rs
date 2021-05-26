use deps::tetra::{
	Context,
	graphics::{
		Texture,
		DrawParams,
		Color,
	},
	math::Vec2,
};

use firecore_text::FontId;
use firecore_text::{TEXT_RENDERER, IntoMQColor};

pub const LIGHTGRAY: Color = Color::rgb(0.78, 0.78, 0.78);
pub const GRAY: Color = Color::rgb(0.51, 0.51, 0.51);
pub const RED: Color = Color::rgb(0.90, 0.16, 0.22);
pub const DARKBLUE: Color = Color::rgb(0.00, 0.32, 0.67);

pub fn byte_texture(ctx: &mut Context, bytes: &[u8]) -> Texture {
	Texture::from_file_data(ctx, bytes).unwrap()
}

pub const ZERO: Vec2<f32> = Vec2::new(0.0, 0.0);

pub const fn position(x: f32, y: f32) -> DrawParams {
	DrawParams {
		position: Vec2::new(x, y),
		scale: Vec2::new(1.0, 1.0),
		origin: ZERO,
		rotation: 0.0,
		color: Color::WHITE,
	}
}

#[inline]
pub fn flip_x(params: DrawParams) -> DrawParams {
	params.scale(Vec2::new(-1.0, 1.0))
}

#[inline]
pub fn flip_y(params: DrawParams) -> DrawParams {
	params.scale(Vec2::new(1.0, -1.0))
}

#[inline]
pub fn draw_bottom(ctx: &mut Context, texture: &Texture, x: f32, y: f32) {
	texture.draw(ctx, position(x, y - texture.height() as f32));
}

#[inline]
pub fn draw_o(ctx: &mut Context, texture: Option<&Texture>, x: f32, y: f32) {
	if let Some(texture) = texture {
		texture.draw(ctx, position(x, y));
	}
}

#[inline]
pub fn draw_o_bottom(ctx: &mut Context, texture: Option<&Texture>, x: f32, y: f32) {
	if let Some(texture) = texture {
		draw_bottom(ctx, texture, x, y);
	}
}

static mut WHITE_TEXTURE: Option<Texture> = None;

#[deprecated]
fn new_white_texture(ctx: &mut Context) {
	if unsafe { WHITE_TEXTURE.is_none() } {
		unsafe { WHITE_TEXTURE = Some(byte_texture(ctx, include_bytes!("../../assets/white.png"))) }		
	}
}

fn white_texture() -> &'static Texture {
	unsafe { WHITE_TEXTURE.as_ref().unwrap() }
}


pub fn draw_rectangle(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, color: Color) {
	new_white_texture(ctx);
	let texture = white_texture();
	deps::tetra::graphics::set_texture(ctx, texture);
	deps::tetra::graphics::push_quad(ctx, x, y, x + w, y + h, 0.0, 0.0, 1.0, 1.0, &DrawParams::default().color(color))
}


pub fn draw_rectangle_lines(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, thickness: f32, color: Color) {
    draw_line_ex(ctx, x, y, w, true, thickness, color);
	draw_line_ex(ctx, x, y, h, false, thickness, color);
	draw_line_ex(ctx, x, y + h, w, true, thickness, color);
	draw_line_ex(ctx, x + w, y, h, false, thickness, color);
}

pub fn draw_line(ctx: &mut Context, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
	let horizontal = x1 != x2;
	let len = if horizontal {
		x2 - x1
	} else {
		y2 - y1
	};
	draw_line_ex(ctx, x1, y1, len, horizontal, thickness, color)
}


pub fn draw_line_ex(ctx: &mut Context, x: f32, y: f32, len: f32, horizontal: bool, thickness: f32, color: Color) {
	let (x, y, w, h) = if horizontal {
		(x, y - thickness / 2.0, len, thickness)
	} else {
		(x - thickness / 2.0, y, thickness, len)
	};
	draw_rectangle(ctx, x, y, w, h, color)
}

#[allow(unused_variables)]
pub fn draw_circle(ctx: &mut Context, x: f32, y: f32, r: f32, color: Color) {
	todo!("draw circle")
}

// #[deprecated]
// pub fn draw_message(message: &Message, x: f32, y: f32) {
// 	if let Some(renderer) = unsafe{TEXT_RENDERER.as_ref()} {
// 		_draw_message(renderer, message.font, &message.message_set[0], message.color.into_color(), x, y)
// 	}	
// }

// fn _draw_message(renderer: &firecore_text::TextRenderer, font_id: u8, message: &MessagePage, color: Color, x: f32, y: f32) {
// 	message.lines.iter().enumerate().for_each(|(index, line)| renderer.draw_text_left(font_id, line, color, x, y + (index << 4) as f32));
// }

pub fn draw_text_left(ctx: &mut Context, font: &FontId, text: &str, color: impl IntoMQColor, x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.draw_text_left(ctx, font, text, color.into_color(), x, y);
	}
}

pub fn draw_text_right(ctx: &mut Context, font: &FontId, text: &str, color: impl IntoMQColor, x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.draw_text_right(ctx, font, text, color.into_color(), x, y);
	}
}

pub fn draw_text_center(ctx: &mut Context, font: &FontId, text: &str, color: impl IntoMQColor, x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.draw_text_center(ctx, font, text, color.into_color(), x, y);
	}
}

pub fn draw_cursor(ctx: &mut Context, x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.draw_cursor(ctx, x, y);
	}
}

pub fn draw_button(ctx: &mut Context, font: &FontId, text: &str, x: f32, y: f32) {
	if let Some(renderer) = unsafe {TEXT_RENDERER.as_ref()} {
		renderer.draw_button(ctx, font, text, x, y)
	}
}

pub fn fade_in_out(ctx: &mut Context, texture: &Texture, x: f32, y: f32, accumulator: f32, end_time: f32, fade_time: f32) {
	let position = position(x, y);
	if accumulator < fade_time {
		texture.draw(ctx, position.color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time)));
	} else if accumulator < end_time - fade_time {
		texture.draw(ctx, position)
	} else if accumulator < end_time {
		texture.draw(ctx, position.color(Color::rgba(1.0, 1.0, 1.0, (end_time - accumulator) / fade_time)));
	}
}

pub fn fade_in(ctx: &mut Context, texture: &Texture, x: f32, y: f32, accumulator: f32, fade_time: f32) {
	let position = position(x, y);
	texture.draw(ctx, if accumulator < fade_time {
		position.color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time))
	} else {
		position
	});
}