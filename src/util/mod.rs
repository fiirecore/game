use opengl_graphics::GlGraphics;
use piston_window::Context;

use self::context::GameContext;
use self::text_renderer::TextRenderer;

pub mod image_util;
pub mod render_util;
pub mod texture_util;
pub mod traits;
pub mod timer;
pub mod input;
pub mod text_renderer;
pub mod random;
pub mod file;
pub mod context;

pub static TILE_SIZE: u8 = 16;

pub trait Update {

	fn update(&mut self, context: &mut GameContext); 

}

pub trait Render {

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer);

}