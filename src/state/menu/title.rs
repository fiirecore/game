

use crate::state::menu::{MenuState, MenuStateAction, MenuStates};

use game::{
	input::{pressed, Control},
	tetra::{
		Context,
		State,
		Result,
		graphics::{
			Texture
		},
	}
};

use game::graphics::{byte_texture, position};

pub struct TitleState {	
	
	action: Option<MenuStateAction>,
	
	accumulator: f32,

	background: Texture,
	title: Texture,
	trademark: Texture,
	subtitle: Texture,
	charizard: Texture,
	start: Texture,
	
}

impl TitleState {
	pub fn new(ctx: &mut Context) -> Self {
		Self {
		    action: None,
			background: byte_texture(ctx, include_bytes!("../../../build/assets/scenes/title/background.png")),		
			title: byte_texture(ctx, include_bytes!("../../../build/assets/scenes/title/title.png")),
			trademark: byte_texture(ctx, include_bytes!("../../../build/assets/scenes/title/trademark.png")),
			subtitle: byte_texture(ctx, include_bytes!("../../../build/assets/scenes/title/subtitle.png")),
			charizard: byte_texture(ctx, include_bytes!("../../../build/assets/scenes/title/charizard.png")),
			start: byte_texture(ctx, include_bytes!("../../../build/assets/scenes/title/start.png")),
		    accumulator: 0.0,
		}		
	}
}

impl State for TitleState {

	fn begin(&mut self, ctx: &mut Context) -> Result {
		game::play_music_named(ctx, "Title");
		self.accumulator = 0.0;
		Ok(())
	}
    
	fn update(&mut self, ctx: &mut Context) -> Result {	
		if pressed(ctx, Control::A) {
			let seed = self.accumulator as u64 % 256;
			game::init::seed_randoms(seed);
			self.action = Some(MenuStateAction::Goto(MenuStates::MainMenu));
		}
		self.accumulator += game::tetra::time::get_delta_time(ctx).as_secs_f32();
		Ok(())
	}
	
	fn draw(&mut self, ctx: &mut Context) -> Result {
		self.background.draw(ctx, position(0.0, 0.0));
		self.title.draw(ctx, position(3.0, 3.0));
		self.trademark.draw(ctx, position(158.0, 53.0));
		self.subtitle.draw(ctx, position(52.0, 57.0));
		if self.accumulator as u8 % 2 == 1 {
			self.start.draw(ctx,  position(44.0, 130.0));
		}
		self.charizard.draw(ctx,  position(129.0, 49.0));
		Ok(())
	}

}

impl MenuState for TitleState {
	fn next(&mut self) -> &mut Option<MenuStateAction> {
		&mut self.action
	}
}