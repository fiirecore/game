use crate::util::input;
use crate::util::texture::Texture;
use crate::util::input::Control;
use crate::util::text_renderer::TextRenderer;

use crate::game::player_data_container::PlayerDataContainer;
use crate::gui::basic_button::BasicButton;
use crate::gui::gui::Activatable;
use crate::gui::gui::GuiComponent;
use crate::util::render::draw;
use crate::util::texture::byte_texture;

static BUTTON_COUNT: u8 = 3;

pub struct PlayerWorldGui {

    active: bool,
    focused: bool,

    x: f32,
    y: f32,

    background: Texture,

    cursor_position: u8,

    save_button: BasicButton,
    pokemon_button: BasicButton,
    exit_button: BasicButton,

    next: u8,

}

impl PlayerWorldGui {

    pub fn new() -> Self {

        let x = 169.0;
        let y = 1.0;
        let font_id = 2;

        let text_x = 15.0;
        let text_y = 7.0;

        Self {

            active: false,
            focused: false,

            x: x,
            y: y,

            background: byte_texture(include_bytes!("../../../include/gui/world/start_menu.png")),

            cursor_position: 0,

            save_button: BasicButton::new("Save".to_uppercase().as_str(), font_id, text_x, text_y, x, y),
            pokemon_button: BasicButton::new("Pokemon".to_uppercase().as_str(), font_id, text_x, text_y + 15.0, x, y),
            exit_button: BasicButton::new("Exit".to_uppercase().as_str(), font_id, text_x, text_y + 30.0, x, y),

            next: 0,

        }

    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        self.focused = !self.focused;
    }

    pub fn click(&mut self, player_data: &mut PlayerDataContainer) {
        if input::pressed(crate::util::input::Control::A) {
            match self.cursor_position {
                0 => {
                    // Save
                    // context.save_data = true;
                },
                1 => {
                    // Pokemon
                    
                },
                2 => {
                    // Exit
                },
                _ => {},
            }
        }
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
            self.exit_button.render(tr);
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

    fn input(&mut self, delta: f32) {
        if input::pressed(Control::Down) {
            if self.cursor_position > 0 {
                self.cursor_position -= 1;
            }            
        } else if input::pressed(Control::Up) {
            if self.cursor_position < BUTTON_COUNT - 1 {
                self.cursor_position += 1;
            } 
        }
    }

    fn next(&self) -> u8 {
        return self.next;
    }

}