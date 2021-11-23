use crate::{
    state::menu::{MenuState, MenuStateAction, MenuStates},
    GameContext,
};

use crate::engine::{
    audio::play_music,
    graphics::Texture,
    input::{pressed, Control},
    Context, State,
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
            background: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/background.png"),
            ).unwrap(),
            title: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/title.png"),
            ).unwrap(),
            trademark: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/trademark.png"),
            ).unwrap(),
            subtitle: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/subtitle.png"),
            ).unwrap(),
            charizard: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/charizard.png"),
            ).unwrap(),
            start: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/start.png"),
            ).unwrap(),
            accumulator: 0.0,
        }
    }
}

impl<'d> State<GameContext> for TitleState {
    fn start(&mut self, ctx: &mut GameContext) {
        play_music(&mut ctx.engine, &"title".parse().unwrap());
        self.accumulator = 0.0;
    }

    fn update(&mut self, ctx: &mut GameContext, delta: f32) {
        if pressed(&ctx.engine, Control::A) {
            let seed = self.accumulator as u64 % u8::MAX as u64;
            self.action = Some(MenuStateAction::SeedAndGoto(seed, MenuStates::MainMenu));
        }
        self.accumulator += delta;
        // Ok(())
    }

    fn draw(&mut self, ctx: &mut GameContext) {
        self.background.draw(ctx, 0.0, 0.0, Default::default());
        self.title.draw(ctx, 3.0, 3.0, Default::default());
        self.trademark.draw(ctx, 158.0, 53.0, Default::default());
        self.subtitle.draw(ctx, 52.0, 57.0, Default::default());
        if self.accumulator as u8 % 2 == 1 {
            self.start.draw(ctx, 44.0, 130.0, Default::default());
        }
        self.charizard.draw(ctx, 129.0, 49.0, Default::default());
        // Ok(())
    }
}

impl<'d> MenuState<'d> for TitleState {
    fn next(&mut self) -> &mut Option<MenuStateAction> {
        &mut self.action
    }
}
