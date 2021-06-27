use pokedex::pokemon::Health;
use crate::tetra::{
	Context,
	math::Vec2,
	graphics::{
		Color,
		Texture,
		DrawParams,		
	},
};

use crate::graphics::draw_rectangle;

use super::ProgressBar;

static mut TEXTURE: Option<Texture> = None;

pub struct HealthBar {
	background: Texture,
	bar: ProgressBar,
}

pub struct HealthBarColor {
	upper: Color,
	lower: Color,
}

impl HealthBar {

	pub const WIDTH: f32 = 48.0;

	pub const GREEN: &'static HealthBarColor = &HealthBarColor { upper: Color::rgb(88.0 / 255.0, 208.0 / 255.0, 128.0 / 255.0), lower: Color::rgb(112.0 / 255.0, 248.0 / 255.0, 168.0 / 255.0) };

	pub const YELLOW: &'static HealthBarColor = &HealthBarColor { upper: Color::rgb(200.0 / 255.0, 168.0 / 255.0, 8.0 / 255.0), lower: Color::rgb(248.0 / 255.0, 224.0 / 255.0, 56.0 / 255.0) };

	pub const RED: &'static HealthBarColor = &HealthBarColor { upper: Color::rgb(168.0 / 255.0, 64.0 / 255.0, 72.0 / 255.0), lower: Color::rgb(248.0 / 255.0, 88.0 / 255.0, 56.0 / 255.0) };
	
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			background: Self::texture(ctx).clone(),
			bar: ProgressBar::new(Self::WIDTH),
		}
	}

	pub fn with_size(ctx: &mut Context, width: f32) -> Self {
		Self {
			background: Self::texture(ctx).clone(),
			bar: ProgressBar::new(width),
		}
	}

	pub fn texture(ctx: &mut Context) -> &Texture {
		unsafe { TEXTURE.get_or_insert(crate::graphics::byte_texture(ctx, include_bytes!("../../../assets/gui/health.png"))) }
	}

	pub fn width(current: Health, max: Health) -> f32 {
		(current as f32 * Self::WIDTH / max as f32).clamp(0.0, Self::WIDTH)
	}
	
	pub fn resize_hp(&mut self, current: Health, max: Health, reset: bool) {
		self.resize(current as f32 / max as f32, reset)
	}

	pub fn resize(&mut self, percent: f32, reset: bool) {
		self.bar.resize(Self::WIDTH * percent, reset);
	}

	pub fn is_moving(&self) -> bool {
		self.bar.moving()
	}
	
	pub fn update(&mut self, delta: f32) {
		self.bar.update(delta)
	}

	pub fn draw(&self, ctx: &mut Context, origin: Vec2<f32>) {
		self.background.draw(ctx, DrawParams::position(DrawParams::default(), origin));
		let x = origin.x + 15.0;
		let w = self.bar.width().ceil();
		let color = if w < Self::WIDTH / 8.0 {
			Self::RED
		} else if w < Self::WIDTH / 2.0 {
			Self::YELLOW
		} else {
			Self::GREEN
		};
		draw_rectangle(ctx, x, origin.y + 2.0, w, 1.0, color.upper);
		draw_rectangle(ctx, x, origin.y + 3.0, w, 2.0, color.lower);
	}

}