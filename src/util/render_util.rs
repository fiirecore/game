use std::time::SystemTime;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::{Context, DrawState, Graphics, Image, Rectangle, Transformed, rectangle::rectangle_by_corners, types::Color, ImageSize};

use crate::app::{WIDTH, HEIGHT};

pub static VIEW_WIDTH: usize = WIDTH;
pub static VIEW_HEIGHT: usize = HEIGHT;
pub static TEXTURE_SIZE: usize = 16;
//pub static TEXTURE_SIZE_ROOT: usize = 4;
	
//fn get_coords(x: isize, y: isize) -> graphics::DrawParam {
	//return graphics::DrawParam::new().dest(Point2::new(x as f32 + crate::game::WIDTH as f32 / 2.0, y as f32 + crate::game::HEIGHT as f32 / 2.0));
//}

pub fn draw(ctx: &mut Context, g: &mut GlGraphics, texture: &Texture, x: isize, y: isize) {
	Image::new_color([1f32, 1f32, 1f32, 1f32]).draw(
		texture,
		&DrawState::default(),
		ctx.trans(x as f64, y as f64).transform,
		g
	);	
}

pub fn draw_o(ctx: &mut Context, g: &mut GlGraphics, texture: &Option<Texture>, x: isize, y: isize) {
	if let Some(texture) = texture {
		draw(ctx, g, texture, x, y);
	}
}

pub fn draw_flip(ctx: &mut Context, g: &mut GlGraphics, tex: &Texture, x: isize, y: isize, flip: bool) {

	let con;
	
	if flip {
		con = ctx.trans((x + tex.get_width() as isize) as f64, y as f64).flip_h();
	} else {
		con = ctx.trans(x as f64, y as f64);
	}
	
	Image::new_color([1f32, 1f32, 1f32, 1f32]).draw(
		tex,
		&DrawState::default(),
		con.transform,
		g
	);
	
}

pub fn draw_rect(ctx: &mut Context, g: &mut GlGraphics, color: Color, x: isize, y: isize, width: usize, height: usize) {
	//let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect::new_i32(x as i32, y as i32, width as i32, height as i32), color).expect("Could not create rectangle");
	//graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
	let dims = rectangle_by_corners(0.0, 0.0, width as f64, height as f64);
	Rectangle::new(color).draw(dims, &DrawState::default(), ctx.trans(x as f64, y as f64).transform, g);
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
pub fn fade_out_o(c: &mut Context, g: &mut GlGraphics, tex: &Option<Texture>, x: u32, y: u32, start_time: SystemTime, offset: u64, duration: u32, red: u8, green: u8, blue: u8) {
	
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

#[allow(clippy::too_many_arguments)]
pub fn fade_in_out_o(c: &mut Context, g: &mut GlGraphics, tex: &Option<Texture>, x: u32, y: u32, start_time: SystemTime, out_time: u64, duration: u32, red: u8, green: u8, blue: u8) {
	
	// prob has bad performance
	
	// let (r, g, b) = (red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0);
	
	g.clear_color([red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0, 1f32]);
	
	let fade: f32;
	if start_time.elapsed().unwrap().as_millis() < duration as u128 {
		fade = start_time.elapsed().unwrap().as_millis() as f32 / duration as f32;
	} else if start_time.elapsed().unwrap().as_millis() > out_time as u128 {
		fade = 1f32 - (start_time.elapsed().unwrap().as_millis() as f32 - out_time as f32) / duration as f32;
	} else {
		fade = 1f32;
	}
	
	Image::new_color([fade, fade, fade, 1f32]).draw(
		tex.iter().next().unwrap(),
		&DrawState::default(),
		c.trans(x as f64, y as f64).transform,
		g
	);
	
}

