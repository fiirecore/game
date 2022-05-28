use std::ops::Deref;

use worldcli::worldlib::character::player::InitPlayerCharacter;

use crate::{
    command::CommandProcessor,
    engine::{egui, App},
    pokedex::{item::Item, moves::Move, pokemon::Pokemon},
};

pub struct Console {
    alive: bool,
    should_focus: bool,
    command: String,
    error: Vec<Error>,
    processor: CommandProcessor,
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
    pub fn new() -> (Console, CommandProcessor) {
        let command = CommandProcessor::default();
        (
            Self {
                alive: Default::default(),
                should_focus: Default::default(),
                command: Default::default(),
                error: Default::default(),
                processor: command.clone(),
            },
            command,
        )
    }

    pub fn reset(&mut self) {
        self.command.clear();
        self.should_focus = true;
        self.error.clear();
    }

    pub fn ui<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &mut self,
        app: &mut App,
        egui: &egui::Context,
        player: Option<&mut InitPlayerCharacter<P, M, I>>,
    ) {
        let mut despawn = false;
        if self.alive {
            egui::Window::new("Console").show(egui, |ui| {
                let response = ui.add(egui::TextEdit::singleline(&mut self.command));
                if self.should_focus {
                    response.request_focus();
                    self.should_focus = false;
                }
                if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    response.request_focus();
                    self.processor
                        .commands
                        .borrow_mut()
                        .push(std::mem::take(&mut self.command));
                }
                self.error.extend(
                    self.processor
                        .errors
                        .borrow_mut()
                        .drain(..)
                        .map(Error::from),
                );
                for error in self.error.iter_mut() {
                    ui.label(error.text);
                    error.timer -= app.timer.delta_f32();
                }
                self.error.retain(|error| error.timer > 0.0);
                if ui.button("Close").clicked() || ui.input().key_pressed(egui::Key::Escape) {
                    despawn = true;
                }
            });
        }
        if app
            .keyboard
            .was_pressed(worldcli::engine::notan::prelude::KeyCode::Slash)
            || despawn
        {
            self.alive = !self.alive;
            self.reset();
            if let Some(player) = player {
                if self.alive {
                    player.character.input_lock.increment();
                } else {
                    player.character.input_lock.decrement();
                }
            }
        }
        // match self.alive {
        //     true => {
        //         self.flicker += delta;
        //         if self.flicker > 2.0 {
        //             self.flicker -= 2.0;
        //         }

        //         if let Some(error) = self.error.as_mut() {
        //             error.timer -= delta;
        //             if error.timer <= 0.0 {
        //                 self.error = None;
        //             }
        //         }

        //         if keyboard::pressed(ctx, Key::Slash) || keyboard::pressed(ctx, Key::Escape) {
        //             self.despawn(player);
        //             return None;
        //         }

        //         if keyboard::pressed(ctx, Key::Left) && self.position > 0 {
        //             self.position(false);
        //         }

        //         if keyboard::pressed(ctx, Key::Right) && self.position < self.command.len() {
        //             self.position(true);
        //         }

        //         if keyboard::pressed(ctx, Key::Backspace)
        //             && self.position <= self.command.len()
        //             && self.position > 0
        //         {
        //             self.command.remove(self.position - 1);
        //             self.position(false);
        //         }

        //         if keyboard::pressed(ctx, Key::Enter) {
        //             self.previous = std::mem::take(&mut self.command);
        //             self.position = 0;

        //             let mut args = self.previous.split_ascii_whitespace();

        //             if let Some(command) = args.next() {
        //                 return Some(CommandResult { command, args });
        //             } else {
        //                 self.error = Some(Error::from("Could not parse command!"));
        //                 return None;
        //             }
        //         } else {
        //             while let Some(char) = keyboard::get_char_queue(ctx) {
        //                 self.command.insert(self.position, char);
        //                 self.position(true);
        //             }
        //         }
        //     }
        //     false => {
        //         if keyboard::pressed(ctx, Key::Slash) {
        //             self.spawn(ctx, player);
        //         }
        //     }
        // }
        // None
    }
}
