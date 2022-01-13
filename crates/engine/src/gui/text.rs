use fiirengine::{graphics::DrawParams, math::Vec2, Context};
use firecore_text::MessageState;

use crate::{
    controls::{pressed, Control},
    graphics::{draw_button_for_text, draw_text_left},
    text::FontId,
    EngineContext,
};

#[derive(Debug, Default, Clone)]
pub struct MessageBox {
    origin: Vec2,
    button: Button,
}

#[derive(Default, Debug, Clone, Copy)]
struct Button {
    position: f32,
    direction: bool,
}

impl MessageBox {
    pub fn new(origin: Vec2) -> Self {
        Self {
            origin,
            button: Default::default(),
        }
    }

    pub fn update<C: Clone + Into<[f32; 4]>>(
        &mut self,
        ctx: &Context,
        eng: &EngineContext,
        delta: f32,
        state: &mut Option<MessageState<FontId, C>>,
    ) {
        let mstate = state;
        if let Some(state) = mstate {
            match state.pages.get(state.page) {
                Some(page) => match state.waiting {
                    false => {
                        if (state.accumulator as usize)
                            < page
                                .lines
                                .get(state.line)
                                .map(String::len)
                                .unwrap_or_default()
                        {
                            state.accumulator += delta * 30.0;
                        } else {
                            match state.scroll < (state.line.saturating_sub(1) << 4) as f32 {
                                true => {
                                    state.scroll += delta * 32.0;
                                }
                                false => {
                                    state.accumulator = 0.0;
                                    if state.line < page.lines.len() - 1 {
                                        state.line += 1;
                                    } else {
                                        state.waiting = true;
                                    }
                                }
                            }
                        }
                    }
                    true => match page.wait {
                        Some(wait) => {
                            state.accumulator += delta;
                            if state.accumulator.abs() >= wait.abs() {
                                if self.finish_waiting(state) {
                                    *mstate = None;
                                }
                            }
                        }
                        None => match pressed(ctx, eng, Control::A) {
                            true => {
                                if self.finish_waiting(state) {
                                    *mstate = None;
                                }
                            }
                            false => {
                                self.button.position += match self.button.direction {
                                    true => delta,
                                    false => -delta,
                                } * 7.5;

                                if self.button.position.abs() > 3.0 {
                                    self.button.direction = !self.button.direction;
                                }
                            }
                        },
                    },
                },
                None => *mstate = None,
            }
        }
    }

    #[must_use]
    fn finish_waiting<C: Clone + Into<[f32; 4]>>(
        &mut self,
        state: &mut MessageState<FontId, C>,
    ) -> bool {
        state.waiting = false;
        match state.page + 1 >= state.pages() {
            true => true,
            false => {
                state.page += 1;
                state.reset_page();
                false
            }
        }
    }

    pub fn draw<C: Clone + Into<[f32; 4]>>(
        &self,
        ctx: &mut Context,
        eng: &EngineContext,
        state: Option<&MessageState<FontId, C>>,
    ) {
        if let Some(state) = state {
            if let Some(page) = state.pages.get(state.page) {
                if let Some(line) = page.lines.get(state.line) {
                    let len = state.accumulator as usize;

                    let (string, finished) = line
                        .char_indices()
                        .nth(len)
                        .filter(|_| !state.waiting)
                        .map(|(len, ..)| line.get(..len).map(|l| (l, false)))
                        .flatten()
                        .unwrap_or_else(|| (line.as_str(), state.line + 1 >= page.lines.len()));

                    let y = (state.line << 4) as f32 - state.scroll;
                    let color = page.color.clone().into().into();

                    draw_text_left(
                        ctx,
                        eng,
                        &state.font,
                        string,
                        self.origin.x,
                        self.origin.y + y,
                        DrawParams::color(color),
                    );

                    for index in 0..state.line {
                        let y = (index << 4) as f32 - state.scroll;
                        if y > -2.0 {
                            draw_text_left(
                                ctx,
                                eng,
                                &state.font,
                                &page.lines[index],
                                self.origin.x,
                                self.origin.y + y,
                                DrawParams::color(color),
                            )
                        }
                    }

                    if finished && page.wait.is_none() {
                        draw_button_for_text(
                            ctx,
                            eng,
                            &state.font,
                            line,
                            self.origin.x,
                            self.origin.y + 2.0 + self.button.position + y,
                            DrawParams::default(),
                        );
                    }
                }
            }
        }
    }
}
