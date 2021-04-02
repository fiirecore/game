use super::LoadingState;

pub struct PokemonLoadingScene {
	state: LoadingState,
}

impl super::LoadingScene for PokemonLoadingScene {

	fn new() -> Self {
		Self {
            state: LoadingState::Continue,
        }
	}

	fn on_start(&mut self) {
		todo!()
	}
	
	fn update(&mut self, _delta: f32) {
        todo!()
    }
	   
	fn render(&self) {
		todo!()
	}

    fn state(&self) -> LoadingState {
		self.state
	}
	
}