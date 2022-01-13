use worldlib::character::player::PlayerCharacter;

use crate::{engine::{
    error::ImageError,
    graphics::Texture,
    gui::MessageBox,
    math::{vec2, Vec2},
    Context, EngineContext,
}, map::input::PlayerInput};

pub struct TextWindow {
    background: Texture,
    pub text: MessageBox,
    pub wait: bool,
}

impl TextWindow {
    const ORIGIN: Vec2 = vec2(6.0, 116.0);
    const TEXT_OFFSET: Vec2 = vec2(11.0, 5.0);

    pub fn new(ctx: &mut Context) -> Result<Self, ImageError> {
        Ok(Self {
            background: Texture::new(
                ctx,
                include_bytes!("../../assets/textures/gui/message.png"),
            )?,
            text: MessageBox::new(Self::ORIGIN + Self::TEXT_OFFSET),
            wait: false,
        })
    }

    pub fn update(&mut self, ctx: &mut Context, eng: &EngineContext, delta: f32, player: &mut PlayerCharacter) {
        if self.wait {
            player.character.flags.remove(&PlayerInput::INPUT_LOCK);
            self.wait = false;
        }
        if player.world.message.is_some() {
            player.character.flags.insert(PlayerInput::INPUT_LOCK);
            self.text
                .update(ctx, eng, delta, &mut player.world.message);
            if player.world.message.is_none() {
                self.wait = true;
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext, player: &PlayerCharacter) {
        if player.world.message.is_some() {
            self.background
                .draw(ctx, Self::ORIGIN.x, Self::ORIGIN.y, Default::default());
            self.text.draw(ctx, eng, player.world.message.as_ref());
        }
    }
}
