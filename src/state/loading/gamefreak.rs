use tinystr::tinystr16;

use crate::engine::{
    audio::play_music,
    error::ImageError,
    graphics::{draw_rectangle, Color, DrawParams, Texture},
    Context,
};

use super::{fade_in, LoadingState};

const BACKGROUND_COLOR: Color = Color::new(24.0 / 255.0, 40.0 / 255.0, 72.0 / 255.0, 1.0);

pub struct GamefreakLoadingScene {
    state: LoadingState,
    accumulator: f32,
    rect_size: f32,
    star: Texture,
    logo: Texture,
    text: Texture,
}

impl super::LoadingScene for GamefreakLoadingScene {
    fn new(ctx: &mut Context) -> Result<Self, ImageError> {
        Ok(Self {
            state: LoadingState::Continue,
            rect_size: 0.0,
            accumulator: 0.0,
            star: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/loading/star.png"),
            )?,
            logo: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/loading/logo.png"),
            )?,
            text: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/loading/text.png"),
            )?,
        })
    }

    fn start(&mut self, ctx: &mut Context) {
        self.state = LoadingState::Continue;
        self.accumulator = 0.0;
        self.rect_size = 0.0;
        play_music(ctx, &tinystr16!("gamefreak"));
    }

    fn update(&mut self, ctx: &mut Context, delta: f32) {
        self.accumulator += delta;
        if self.rect_size < 96.0 {
            self.rect_size += delta * 480.0;
            if self.rect_size > 96.0 {
                self.rect_size = 96.0;
            }
        }
        if self.accumulator > 8.5 {
            self.state = LoadingState::End;
        }
    }

    fn draw(&self, ctx: &mut Context) {
        draw_rectangle(
            ctx,
            0.0,
            (crate::HEIGHT - self.rect_size) / 2.0,
            240.0,
            self.rect_size,
            BACKGROUND_COLOR,
        );

        if self.accumulator < 2.0 {
            self.star.draw(
                ctx,
                crate::WIDTH - self.accumulator * 240.0,
                64.0 + self.accumulator * 5.0,
                DrawParams {
                    rotation: self.accumulator * 2.0,
                    ..Default::default()
                },
            );
        }

        fade_in(ctx, &self.logo, 108.0, 45.0, self.accumulator - 6.0, 1.0); //108x, 12y
        fade_in(ctx, &self.text, 51.0, 74.0, self.accumulator - 4.0, 1.0); //51x, 41y

        // draw_text_left(
        // 	ctx,
        //     &1,
        //     &format!(
        //         "A Button is{:?}Key",
        //         keys(&Control::A).expect("Could not get keys for A button")
        //     ),
        //     &Message::White,
        //     5.0,
        //     5.0,
        // );
        // draw_text_left(
        //     1,
        //     &format!(
        //         "B Button is{:?}Key",
        //         keys(&Control::B).expect("Could not get keys for B button")
        //     ),
        //     &Message::White,
        //     125.0,
        //     5.0,
        // );
        // draw_text_left(1, "D-Pad is Arrow Keys", &Message::White, 5.0, 15.0);
    }

    fn state(&self) -> LoadingState {
        self.state
    }
}
