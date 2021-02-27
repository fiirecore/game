pub struct PokemonLoadingScene {
	finished: bool,
}

impl PokemonLoadingScene {
	pub fn new() -> Self {
		Self {
            finished: false,
        }
	}
}

impl super::LoadingScene for PokemonLoadingScene {

	fn on_start(&mut self) {
		self.finished = false;
	}
	
	fn update(&mut self, _delta: f32) {
        self.finished = true;
    }
	   
	fn render(&self) {}

    fn finished(&self) -> bool {
        self.finished
    }
	
}