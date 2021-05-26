use game::tetra::{State, Context, Result};

use crate::state::{MainState, MainStates};

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
	next: Option<MainStates>,

	title: TitleState,
	main_menu: MainMenuState,
	character: CharacterCreationState,

}

impl MenuStateManager {

	pub fn new(ctx: &mut Context) -> Self {
		Self {
			current: MenuStates::default(),
			next: None,
			title: TitleState::new(ctx),
			main_menu: MainMenuState::new(ctx),
			character: CharacterCreationState::new(ctx),
		}
	}

	// fn get(&self) -> &dyn MenuState {
	// 	match self.current {
	// 	    MenuStates::Title => &self.title,
	// 	    MenuStates::MainMenu => &self.main_menu,
	// 		MenuStates::CharacterCreation => &self.character,		    
	// 	}
	// }

	fn get_mut(&mut self) -> &mut dyn MenuState {
		match self.current {
		    MenuStates::Title => &mut self.title,
		    MenuStates::MainMenu => &mut self.main_menu,
			MenuStates::CharacterCreation => &mut self.character,
		}
	}
	
}

impl State for MenuStateManager {

    fn begin(&mut self, ctx: &mut Context) -> Result {
		self.get_mut().begin(ctx)
    }

    fn end(&mut self, ctx: &mut Context) -> Result {
        self.get_mut().end(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> Result {
		self.get_mut().update(ctx)?;
		if let Some(action) = self.get_mut().next().take() {
			match action {
				MenuStateAction::Goto(state) => {
					self.get_mut().end(ctx)?;
					self.current = state;
					self.get_mut().begin(ctx)?;
				}
				MenuStateAction::StartGame => {
					self.next = Some(MainStates::Game);
				}
			}
		}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
        self.get_mut().draw(ctx)
    }

}

impl MainState for MenuStateManager {
    fn next(&mut self) -> &mut Option<MainStates> {
        &mut self.next
    }
}