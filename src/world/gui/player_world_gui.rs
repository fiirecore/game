use firecore_util::{Entity, text::TextColor};
use firecore_input::{self as input, Control};
use macroquad::prelude::Vec2;

use crate::gui::text::StaticText;
use crate::util::graphics::{Texture, texture::byte_texture, draw};

const BUTTON_COUNT: u8 = 4;

pub struct PlayerWorldGui {

    active: bool,
    focused: bool,

    pos: Vec2,

    background: Texture,

    cursor_position: u8,

    save_button: StaticText,
    pokemon_button: StaticText,
    exit_menu_button: StaticText,
    exit_game_button: StaticText,

}

impl PlayerWorldGui {

    pub fn new() -> Self {

        use Button::*;

        let pos = Vec2::new(169.0, 1.0);

        Self {

            active: false,
            focused: false,

            pos,

            background: byte_texture(include_bytes!("../../../build/assets/gui/world/start_menu.png")),

            cursor_position: 0,

            save_button: Save.to_button(pos),
            pokemon_button: Pokemon.to_button(pos),
            exit_game_button: ExitGame.to_button(pos),
            exit_menu_button: Close.to_button(pos),

        }

    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        self.focused = !self.focused;
    }

    pub fn input(&mut self) {

        if input::pressed(Control::A) {
            match self.cursor_position {
                0 => {
                    // Save
                    unsafe { crate::data::player::DIRTY = true; }
                },
                1 => {
                    // Pokemon
                    crate::gui::game::party::spawn();
                },
                2 => {
                    // Exit Game
                    crate::queue_quit();
                },
                3 => {
                    // Exit Menu
                    self.despawn();
                }
                _ => {},
            }
        }

        if input::pressed(Control::Up) {
            if self.cursor_position > 0 {
                self.cursor_position -= 1;
            } else {
                self.cursor_position = BUTTON_COUNT - 1;
            }    
        } else if input::pressed(Control::Down) {
            if self.cursor_position < BUTTON_COUNT - 1 {
                self.cursor_position += 1;
            } else {
                self.cursor_position = 0;
            }
        }
    }

    pub fn render(&self) {
        if self.is_alive() {
            draw(self.background, self.pos.x, self.pos.y);
            self.save_button.render();
            self.pokemon_button.render();
            self.exit_menu_button.render();
            self.exit_game_button.render();
            crate::util::graphics::draw_cursor(self.pos.x + 8.0, self.pos.y + 9.0 + 15.0 * self.cursor_position as f32);
        }
    }

}

impl Entity for PlayerWorldGui {

    fn spawn(&mut self) {
        self.active = true;
    }

    fn despawn(&mut self) {
        self.active = false;
    }

    fn is_alive(&self) -> bool {
        self.active
    }

}

#[derive(Debug)]
enum Button {

    Save,
    Pokemon,
    ExitGame,
    Close,

}

impl Button {

    pub const fn name(&self) -> &'static str {
        match self {
            Button::Save => "SAVE",
            Button::Pokemon => "POKEMON",
            Button::ExitGame => "EXIT GAME",
            Button::Close => "CLOSE",
        }
    }

    pub const fn position(&self) -> u8 {
        match self {
            Button::Save => 0,
            Button::Pokemon => 1,
            Button::ExitGame => 2,
            Button::Close => 3,
        }
    }

    pub fn to_button(self, panel: Vec2) -> StaticText {
        StaticText::new(vec![self.name().to_owned()], TextColor::Black, 1, false, Vec2::new(15.0, 7.0 + 15.0 * self.position() as f32), panel)
    }

}