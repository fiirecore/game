use crate::engine::{error::ImageError, Context};

mod manager;
pub use manager::LoadingStateManager;

mod copyright;
mod gamefreak;
mod pokemon;

// pub async fn load_coroutine(ctx: &mut Context) {
//     use crate::engine::input::controls::{pressed, Control};
//     use crate::engine::macroquad::prelude::{clear_background, get_frame_time, next_frame, BLACK};

//     info!("Starting loading scene coroutine");

//     let mut manager = manager::LoadingSceneManager::new(ctx).unwrap();

//     while !manager.finished {
//         if LOADING_FINISHED.get() {
//             if pressed(ctx, Control::A) {
//                 manager.finished = true;
//             }
//         }

//         manager.update(ctx, delta);
//         clear_background(BLACK);
//         manager.render(ctx);
//         next_frame().await;
//     }
// }

#[derive(Clone, Copy)]
pub enum LoadingState {
    Continue,
    Next(LoadingScenes),
    End,
}

#[derive(Clone, Copy)]
pub enum LoadingScenes {
    Copyright,
    Gamefreak,
    Pokemon,
}

impl Default for LoadingScenes {
    fn default() -> Self {
        Self::Copyright
    }
}

pub trait LoadingScene {
    fn new(ctx: &mut Context) -> Result<Self, ImageError>
    where
        Self: Sized;

    fn start(&mut self, ctx: &mut Context);

    fn update(&mut self, ctx: &mut Context, delta: f32);

    fn draw(&self, ctx: &mut Context);

    fn state(&self) -> LoadingState;
}

use crate::engine::graphics::{Color, DrawParams, Texture};

pub fn fade_in_out(
    ctx: &mut Context,
    texture: &Texture,
    x: f32,
    y: f32,
    accumulator: f32,
    end_time: f32,
    fade_time: f32,
) {
    if accumulator < fade_time {
        texture.draw(
            ctx,
            x,
            y,
            DrawParams::color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time)),
        );
    } else if accumulator < end_time - fade_time {
        texture.draw(ctx, x, y, Default::default())
    } else if accumulator < end_time {
        texture.draw(
            ctx,
            x,
            y,
            DrawParams::color(Color::rgba(
                1.0,
                1.0,
                1.0,
                (end_time - accumulator) / fade_time,
            )),
        );
    }
}

pub fn fade_in(
    ctx: &mut Context,
    texture: &Texture,
    x: f32,
    y: f32,
    accumulator: f32,
    fade_time: f32,
) {
    texture.draw(
        ctx,
        x,
        y,
        if accumulator < fade_time {
            DrawParams::color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time))
        } else {
            Default::default()
        },
    );
}
