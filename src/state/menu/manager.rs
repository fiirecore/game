use crate::{
    saves::Player,
    state::{MainStates, StateMessage},
};

use worldcli::worldlib::events::{split, Receiver, Sender};

use crate::engine::{Context, EngineContext, EngineError};

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
    pub(crate) fn new(
        ctx: &mut Context,
        sender: Sender<StateMessage>,
    ) -> Result<Self, EngineError> {
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
    pub fn start(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        match self.current {
            MenuStates::Title => self.title.start(ctx, eng),
            MenuStates::MainMenu => self.main_menu.start(),
        }
    }

    pub fn end(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        // self.get_mut().end(ctx)
        match self.current {
            MenuStates::Title => self.title.end(ctx, eng),
            MenuStates::MainMenu => (),
        }
    }

    pub fn update(&mut self, ctx: &mut Context, eng: &mut EngineContext, delta: f32, save: &mut Option<Player>) {
        match self.current {
            MenuStates::Title => self.title.update(ctx, eng, delta),
            MenuStates::MainMenu => self.main_menu.update(ctx, eng, save),
        }
        for action in self.receiver.try_iter() {
            match action {
                MenuActions::Seed(seed) => self.sender.send(StateMessage::Seed(seed)),
                MenuActions::Goto(state) => {
                    match self.current {
                        MenuStates::Title => self.title.end(ctx, eng),
                        MenuStates::MainMenu => (),
                    }
                    self.current = state;
                    match self.current {
                        MenuStates::Title => self.title.start(ctx, eng),
                        MenuStates::MainMenu => self.main_menu.start(),
                    }
                }
                MenuActions::StartGame => {
                    if save.is_some() {
                        self.sender.send(StateMessage::LoadSave);
                        self.sender.send(StateMessage::Goto(MainStates::Game));
                    }
                }
                MenuActions::ExitGame => self.sender.send(StateMessage::Exit),
            }
        }
        // Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, eng: &EngineContext) {
        match self.current {
            MenuStates::Title => self.title.draw(ctx),
            MenuStates::MainMenu => self.main_menu.draw(ctx, eng),
        }
    }
}
