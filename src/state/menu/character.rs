use game::{
	storage::{get_mut, player::PlayerSaves},
	macroquad::miniquad::date,
};

use super::{MenuState, MenuStateAction, MenuStates};

pub struct CharacterCreationState {
	action: Option<MenuStateAction>,
}

impl MenuState for CharacterCreationState {

	fn new() -> Self {
		Self {
			action: None,
		}
	}
	
	fn on_start(&mut self) {
		if let Some(mut saves) = get_mut::<PlayerSaves>() {
			saves.select_new(&format!("Player{}", date::now() as u64 % 1000000));
		} else {
			panic!("Could not get player data!");
		}
		self.action = Some(MenuStateAction::Goto(MenuStates::MainMenu));
	}
	
	fn input(&mut self, _delta: f32) {
		
	}
	
	fn update(&mut self, _delta: f32) {
		
	}

	fn render(&self) {}

	fn quit(&mut self) {
	}

	fn action(&mut self) -> &mut Option<MenuStateAction> {
		&mut self.action
	}
}