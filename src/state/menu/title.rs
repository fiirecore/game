use crate::state::menu::{MenuActions, MenuStates};

use worldcli::worldlib::events::Sender;

use crate::engine::{
    music::{play_music, stop_music},
    controls::{pressed, Control},
    error::ImageError,
    graphics::Texture,
    Context, EngineContext,
};

pub struct TitleState {
    accumulator: f32,

    background: Texture,
    title: Texture,
    trademark: Texture,
    subtitle: Texture,
    charizard: Texture,
    start: Texture,

    sender: Sender<MenuActions>,
}

impl TitleState {
    pub(crate) fn new(ctx: &mut Context, sender: Sender<MenuActions>) -> Result<Self, ImageError> {
        Ok(Self {
            accumulator: 0.0,
            background: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/background.png"),
            )?,
            title: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/title.png"),
            )?,
            trademark: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/trademark.png"),
            )?,
            subtitle: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/subtitle.png"),
            )?,
            charizard: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/charizard.png"),
            )?,
            start: Texture::new(
                ctx,
                include_bytes!("../../../build/assets/scenes/title/start.png"),
            )?,
            sender,
        })
    }
}

impl TitleState {
    pub fn start(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        play_music(ctx, eng, &"title".parse().unwrap());
        self.accumulator = 0.0;
    }

    pub fn end(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        stop_music(ctx, eng);
    }

    pub fn update(&mut self, ctx: &mut Context, eng: &EngineContext, delta: f32) {
        if pressed(ctx, eng, Control::A) {
            let seed = (self.accumulator as usize % u8::MAX as usize) as u8;
            self.sender.send(MenuActions::Seed(seed));
            // self.action = Some(MenuStateAction::SeedAndGoto(seed, MenuStates::MainMenu));
            self.sender.send(MenuActions::Goto(MenuStates::MainMenu));
        }
        self.accumulator += delta;
        // Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) {
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
