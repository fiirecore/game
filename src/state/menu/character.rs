use crate::engine::{State, Context};
use crate::saves::default_name_str;

use crate::{
    state::menu::{MenuActions, MenuStates},
};

pub struct CharacterCreationState {
    action: Option<MenuStateAction>,
}

impl CharacterCreationState {
    pub fn new() -> Self {
        Self { action: None }
    }
}

impl State for CharacterCreationState {
    fn start(&mut self, ctx: &mut Context) {
        ctx.saves.select_new(
            default_name_str(),
            &mut ctx.random,
            crate::pokedex(),
            crate::movedex(),
            crate::itemdex(),
        );
        self.action = Some(MenuStateAction::Goto(MenuStates::MainMenu));
    }
}

impl MenuState for CharacterCreationState {
    fn next(&mut self) -> &mut Option<MenuStateAction> {
        &mut self.action
    }
}
