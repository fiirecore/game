use super::LoadingState;

use crate::engine::{Context, error::ImageError};

pub struct PokemonLoadingScene {
	state: LoadingState,
}

impl super::LoadingScene for PokemonLoadingScene {

	fn new(ctx: &mut Context) -> Result<Self, ImageError> {
		Ok(Self {
            state: LoadingState::Continue,
        })
	}

	fn start(&mut self, ctx: &mut Context) {
		todo!()
	}
	
	fn update(&mut self, ctx: &mut Context, _delta: f32) {
        todo!()
    }
	   
	fn draw(&self, ctx: &mut Context) {
		todo!()
	}

    fn state(&self) -> LoadingState {
		self.state
	}
	
}