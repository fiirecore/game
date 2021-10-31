use std::borrow::Cow;

use engine::{
    graphics::draw_text_center,
    gui::Panel,
    input::{pressed, Control},
    tetra::{graphics::Color, input, math::Vec2},
    text::TextColor,
    EngineContext,
};

pub struct Button {
    pub origin: Vec2<f32>,
    pub button: ButtonBase,
}

impl Button {
    pub fn new(
        position: Vec2<f32>,
        size: Vec2<f32>,
        text: Cow<'static, str>,
    ) -> Self {
        Self {
            origin: position,
            button: ButtonBase::new(size, text),
        }
    }

    pub fn update(
        &mut self,
        ctx: &EngineContext,
        mouse: Option<Vec2<f32>>,
        selected: bool,
    ) -> (bool, bool) {
        self.button.update(ctx, &self.origin, mouse, selected)
    }

    pub fn state(&self) -> &ButtonState {
        self.button.state()
    }

    pub fn draw(&self, ctx: &mut EngineContext) {
        self.button.draw(ctx, self.origin)
    }
}

pub struct ButtonBase {
    pub size: Vec2<f32>,
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

impl ButtonBase {
    pub const NONE: Color = Color::WHITE;
    pub const HOVERED: Color = Color::rgb(0.9, 0.9, 0.9);
    pub const CLICKED: Color = Color::rgb(0.8, 0.8, 0.8);

    pub fn new(size: Vec2<f32>, text: Cow<'static, str>) -> Self {
        Self {
            size,
            state: Default::default(),
            text,
        }
    }

    pub fn update(
        &mut self,
        ctx: &EngineContext,
        origin: &Vec2<f32>,
        mouse: Option<Vec2<f32>>,
        selected: bool,
    ) -> (bool, bool) {
        if selected {
            if pressed(ctx, Control::A) {
                self.state = ButtonState::Clicked;
                return (true, false);
            } else {
                self.state = ButtonState::Hovered;
            }
        }
        if let Some(mouse) = mouse {
            if mouse.x > origin.x
                && mouse.y > origin.y
                && mouse.x < origin.x + self.size.x
                && mouse.y < origin.y + self.size.y
            /*(*origin + self.size).gt(&mouse)*/
            {
                if input::is_mouse_button_pressed(ctx, input::MouseButton::Left) {
                    self.state = ButtonState::Clicked;
                    return (true, true);
                } else {
                    self.state = ButtonState::Hovered;
                    return (false, true);
                }
            } else {
                self.state = ButtonState::None;
            }
        } else if input::is_mouse_button_pressed(ctx, input::MouseButton::Left)
            && matches!(self.state, ButtonState::Hovered)
        {
            self.state = ButtonState::Clicked;
            return (true, true);
        }
        (false, false)
    }

    pub fn state(&self) -> &ButtonState {
        &self.state
    }

    pub fn draw(&self, ctx: &mut EngineContext, origin: Vec2<f32>) {
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
            TextColor::Black,
            center.x,
            center.y,
            true,
        );
    }
}
