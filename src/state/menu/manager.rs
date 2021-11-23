use crate::engine::{Context, State};

use crate::{
    state::{Action, MainState, MainStates},
    GameContext,
};

use super::{
    character::CharacterCreationState, main_menu::MainMenuState, title::TitleState, MenuState,
    MenuStateAction, MenuStates,
};

pub struct MenuStateManager {
    current: MenuStates,
    action: Option<Action>,

    title: TitleState,
    main_menu: MainMenuState,
    character: CharacterCreationState,
}

impl MenuStateManager {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            current: MenuStates::default(),
            action: Default::default(),
            title: TitleState::new(ctx),
            main_menu: MainMenuState::new(),
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

impl<'d> State<GameContext> for MenuStateManager {
    fn start(&mut self, ctx: &mut GameContext) {
        self.get_mut().start(ctx)
    }

    fn end(&mut self, ctx: &mut GameContext) {
        self.get_mut().end(ctx)
    }

    fn update(&mut self, ctx: &mut GameContext, delta: f32) {
        self.get_mut().update(ctx, delta);
        if let Some(action) = self.get_mut().next().take() {
            match action {
                MenuStateAction::Goto(state) => {
                    self.state(ctx, state);
                }
                MenuStateAction::StartGame => {
                    self.action = Some(Action::Goto(MainStates::Game));
                }
                MenuStateAction::SeedAndGoto(seed, state) => {
                    self.state(ctx, state);
                    self.action = Some(Action::Seed(seed));
                }
            }
        }
        // Ok(())
    }

    fn draw(&mut self, ctx: &mut GameContext) {
        self.get_mut().draw(ctx)
    }
}

impl MenuStateManager {
    fn state<'d>(&mut self, ctx: &mut GameContext, state: MenuStates) {
        self.get_mut().end(ctx);
        self.current = state;
        self.get_mut().start(ctx)
    }
}

impl<'d> MainState for MenuStateManager {
    fn action(&mut self) -> Option<Action> {
        self.action.take()
    }
}
