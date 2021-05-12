use pokedex::pokemon::Health;
use macroquad::prelude::{Color, color_u8, Vec2, Texture2D, draw_rectangle};
use crate::graphics::{byte_texture, draw};

use super::bar::ProgressBar;

static mut TEXTURE: Option<Texture2D> = None;

pub struct HealthBar {
	background: Texture2D,
	bar: ProgressBar,
}

impl HealthBar {

	pub const WIDTH: f32 = 48.0;

	pub const UPPER: Color = color_u8!(88, 208, 128, 255);
	pub const LOWER: Color = color_u8!(112, 248, 168, 255);
	
	pub fn new() -> Self {
		Self {
			background: Self::texture(),
			bar: ProgressBar::new(Self::WIDTH),
		}
	}

	pub fn with_size(width: f32) -> Self {
		Self {
			background: Self::texture(),
			bar: ProgressBar::new(width),
		}
	}

	pub fn texture() -> Texture2D {
		unsafe { *TEXTURE.get_or_insert(byte_texture(include_bytes!("../../assets/gui/health.png"))) }
	}

	pub fn width(current: Health, max: Health) -> f32 {
		current as f32 * Self::WIDTH / max as f32
	}
	
	pub fn resize(&mut self, current: Health, max: Health, reset: bool) {
		self.bar.resize(Self::width(current, max), reset);
	}

	pub fn is_moving(&self) -> bool {
		self.bar.moving()
	}
	
	pub fn update(&mut self, delta: f32) {
		self.bar.update(delta)
	}

	pub fn render(&self, origin: Vec2) {
		draw(self.background, origin.x, origin.y);
		let x = origin.x + 15.0;
		let w = self.bar.width();
		draw_rectangle(x, origin.y + 2.0, w, 1.0, Self::UPPER);
		draw_rectangle(x, origin.y + 3.0, w, 2.0, Self::LOWER);
	}

}