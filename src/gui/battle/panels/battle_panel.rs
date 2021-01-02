use piston_window::Context;
use opengl_graphics::GlGraphics;
use crate::gui::basic_button::BasicButton;
use crate::engine::game_context::GameContext;
use crate::entity::util::direction::Direction;
use crate::engine::engine::Texture;
use crate::engine::text::TextRenderer;

use crate::gui::gui::{GuiComponent, Activatable};

use crate::gui::gui::BasicText;

use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;

use crate::util::{file_util::asset_as_pathbuf, texture_util::texture_from_path, render_util::draw};

pub struct BattlePanel {

    active: bool,
    focus: bool,

	x: isize,
    y: isize,
    panel_x: isize,
    panel_y: isize,
    
    background: Texture,
    
    pub cursor_position: u8,
    pub next: u8,

    pub fight_button: BasicButton,
    pub bag_button: BasicButton,
    pub pokemon_button: BasicButton,
    pub run_button: BasicButton,

    top_text: BasicText,
    bottom_text: BasicText,

}

impl BattlePanel {

	pub fn new(panel_x: isize, panel_y: isize) -> Self {

        let x = 121;
        let y = 0;

        let font_id = 2;

        let text_y = 10;

		Self {

            active: false,
            focus: false,
			
			x: x,
            y: y,
            panel_x: panel_x,
            panel_y: panel_y,
            
            background: texture_from_path(asset_as_pathbuf("gui/battle/button_panel.png")),

            cursor_position: 0,
            next: 0,

            fight_button: BasicButton::new("Fight".to_uppercase().as_str(), font_id, 17, text_y, x + panel_x, y + panel_y),
            bag_button: BasicButton::new("Bag".to_uppercase().as_str(), font_id, 73, text_y, x + panel_x, y + panel_y),
            pokemon_button: BasicButton::new("Pokemon".to_uppercase().as_str(), font_id, 17, text_y + 16, x + panel_x, y + panel_y),
            run_button: BasicButton::new("Run".to_uppercase().as_str(), font_id, 73, text_y + 16, x + panel_x, y + panel_y),

            top_text: BasicText::new("What will", 1, Direction::Left, -111, 10, x + panel_x, y + panel_y),
            bottom_text: BasicText::new("POKEMON do?", 1, Direction::Left, -111, 26, x + panel_x, y + panel_y),

		}
    }
    
    pub fn update_text(&mut self, instance: &PokemonInstance) {
        self.bottom_text.text = instance.pokemon.name.clone();
        self.bottom_text.text.push_str(" do?");
    }

}

impl GuiComponent for BattlePanel {

	fn enable(&mut self) {
        self.active = true;
        self.fight_button.enable();
        self.bag_button.enable();
        self.pokemon_button.enable();
        self.run_button.enable();
        self.cursor_position = 0;
        self.next = 0;
	}

	fn disable(&mut self) {
        self.active = false;
        self.unfocus();
        self.fight_button.disable();
        self.bag_button.disable();
        self.pokemon_button.disable();
        self.run_button.disable();
        self.next = 0;
	}

	fn is_active(& self) -> bool {
		self.active
	}

	fn update(&mut self, _context: &mut GameContext) {
        if self.is_active() {

        }
	}

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		if self.is_active() {
            draw(ctx, g, &self.background, self.panel_x + self.x, self.panel_y + self.y);
            let mut cursor_x = 0;
            let mut cursor_y = 0;
            match self.cursor_position {
                1 => {
                    cursor_x = 56;
                },
                2 => {
                    cursor_y = 16;
                },
                3 => {
                    cursor_x = 56;
                    cursor_y = 16;
                },
                _ => {}
            }
            self.top_text.render(ctx, g, tr);
            self.bottom_text.render(ctx, g, tr);
            tr.render_cursor(ctx, g, self.panel_x + self.x + 10 + cursor_x, self.panel_y + self.y + 13 + cursor_y);
            self.fight_button.render(ctx, g, tr);
            self.bag_button.render(ctx, g, tr);
            self.pokemon_button.render(ctx, g, tr);
            self.run_button.render(ctx, g, tr);
            
		}		
	}

	fn update_position(&mut self, panel_x: isize, panel_y: isize) {
        self.panel_x = panel_x;
        self.panel_y = panel_y;
        self.fight_button.update_position(panel_x, panel_y);
        self.bag_button.update_position(panel_x, panel_y);
        self.pokemon_button.update_position(panel_x, panel_y);
        self.run_button.update_position(panel_x, panel_y);
	}

	fn load(&mut self) {

	}
}

impl Activatable for BattlePanel {

    fn focus(&mut self) {
        self.focus = true;
    }

    fn unfocus(&mut self) {
        self.focus = false;
    }

    fn in_focus(&mut self) -> bool {
        self.focus
    }

    fn input(&mut self, context: &mut GameContext) {
        if context.keys[2] == 1 {
            if self.cursor_position >= 2 {
                self.cursor_position -= 2;
            }            
        } else if context.keys[3] == 1 {
            if self.cursor_position < 2 {
                self.cursor_position += 2;
            } 
        } else if context.keys[4] == 1 {
            if self.cursor_position > 0 {
                self.cursor_position -= 1;
            }
        } else if context.keys[5] == 1 {
            if self.cursor_position < 3 {
                self.cursor_position += 1;
            }
        }
    }

    fn next(&self) -> u8 {
        self.next
    }

}