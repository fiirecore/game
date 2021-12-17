use firecore_battle_gui::pokedex::engine::EngineError;

use crate::{
    saves::SavedPlayer,
    state::{MainStates, StateMessage},
};
use crate::{split, Receiver, Sender};

use crate::engine::Context;

use super::{main::MainMenuState, title::TitleState, MenuActions, MenuStates};

pub struct MenuStateManager {
    current: MenuStates,

    title: TitleState,
    main_menu: MainMenuState,
    // character: CharacterCreationState,
    sender: Sender<StateMessage>,
    receiver: Receiver<MenuActions>,
}

impl MenuStateManager {
    pub(crate) fn new(ctx: &mut Context, sender: Sender<StateMessage>) -> Result<Self, EngineError> {
        let (actions, receiver) = split();

        Ok(Self {
            current: MenuStates::default(),

            title: TitleState::new(ctx, actions.clone())?,
            main_menu: MainMenuState::new(actions),
            // character: CharacterCreationState::new(),
            sender,
            receiver,
        })
    }
}

impl MenuStateManager {
    pub fn start(&mut self, ctx: &mut Context, saves: &[SavedPlayer]) {
        match self.current {
            MenuStates::Title => self.title.start(ctx),
            MenuStates::MainMenu => self.main_menu.start(saves),
        }
    }

    pub fn end(&mut self, ctx: &mut Context) {
        // self.get_mut().end(ctx)
        match self.current {
            MenuStates::Title => self.title.end(ctx),
            MenuStates::MainMenu => (),
        }
    }

    pub fn update(&mut self, ctx: &mut Context, delta: f32, saves: &mut Vec<SavedPlayer>) {
        match self.current {
            MenuStates::Title => self.title.update(ctx, delta),
            MenuStates::MainMenu => self.main_menu.update(ctx, saves),
        }
        for action in self.receiver.try_iter() {
            match action {
                MenuActions::Seed(seed) => self.sender.send(StateMessage::Seed(seed)),
                MenuActions::Goto(state) => {
                    match self.current {
                        MenuStates::Title => self.title.end(ctx),
                        MenuStates::MainMenu => (),
                    }
                    self.current = state;
                    match self.current {
                        MenuStates::Title => self.title.start(ctx),
                        MenuStates::MainMenu => self.main_menu.start(saves),
                    }
                }
                MenuActions::StartGame(index) => {
                    if let Some(save) = saves.get(index) {
                        self.sender.send(StateMessage::UseSave(save.clone()));
                        self.sender.send(StateMessage::Goto(MainStates::Game));
                    }
                }
                MenuActions::ExitGame => self.sender.send(StateMessage::Exit),
            }
        }
        // Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, saves: &[SavedPlayer]) {
        match self.current {
            MenuStates::Title => self.title.draw(ctx),
            MenuStates::MainMenu => self.main_menu.draw(ctx, saves),
        }
    }
}
