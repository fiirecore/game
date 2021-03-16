use firecore_util::Entity;
use crate::util::Input;
use firecore_input::{self as input, Control};
use crate::util::graphics::Texture;

use crate::gui::button::BasicButton;
use crate::gui::GuiComponent;
use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;

static BUTTON_COUNT: u8 = 4;

pub struct PlayerWorldGui {

    active: bool,
    focused: bool,

    x: f32,
    y: f32,

    background: Texture,

    cursor_position: u8,

    save_button: BasicButton,
    pokemon_button: BasicButton,
    exit_menu_button: BasicButton,
    exit_game_button: BasicButton,

}

impl PlayerWorldGui {

    pub fn new() -> Self {

        use Button::*;

        let x = 169.0;
        let y = 1.0;

        Self {

            active: false,
            focused: false,

            x: x,
            y: y,

            background: byte_texture(include_bytes!("../../../build/assets/gui/world/start_menu.png")),

            cursor_position: 0,

            save_button: Save.to_button(x, y),
            pokemon_button: Pokemon.to_button(x, y),
            exit_game_button: ExitGame.to_button(x, y),
            exit_menu_button: Close.to_button(x, y),

        }

    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        self.focused = !self.focused;
    }

}

impl GuiComponent for PlayerWorldGui {

    fn update_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn render(&self) {
        if self.is_alive() {
            draw(self.background, self.x, self.y);
            self.save_button.render();
            self.pokemon_button.render();
            self.exit_menu_button.render();
            self.exit_game_button.render();
            crate::util::graphics::draw_cursor(self.x + 8.0, self.y + 9.0 + 15.0 * self.cursor_position as f32);
        }
    }

}

impl Input for PlayerWorldGui {

    fn input(&mut self, _delta: f32) {

        if input::pressed(Control::A) {
            match self.cursor_position {
                0 => {
                    // Save
                    unsafe { crate::io::data::player::DIRTY = true; }
                },
                1 => {
                    // Pokemon
                    crate::gui::game::pokemon_party_gui::spawn(false);
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

// const BUTTONS: [Button; 4] = [
//     Button::Save,
//     Button::Pokemon,
//     Button::ExitGame,
//     Button::Close,
// ];

impl Button {

    pub fn name(&self) -> &str {
        match self {
            Button::Save => "SAVE",
            Button::Pokemon => "POKEMON",
            Button::ExitGame => "EXIT GAME",
            Button::Close => "CLOSE",
        }
    }

    pub fn position(&self) -> u8 {
        match self {
            Button::Save => 0,
            Button::Pokemon => 1,
            Button::ExitGame => 2,
            Button::Close => 3,
        }
    }

    pub fn to_button(self, panel_x: f32, panel_y: f32) -> BasicButton {
        BasicButton::new(self.name(), 2, 15.0, 7.0 + 15.0 * self.position() as f32, panel_x, panel_y)
    }

}