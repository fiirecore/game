use crate::engine::State;
use saves::default_name_str;

use crate::{
    state::menu::{MenuState, MenuStateAction, MenuStates},
    GameContext,
};

pub struct CharacterCreationState {
    action: Option<MenuStateAction>,
}

impl CharacterCreationState {
    pub fn new() -> Self {
        Self { action: None }
    }
}

impl<'d> State<GameContext> for CharacterCreationState {
    fn start(&mut self, ctx: &mut GameContext) {
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

impl<'d> MenuState<'d> for CharacterCreationState {
    fn next(&mut self) -> &mut Option<MenuStateAction> {
        &mut self.action
    }
}
