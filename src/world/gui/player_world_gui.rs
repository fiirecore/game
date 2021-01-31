use crate::io::data::player::PlayerData;
use crate::util::input;
use crate::util::texture::Texture;
use crate::util::input::Control;
use crate::util::text_renderer::TextRenderer;
use crate::gui::button::BasicButton;
use crate::gui::Activatable;
use crate::gui::GuiComponent;
use crate::util::render::draw;
use crate::util::texture::byte_texture;

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

    next: u8,

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

            next: 0,

        }

    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        self.focused = !self.focused;
    }

}

impl GuiComponent for PlayerWorldGui {

    fn enable(&mut self) {
        self.active = true;
        self.focus();
    }

    fn disable(&mut self) {
        self.active = false;
        self.unfocus();
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn update_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn render(&self, tr: &TextRenderer) {
        if self.is_active() {
            draw(self.background, self.x, self.y);
            self.save_button.render(tr);
            self.pokemon_button.render(tr);
            self.exit_menu_button.render(tr);
            self.exit_game_button.render(tr);
            tr.render_cursor(self.x + 8.0, self.y + 9.0 + 15.0 * self.cursor_position as f32);
        }
    }

}

impl Activatable for PlayerWorldGui {

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn in_focus(&mut self) -> bool {
        return self.focused;
    }

    fn input(&mut self, _delta: f32) {

        if input::pressed(Control::A) {
            match self.cursor_position {
                0 => {
                    // Save
                    macroquad::prelude::collections::storage::get_mut::<PlayerData>().expect("Could not get Player Data").mark_dirty();
                },
                1 => {
                    // Pokemon
                    
                },
                2 => {
                    // Exit Game
                    *crate::QUIT_SIGNAL.write() = true;
                },
                3 => {
                    // Exit Menu
                    self.disable();
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

    fn next(&self) -> u8 {
        return self.next;
    }

}

#[derive(Debug, enum_iterator::IntoEnumIterator)]
enum Button {

    Save,
    Pokemon,
    ExitGame,
    Close,

}

impl Button {

    pub fn name(&self) -> &str {
        match *self {
            Button::Save => "Save",
            Button::Pokemon => "POKEMON",
            Button::ExitGame => "EXIT GAME",
            Button::Close => "CLOSE",
        }
    }

    pub fn position(&self) -> u8 {
        match *self {
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