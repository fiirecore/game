use std::collections::VecDeque;

use crate::{
    engine::{
        tetra::{
            input::{self, Key},
            Context,
            graphics::Color,
        },
        util::{Entity, Reset, HEIGHT},
        graphics,
        text::TextColor,
    },
    game::CommandResult,
    is_debug,
};

use log::warn;

#[derive(Default)]
pub struct Console {
    alive: bool,
    commands: VecDeque<String>,
    position: usize,
}

impl Console {
    const MAX_COMMANDS: usize = 10;

    pub fn update(&mut self, ctx: &Context) -> Option<CommandResult> {
        match self.alive {
            true => {
                if self.commands.is_empty() {
                    self.commands.push_front(String::new());
                }
                if input::is_key_pressed(ctx, Key::Slash) || input::is_key_pressed(ctx, Key::Escape)
                {
                    self.despawn();
                    return None;
                }
                if input::is_key_pressed(ctx, Key::Up) {
                    self.position = (self.position + 1).min(self.commands.len().saturating_sub(1));
                }
                if input::is_key_pressed(ctx, Key::Down) {
                    self.position = self.position.saturating_sub(1);
                }
                if input::is_key_pressed(ctx, Key::Backspace) {
                    if let Some(command) = self.commands.get_mut(self.position) {
                        command.pop();
                    }
                }
                if input::is_key_pressed(ctx, Key::Enter) {
                    if self.commands.len() == Self::MAX_COMMANDS {
                        self.commands.pop_back();
                    }

                    self.commands.push_front(String::new());

                    if let Some(command) = self.commands.get(self.position + 1) {
                        let mut args = command.split_ascii_whitespace();

                        let command = match args.next() {
                            Some(command) => command,
                            None => {
                                warn!(
                                    "Could not parse command {} at position {}!",
                                    command, self.position
                                );
                                self.alive = false;
                                return None;
                            }
                        };

                        self.alive = false;

                        return Some(CommandResult { command, args });
                    }
                } else {
                    if let Some(new) = input::get_text_input(ctx) {
                        match self.commands.get_mut(self.position) {
                            Some(command) => command.push_str(new),
                            None => warn!("Could not get current command at {}!", self.position),
                        }
                    }
                }
            }
            false => {
                if input::is_key_pressed(ctx, Key::Slash) && is_debug() {
                    self.spawn();
                }
            }
        }
        None
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.alive {
            if let Some(command) = self.commands.get(self.position) {
                const Y: f32 = HEIGHT - 30.0;
                graphics::draw_rectangle(
                    ctx,
                    8.0,
                    Y,
                    graphics::text_len(&1, command) + 10.0,
                    18.0,
                    Color::BLACK,
                );
                graphics::draw_text_left(
                    ctx,
                    &1,
                    "/",
                    &TextColor::White,
                    10.0,
                    Y,
                );
                graphics::draw_text_left(
                    ctx,
                    &1,
                    command,
                    &TextColor::White,
                    16.0,
                    Y,
                );
            } else {
                warn!("Cannot get string at position {}", self.position);
            }
        }
    }
}

impl Entity for Console {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}

impl Reset for Console {
    fn reset(&mut self) {
        self.position = 0;
    }
}
