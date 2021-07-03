use game::{
	storage::saves,
	tetra::{State, Context, Result},
};

use crate::state::menu::{MenuState, MenuStateAction, MenuStates};

pub struct CharacterCreationState {
	action: Option<MenuStateAction>,
}

impl CharacterCreationState {
	pub fn new(_ctx: &mut Context) -> Self {
		Self {
			action: None,
		}
	}
}

impl State for CharacterCreationState {
	fn begin(&mut self, _ctx: &mut Context) -> Result {
		saves().select_new(&format!("Player{}", game::util::date()));
		self.action = Some(MenuStateAction::Goto(MenuStates::MainMenu));
		Ok(())
	}
}

impl MenuState for CharacterCreationState {
    fn next(&mut self) -> &mut Option<MenuStateAction> {
        &mut self.action
    }
}