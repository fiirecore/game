use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;
use crate::util::context::GameContext;
use crate::util::text_renderer::TextRenderer;

use crate::game::player_data_container::PlayerDataContainer;
use crate::gui::basic_button::BasicButton;
use crate::gui::gui::Activatable;
use crate::gui::gui::GuiComponent;
use crate::util::file::asset_as_pathbuf;
use crate::util::render_util::draw;
use crate::util::texture_util::texture_from_path;

static BUTTON_COUNT: u8 = 3;

pub struct PlayerWorldGui {

    active: bool,
    focused: bool,

    x: isize,
    y: isize,

    background: Texture,

    cursor_position: u8,

    save_button: BasicButton,
    pokemon_button: BasicButton,
    exit_button: BasicButton,

    next: u8,

}

impl PlayerWorldGui {

    pub fn new() -> Self {

        let x = 169;
        let y = 1;
        let font_id = 2;

        let text_x = 15;
        let text_y = 7;

        Self {

            active: false,
            focused: false,

            x: x,
            y: y,

            background: texture_from_path(asset_as_pathbuf("gui/world/start_menu.png")),

            cursor_position: 0,

            save_button: BasicButton::new("Save".to_uppercase().as_str(), font_id, text_x, text_y, x, y),
            pokemon_button: BasicButton::new("Pokemon".to_uppercase().as_str(), font_id, text_x, text_y + 15, x, y),
            exit_button: BasicButton::new("Exit".to_uppercase().as_str(), font_id, text_x, text_y + 30, x, y),

            next: 0,

        }

    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        self.focused = !self.focused;
    }

    pub fn click(&mut self, context: &mut GameContext, player_data: &mut PlayerDataContainer) {
        if context.keys[0] == 1 {
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

    fn update_position(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        if self.is_active() {
            draw(ctx, g, &self.background, self.x, self.y);
            self.save_button.render(ctx, g, tr);
            self.pokemon_button.render(ctx, g, tr);
            self.exit_button.render(ctx, g, tr);
            tr.render_cursor(ctx, g, self.x + 8, self.y + 9 + 15 * self.cursor_position as isize);
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

    fn input(&mut self, context: &mut GameContext) {
        if context.keys[2] == 1 {
            if self.cursor_position > 0 {
                self.cursor_position -= 1;
            }            
        } else if context.keys[3] == 1 {
            if self.cursor_position < BUTTON_COUNT - 1 {
                self.cursor_position += 1;
            } 
        }
    }

    fn next(&self) -> u8 {
        return self.next;
    }

}