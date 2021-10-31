use crate::{
    state::menu::{MenuState, MenuStateAction, MenuStates},
    GameContext,
};

use engine::{
    audio::play_music_named,
    input::{pressed, Control},
    tetra::{graphics::Texture, time::get_delta_time, Context, Result, State},
    graphics::{byte_texture, position},
};

pub struct TitleState {
    action: Option<MenuStateAction>,

    accumulator: f32,

    background: Texture,
    title: Texture,
    trademark: Texture,
    subtitle: Texture,
    charizard: Texture,
    start: Texture,
}

impl TitleState {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            action: None,
            background: byte_texture(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/background.png"),
            ),
            title: byte_texture(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/title.png"),
            ),
            trademark: byte_texture(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/trademark.png"),
            ),
            subtitle: byte_texture(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/subtitle.png"),
            ),
            charizard: byte_texture(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/charizard.png"),
            ),
            start: byte_texture(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/start.png"),
            ),
            accumulator: 0.0,
        }
    }
}

impl<'d> State<GameContext<'d>> for TitleState {
    fn begin(&mut self, ctx: &mut GameContext) -> Result {
        play_music_named(&mut ctx.engine, "Title");
        self.accumulator = 0.0;
        Ok(())
    }

    fn update(&mut self, ctx: &mut GameContext) -> Result {
        if pressed(&ctx.engine, Control::A) {
            let seed = self.accumulator as u64 % u8::MAX as u64;
            self.action = Some(MenuStateAction::SeedAndGoto(seed, MenuStates::MainMenu));
        }
        self.accumulator += get_delta_time(ctx).as_secs_f32();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut GameContext) -> Result {
        self.background.draw(ctx, position(0.0, 0.0));
        self.title.draw(ctx, position(3.0, 3.0));
        self.trademark.draw(ctx, position(158.0, 53.0));
        self.subtitle.draw(ctx, position(52.0, 57.0));
        if self.accumulator as u8 % 2 == 1 {
            self.start.draw(ctx, position(44.0, 130.0));
        }
        self.charizard.draw(ctx, position(129.0, 49.0));
        Ok(())
    }
}

impl<'d> MenuState<'d> for TitleState {
    fn next(&mut self) -> &mut Option<MenuStateAction> {
        &mut self.action
    }
}
