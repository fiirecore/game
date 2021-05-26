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

use super::bar::ProgressBar;

static mut TEXTURE: Option<Texture> = None;

pub struct HealthBar {
	background: Texture,
	bar: ProgressBar,
}

impl HealthBar {

	pub const WIDTH: f32 = 48.0;

	pub const UPPER: Color = Color::rgb(88.0 / 255.0, 208.0 / 255.0, 128.0 / 255.0);
	pub const LOWER: Color = Color::rgb(112.0 / 255.0, 248.0 / 255.0, 168.0 / 255.0);
	
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
		unsafe { TEXTURE.get_or_insert(crate::graphics::byte_texture(ctx, include_bytes!("../../assets/gui/health.png"))) }
	}

	pub fn width(current: Health, max: Health) -> f32 {
		(current as f32 * Self::WIDTH / max as f32).clamp(0.0, Self::WIDTH)
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

	pub fn draw(&self, ctx: &mut Context, origin: Vec2<f32>) {
		self.background.draw(ctx, DrawParams::position(DrawParams::default(), origin));
		let x = origin.x + 15.0;
		let w = self.bar.width();
		draw_rectangle(ctx, x, origin.y + 2.0, w, 1.0, Self::UPPER);
		draw_rectangle(ctx, x, origin.y + 3.0, w, 2.0, Self::LOWER);
	}

}