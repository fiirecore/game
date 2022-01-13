use worldcli::{worldlib::character::player::PlayerCharacter, map::input::PlayerInput};

use crate::{
    command::CommandResult,
    engine::{
        graphics::{self, Color, DrawParams},
        input::keyboard::{self, Key},
        text::TextColor,
        utils::{Reset, HEIGHT},
        Context, EngineContext,
    },
};

#[derive(Default)]
pub struct Console {
    alive: bool,
    command: String,
    previous: String,
    error: Option<Error>,
    position: usize,
    flicker: f32,
}

struct Error {
    text: &'static str,
    timer: f32,
}

impl From<&'static str> for Error {
    fn from(text: &'static str) -> Self {
        Self { text, timer: 3.0 }
    }
}

impl Console {
    pub fn spawn(&mut self, ctx: &mut Context, player: Option<&mut PlayerCharacter>) {
        while keyboard::get_char_queue(ctx).is_some() {}
        if let Some(player) = player {
            player.character.flags.insert(PlayerInput::INPUT_LOCK);
        }
        self.alive = true;
        self.reset();
    }

    pub fn despawn(&mut self, player: Option<&mut PlayerCharacter>) {
        self.alive = false;
        if let Some(player) = player {
            player.character.flags.remove(&PlayerInput::INPUT_LOCK);
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        delta: f32,
        player: Option<&mut PlayerCharacter>,
    ) -> Option<CommandResult> {
        match self.alive {
            true => {
                self.flicker += delta;
                if self.flicker > 2.0 {
                    self.flicker -= 2.0;
                }

                if let Some(error) = self.error.as_mut() {
                    error.timer -= delta;
                    if error.timer <= 0.0 {
                        self.error = None;
                    }
                }

                if keyboard::pressed(ctx, Key::Slash) || keyboard::pressed(ctx, Key::Escape) {
                    self.despawn(player);
                    return None;
                }

                if keyboard::pressed(ctx, Key::Left) && self.position > 0 {
                    self.position(false);
                }

                if keyboard::pressed(ctx, Key::Right) && self.position < self.command.len() {
                    self.position(true);
                }

                if keyboard::pressed(ctx, Key::Backspace)
                    && self.position <= self.command.len()
                    && self.position > 0
                {
                    self.command.remove(self.position - 1);
                    self.position(false);
                }

                if keyboard::pressed(ctx, Key::Enter) {
                    self.previous = std::mem::take(&mut self.command);
                    self.position = 0;

                    let mut args = self.previous.split_ascii_whitespace();

                    if let Some(command) = args.next() {
                        return Some(CommandResult { command, args });
                    } else {
                        self.error = Some(Error::from("Could not parse command!"));
                        return None;
                    }
                } else {
                    while let Some(char) = keyboard::get_char_queue(ctx) {
                        self.command.insert(self.position, char);
                        self.position(true);
                    }
                }
            }
            false => {
                if keyboard::pressed(ctx, Key::Slash) {
                    self.spawn(ctx, player);
                }
            }
        }
        None
    }

    fn position(&mut self, add: bool) {
        match add {
            true => self.position += 1,
            false => self.position -= 1,
        }
        self.flicker = 1.0;
    }

    pub fn error(&mut self, error: &'static str) {
        self.error = Some(Error::from(error))
    }

    pub fn draw(&self, ctx: &mut Context, eng: &mut EngineContext) {
        const Y: f32 = HEIGHT - 30.0;
        const Y2: f32 = Y - 20.0;

        if let Some(error) = self.error.as_ref() {
            graphics::draw_text_left(ctx, eng, &1, error.text, 8.0, Y2, DrawParams::color(Color::RED));
        }

        if self.alive {
            graphics::draw_rectangle(
                ctx,
                8.0,
                Y,
                graphics::text_len(eng, &1, &self.command) + 10.0,
                18.0,
                Color::BLACK,
            );
            graphics::draw_text_left(ctx, eng, &1, "/", 10.0, Y, DrawParams::color(TextColor::WHITE));
            graphics::draw_text_left(
                ctx,
                eng,
                &1,
                &self.command,
                16.0,
                Y,
                DrawParams::color(TextColor::WHITE),
            );
            if self.flicker >= 1.0 {
                if let Some(text) = self.command.get(..self.position) {
                    graphics::draw_rectangle(
                        ctx,
                        16.0 + graphics::text_len(eng, &1, text) as f32,
                        Y + 1.0,
                        1.0,
                        14.0,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}

impl Reset for Console {
    fn reset(&mut self) {
        self.position = 0;
    }
}
