use super::LoadingState;

use crate::engine::{error::ImageError, Context, EngineContext};

pub struct PokemonLoadingScene {
    state: LoadingState,
}

impl super::LoadingScene for PokemonLoadingScene {
    fn new(ctx: &mut Context) -> Result<Self, ImageError> {
        Ok(Self {
            state: LoadingState::Continue,
        })
    }

    fn start(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        todo!()
    }

    fn update(&mut self, ctx: &mut Context, eng: &mut EngineContext, _delta: f32) {
        todo!()
    }

    fn draw(&self, ctx: &mut Context, eng: &mut EngineContext) {
        todo!()
    }

    fn state(&self) -> LoadingState {
        self.state
    }
}
