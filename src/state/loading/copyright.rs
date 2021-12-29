use crate::engine::{
    error::ImageError,
    graphics::{draw_text_left, DrawParams, Texture},
    text::MessagePage,
    Context,
};

use super::{fade_in_out, LoadingScenes, LoadingState};

pub struct CopyrightLoadingScene {
    state: LoadingState,
    accumulator: f32,
    scene_texture: Texture,
    version: String,
}

impl super::LoadingScene for CopyrightLoadingScene {
    fn new(ctx: &mut Context) -> Result<Self, ImageError> {
        Ok(Self {
            state: LoadingState::Continue,
            scene_texture: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/loading/copyright.png"),
            )?,
            accumulator: 0.0,
            version: format!("v{}", crate::VERSION),
        })
    }

    fn start(&mut self, ctx: &mut Context) {
        self.state = LoadingState::Continue;
        self.accumulator = 0.0;
    }

    fn update(&mut self, ctx: &mut Context, delta: f32) {
        self.accumulator += delta;
        if self.accumulator > 4.0 {
            self.state = LoadingState::Next(LoadingScenes::Gamefreak);
        }
    }

    fn draw(&self, ctx: &mut Context) {
        fade_in_out(
            ctx,
            &self.scene_texture,
            0.0,
            0.0,
            self.accumulator,
            3.0,
            0.5,
        );
        draw_text_left(
            ctx,
            &1,
            &self.version,
            2.0,
            0.0,
            DrawParams::color(MessagePage::WHITE),
        );
    }

    fn state(&self) -> LoadingState {
        self.state
    }
}
