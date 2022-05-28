use pokengine::engine::{
    math::const_vec2,
    notan::prelude::{App, Graphics, Plugins},
};
use worldlib::character::player::PlayerCharacter;

use crate::{
    engine::{graphics::Texture, gui::MessageBox, math::Vec2},
    map::input::PlayerInput,
};

pub struct TextWindow {
    background: Texture,
    pub text: MessageBox,
    pub wait: bool,
}

impl TextWindow {
    const ORIGIN: Vec2 = const_vec2!([6.0, 116.0]);
    const TEXT_OFFSET: Vec2 = const_vec2!([11.0, 5.0]);

    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        Ok(Self {
            background: gfx
                .create_texture()
                .from_image(include_bytes!("../../assets/textures/gui/message.png"))
                .build()?,
            text: MessageBox::new(Self::ORIGIN + Self::TEXT_OFFSET),
            wait: false,
        })
    }

    pub fn update<P, B: Default>(
        &mut self,
        app: &App,
        plugins: &mut Plugins,
        delta: f32,
        player: &mut PlayerCharacter<P, B>,
    ) {
        if self.wait {
            // player.character.flags.remove(&PlayerInput::INPUT_LOCK);
            self.wait = false;
        }
        if player.state.message.is_some() {
            // player
            //     .character
            //     .flags
            //     .insert(PlayerInput::INPUT_LOCK, PlayerInput::INPUT_LOCK_VAL);
            if player.state.message.is_none() {
                self.wait = true;
            }
        }
    }

    pub fn ui<P, B: Default>(
        &self,
        app: &App,
        plugins: &mut Plugins,
        gfx: &mut Graphics,
        player: &mut PlayerCharacter<P, B>,
    ) {
            // self.background
            //     .draw(ctx, Self::ORIGIN.x, Self::ORIGIN.y, Default::default());
        MessageBox::ui(app, plugins, gfx, &mut player.state.message);
    }
}
