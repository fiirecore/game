use firecore_world::events::Sender;

use super::{
    copyright::CopyrightLoadingScene, gamefreak::GamefreakLoadingScene,
    pokemon::PokemonLoadingScene, LoadingScene, LoadingScenes, LoadingState,
};

use crate::{
    engine::{
        error::ImageError,
        input::controls::{pressed, Control},
        log::info,
        Context,
    },
    state::{MainStates, StateMessage},
};

pub struct LoadingStateManager {
    current: LoadingScenes,

    copyright: CopyrightLoadingScene,
    gamefreak: GamefreakLoadingScene,
    pokemon: PokemonLoadingScene,

    sender: Sender<StateMessage>,
}

impl LoadingStateManager {
    pub(crate) fn new(ctx: &mut Context, sender: Sender<StateMessage>) -> Result<Self, ImageError> {
        Ok(Self {
            current: LoadingScenes::default(),
            copyright: CopyrightLoadingScene::new(ctx)?,
            gamefreak: GamefreakLoadingScene::new(ctx)?,
            pokemon: PokemonLoadingScene::new(ctx)?,
            sender,
        })
    }

    pub fn start(&mut self, ctx: &mut Context) {
        self.current = LoadingScenes::Copyright;
        self.get_mut().start(ctx);
    }

    pub fn update(&mut self, ctx: &mut Context, delta: f32) {
        if pressed(ctx, Control::A) {
            self.sender.send(StateMessage::Goto(MainStates::Menu));
        }

        match self.get().state() {
            LoadingState::Continue => {
                self.get_mut().update(ctx, delta);
            }
            LoadingState::Next(scene) => {
                self.current = scene;
                self.get_mut().start(ctx);
            }
            LoadingState::End => {
                self.finish();
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.get().draw(ctx);
    }

    fn finish(&mut self) {
        self.sender.send(StateMessage::Goto(MainStates::Menu));
        info!("Finished loading scene sequence.");
    }

    fn get(&self) -> &dyn LoadingScene {
        match self.current {
            LoadingScenes::Copyright => &self.copyright,
            LoadingScenes::Gamefreak => &self.gamefreak,
            LoadingScenes::Pokemon => &self.pokemon,
        }
    }

    fn get_mut(&mut self) -> &mut dyn LoadingScene {
        match self.current {
            LoadingScenes::Copyright => &mut self.copyright,
            LoadingScenes::Gamefreak => &mut self.gamefreak,
            LoadingScenes::Pokemon => &mut self.pokemon,
        }
    }
}
