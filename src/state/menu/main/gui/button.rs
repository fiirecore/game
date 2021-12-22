use std::borrow::Cow;

use crate::engine::{
    graphics::{draw_text_center, Color, DrawParams},
    gui::Panel,
    input::{
        controls::{pressed, Control},
    },
    math::Vec2,
    text::MessagePage,
    Context,
};

pub struct Button {
    pub size: Vec2,
    pub state: ButtonState,
    pub text: Cow<'static, str>,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    None,
    Hovered,
    Clicked,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::None
    }
}

impl Button {
    pub const NONE: Color = Color::WHITE;
    pub const HOVERED: Color = Color::rgb(0.9, 0.9, 0.9);
    pub const CLICKED: Color = Color::rgb(0.8, 0.8, 0.8);

    pub fn new(size: Vec2, text: Cow<'static, str>) -> Self {
        Self {
            size,
            state: Default::default(),
            text,
        }
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        selected: bool,
    ) -> bool {
        if selected {
            if pressed(ctx, Control::A) {
                self.state = ButtonState::Clicked;
                return true;
            } else {
                self.state = ButtonState::Hovered;
            }
        } else {
            self.state = ButtonState::None;
        }
        false
    }

    pub fn draw(&self, ctx: &mut Context, origin: Vec2) {
        // draw_rectangle(ctx, origin.x, origin.y, self.size.x, self.size.y);
        // draw_rectangle_lines(ctx, origin.x, origin.y, self.size.x, self.size.y, 2.0, Color::BLACK);
        Panel::draw_color(
            ctx,
            origin.x,
            origin.y,
            self.size.x,
            self.size.y,
            match &self.state {
                ButtonState::None => Self::NONE,
                ButtonState::Hovered => Self::HOVERED,
                ButtonState::Clicked => Self::CLICKED,
            },
        );
        let center = origin + self.size / 2.0;
        draw_text_center(
            ctx,
            &1,
            &self.text,
            true,
            center.x,
            center.y,
            DrawParams::color(MessagePage::BLACK),
        );
    }
}
