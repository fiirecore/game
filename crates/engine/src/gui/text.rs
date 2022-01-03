use fiirengine::{graphics::DrawParams, math::Vec2, Context};

use crate::{
    controls::{pressed, Control},
    graphics::{draw_button_for_text, draw_text_left},
    text::{FontId, MessagePage},
    utils::{Completable, Entity, Reset}, EngineContext,
};

#[derive(Default, Clone)]
pub struct MessageBox {
    alive: bool,
    origin: Vec2,

    pub font: FontId,
    pub pages: Vec<MessagePage>,

    button: Button,

    page: usize,
    line: usize,
    accumulator: f32,

    waiting: bool,
    finished: bool,
}

#[derive(Default, Clone, Copy)]
struct Button {
    position: f32,
    direction: bool,
}

impl MessageBox {
    pub fn new(origin: Vec2, font: FontId) -> Self {
        Self {
            alive: false,
            origin,
            font,
            pages: Default::default(),
            button: Default::default(),
            page: 0,
            line: 0,
            accumulator: 0.0,
            waiting: false,
            finished: false,
        }
    }

    pub fn page(&self) -> usize {
        self.page
    }

    pub fn pages(&self) -> usize {
        self.pages.len()
    }

    pub fn waiting(&self) -> bool {
        self.waiting
    }

    fn reset_page(&mut self) {
        self.line = 0;
        self.accumulator = 0.0;
    }

    pub fn update(&mut self, ctx: &Context, eng: &EngineContext, delta: f32) {
        if self.alive {
            match self.pages.get(self.page) {
                Some(page) => match self.waiting {
                    false => {
                        if (self.accumulator as usize)
                            < page
                                .lines
                                .get(self.line)
                                .map(String::len)
                                .unwrap_or_default()
                        {
                            self.accumulator += delta * 30.0;
                        } else {
                            self.accumulator = 0.0;
                            if self.line < page.lines.len() - 1 {
                                self.line += 1;
                            } else {
                                self.waiting = true;
                            }
                        }
                    }
                    true => match page.wait {
                        Some(wait) => {
                            self.accumulator += delta;
                            if self.accumulator.abs() >= wait.abs() {
                                self.finish_waiting();
                            }
                        }
                        None => match pressed(ctx, eng, Control::A) {
                            true => self.finish_waiting(),
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
                None => self.finished = true,
            }
        }
    }

    fn finish_waiting(&mut self) {
        self.waiting = false;
        match self.page + 1 < self.pages() {
            false => self.finished = true,
            true => {
                self.page += 1;
                self.reset_page();
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        if self.alive {
            if let Some(page) = self.pages.get(self.page) {
                if let Some(line) = page.lines.get(self.line) {
                    let len = self.accumulator as usize;

                    let (string, finished) = line
                        .char_indices()
                        .nth(len)
                        .filter(|_| !self.waiting)
                        .map(|(len, ..)| line.get(..len).map(|l| (l, false)))
                        .flatten()
                        .unwrap_or_else(|| (line.as_str(), self.line + 1 >= page.lines.len()));

                    let y = (self.line << 4) as f32;
                    draw_text_left(
                        ctx,
                        eng,
                        &self.font,
                        string,
                        self.origin.x,
                        self.origin.y + y,
                        DrawParams::color(page.color),
                    );

                    for index in 0..self.line {
                        draw_text_left(
                            ctx,
                            eng,
                            &self.font,
                            &page.lines[index],
                            self.origin.x,
                            self.origin.y + (index << 4) as f32,
                            DrawParams::color(page.color),
                        );
                    }

                    if finished && page.wait.is_none() {
                        draw_button_for_text(
                            ctx,
                            eng,
                            &self.font,
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

impl Reset for MessageBox {
    fn reset(&mut self) {
        self.page = 0;
        self.reset_page();
        self.finished = false;
        self.button = Default::default();
    }
}

impl Completable for MessageBox {
    fn finished(&self) -> bool {
        self.finished || self.pages.is_empty()
    }
}

impl Entity for MessageBox {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.reset();
        self.pages.clear();
    }

    fn alive(&self) -> bool {
        self.alive
    }
}
