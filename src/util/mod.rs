use self::text_renderer::TextRenderer;

pub mod image;
pub mod render;
pub mod texture;
pub mod timer;
pub mod input;
pub mod text_renderer;
pub mod file;
// pub mod context;
pub mod battle_data;

pub static TILE_SIZE: u8 = 16;

pub trait Load {

	fn load(&mut self);
	
	fn on_start(&mut self) {

	}

}

pub trait Quit {

	fn quit(&mut self);

}

pub trait Update {

	fn update(&mut self, delta: f32); 

}

pub trait Render {

	fn render(&self, tr: &TextRenderer);

}

pub trait Reset {

	fn reset(&mut self);

}
pub trait Completable: Reset {

    fn is_finished(&self) -> bool;

}