use engine::tetra::{Result, State};
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

impl<'d> State<GameContext<'d>> for CharacterCreationState {
    fn begin(&mut self, ctx: &mut GameContext<'d>) -> Result {
        ctx.saves.select_new(default_name_str(), &mut rand::thread_rng(), ctx.dex.pokedex, ctx.dex.movedex, ctx.dex.itemdex);
        self.action = Some(MenuStateAction::Goto(MenuStates::MainMenu));
        Ok(())
    }
}

impl<'d> MenuState<'d> for CharacterCreationState {
    fn next(&mut self) -> &mut Option<MenuStateAction> {
        &mut self.action
    }
}
