use worldcli::worldlib::events::Sender;

use super::{
    copyright::CopyrightLoadingScene, gamefreak::GamefreakLoadingScene,
    pokemon::PokemonLoadingScene, LoadingScene, LoadingScenes, LoadingState,
};

use crate::{
    engine::{
        controls::{pressed, Control},
        error::ImageError,
        log::info,
        Context, EngineContext,
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

    pub fn start(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        self.current = LoadingScenes::Copyright;
        self.get_mut().start(ctx, eng);
    }

    pub fn update(&mut self, ctx: &mut Context, eng: &mut EngineContext, delta: f32) {
        if pressed(ctx, eng, Control::A) {
            self.sender.send(StateMessage::Goto(MainStates::Menu));
        }

        match self.get().state() {
            LoadingState::Continue => {
                self.get_mut().update(ctx, eng, delta);
            }
            LoadingState::Next(scene) => {
                self.current = scene;
                self.get_mut().start(ctx, eng);
            }
            LoadingState::End => {
                self.finish();
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &mut EngineContext) {
        self.get().draw(ctx, eng);
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
