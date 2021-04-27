use game::data::{get_mut, player::PlayerSaves};

use super::{
	State,
	States,
	game::GameState,
	menu::manager::MenuStateManager,
};

pub struct StateManager {

	current: States,

	menu: MenuStateManager,
	game: GameState,

}

impl StateManager {

	pub fn new() -> Self {
		Self {
			current: States::default(),
			menu: MenuStateManager::new(),
			game: GameState::new(),
		}
	}

	pub async fn load(&mut self) {
		self.game.load().await;
	}

    pub fn on_start(&mut self) {
		#[cfg(debug_assertions)] {
			let mut saves = get_mut::<PlayerSaves>().expect("Could not get player saves");
			if saves.saves.is_empty() {
				self.current = States::Menu;
			} else {
				saves.select(0);
			}			
		}
		self.get_mut().on_start();
    }

    pub fn input(&mut self, delta: f32) {
        self.get_mut().input(delta);
    }

    pub fn update(&mut self, delta: f32) {
		if let Some(state) = self.get_mut().update(delta) {
			self.get_mut().quit();
			self.current = state;
			self.get_mut().on_start();
		}
	}

    pub fn render(&self) {
        self.get().render();
    }

    pub fn quit(&mut self) {
        self.get_mut().quit();
    }

	

	fn get(&self) -> &dyn State {
		match self.current {
		    States::Menu => &self.menu,
		    States::Game => &self.game,
		}
	}

	fn get_mut(&mut self) -> &mut dyn State {
		match self.current {
		    States::Menu => &mut self.menu,
		    States::Game => &mut self.game,
		}
	}
	
}
