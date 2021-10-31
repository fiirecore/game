use engine::tetra::{
	State, Context, Result,
	math::Vec2,
};

use crate::{GameContext, state::{Action, MainState, MainStates}};

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
	action: Option<Action>,

	title: TitleState,
	main_menu: MainMenuState,
	character: CharacterCreationState,

}

impl MenuStateManager {

	pub fn new(ctx: &mut Context, scaler: Vec2<f32>) -> Self {
		Self {
			current: MenuStates::default(),
			action: Default::default(),
			title: TitleState::new(ctx),
			main_menu: MainMenuState::new(scaler),
			character: CharacterCreationState::new(),
		}
	}

	// fn get(&self) -> &dyn MenuState {
	// 	match self.current {
	// 	    MenuStates::Title => &self.title,
	// 	    MenuStates::MainMenu => &self.main_menu,
	// 		MenuStates::CharacterCreation => &self.character,		    
	// 	}
	// }

	fn get_mut<'d>(&mut self) -> &mut dyn MenuState<'d> {
		match self.current {
		    MenuStates::Title => &mut self.title,
		    MenuStates::MainMenu => &mut self.main_menu,
			MenuStates::CharacterCreation => &mut self.character,
		}
	}
	
}

impl<'d> State<GameContext<'d>> for MenuStateManager {

    fn begin(&mut self, ctx: &mut GameContext<'d>) -> Result {
		self.get_mut().begin(ctx)
    }

    fn end(&mut self, ctx: &mut GameContext<'d>) -> Result {
        self.get_mut().end(ctx)
    }

    fn update(&mut self, ctx: &mut GameContext<'d>) -> Result {
		self.get_mut().update(ctx)?;
		if let Some(action) = self.get_mut().next().take() {
			match action {
				MenuStateAction::Goto(state) => {
					self.state(ctx, state)?;
				}
				MenuStateAction::StartGame => {
					self.action = Some(Action::Goto(MainStates::Game));
				}
				MenuStateAction::SeedAndGoto(seed, state) => {
					self.state(ctx, state)?;
					self.action = Some(Action::Seed(seed));
				}
			}
		}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut GameContext<'d>) -> Result {
        self.get_mut().draw(ctx)
    }

}

impl MenuStateManager {
	fn state<'d>(&mut self, ctx: &mut GameContext<'d>, state: MenuStates) -> Result {
		self.get_mut().end(ctx)?;
		self.current = state;
		self.get_mut().begin(ctx)
	}
}

impl<'d> MainState<'d> for MenuStateManager {
    fn action(&mut self) -> Option<Action> {
        self.action.take()
    }
}