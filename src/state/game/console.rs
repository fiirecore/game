use std::collections::VecDeque;

use game::{
    util::{Entity, Reset},
    tetra::{Context, input::{self, Key}},
    log::warn,
    game::CommandResult,
};

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
                if self.commands.len() == 0 {
                    self.commands.push_front(String::new());
                }
                if input::is_key_pressed(ctx, Key::Slash) || input::is_key_pressed(ctx, Key::Escape) {
                    self.despawn();
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

                    if let Some(command) = self.commands.get(self.position) {
                        let mut args = command.split_ascii_whitespace();

                        let command = match args.next() {
                            Some(command) => command,
                            None => {
                                game::log::warn!("Could not parse command {}!", command);
                                self.alive = false;
                                return None;
                            },
                        };
    
                        let args = args.collect();
    
                        self.alive = false;

                        return Some(
                            CommandResult {
                                command,
                                args,
                            }
                        );
                    }
                } else {
                    if let Some(new) = input::get_text_input(ctx) {
                        match self.commands.get_mut(self.position) {
                            Some(command) => command.push_str(new),
                            None => warn!("Could not get current command at {}!", self.position),
                        }
                    }
                }
            },
            false => if input::is_key_pressed(ctx, Key::Slash) && game::is_debug() {
                self.spawn();
            },
        }
        None        
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.alive {
            if let Some(command) = self.commands.get(self.position) {
                const Y: f32 = game::util::HEIGHT - 30.0;
                game::graphics::draw_rectangle(ctx, 8.0, Y, game::graphics::text_len(&1, command) + 10.0, 18.0, game::tetra::graphics::Color::BLACK);
                game::graphics::draw_text_left(ctx, &1, "/", &game::text::TextColor::White, 10.0, Y);
                game::graphics::draw_text_left(ctx, &1, command, &game::text::TextColor::White, 16.0, Y);
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