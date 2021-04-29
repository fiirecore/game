use game::storage::{get_mut, player::PlayerSaves};

use crate::state::State;
use crate::state::States;

use super::{
	MenuState,
    MenuStateAction,
    MenuStates,
	title::TitleState,
	main_menu::MainMenuState,
	character::CharacterCreationState,
};

pub struct MenuStateManager {

	current: MenuStates,

	title: TitleState,
	main_menu: MainMenuState,
	character: CharacterCreationState,

}

impl MenuStateManager {

	fn get(&self) -> &dyn MenuState {
		match self.current {
		    MenuStates::Title => &self.title,
		    MenuStates::MainMenu => &self.main_menu,
			MenuStates::CharacterCreation => &self.character,		    
		}
	}

	fn get_mut(&mut self) -> &mut dyn MenuState {
		match self.current {
		    MenuStates::Title => &mut self.title,
		    MenuStates::MainMenu => &mut self.main_menu,
			MenuStates::CharacterCreation => &mut self.character,
		}
	}
	
}

impl State for MenuStateManager {

	fn new() -> Self {
		Self {
			current: MenuStates::default(),
			title: TitleState::new(),
			main_menu: MainMenuState::new(),
			character: CharacterCreationState::new(),
		}
	}

    fn on_start(&mut self) {
		#[cfg(debug_assertions)] {
			let mut saves = get_mut::<PlayerSaves>().expect("Could not get player saves");
			if saves.saves.is_empty() {
				self.current = MenuStates::Title;
			} else {
				saves.select(0);
			}			
		}
		self.get_mut().on_start();
    }

    fn input(&mut self, delta: f32) {
        self.get_mut().input(delta);
    }

    fn update(&mut self, delta: f32) -> Option<States> {
		match self.get_mut().action().take() {
			Some(action) => {
				match action {
				    MenuStateAction::Goto(state) => {
						self.get_mut().quit();
						self.current = state;
						self.get_mut().on_start();
					}
				    MenuStateAction::StartGame => {
						return Some(States::Game);
					}
				}
			},
			None => {
				self.get_mut().update(delta);
			}
		}
		None
	}

    fn render(&self) {
        self.get().render();
    }

    fn quit(&mut self) {
        self.get_mut().quit();
    }
	
}
