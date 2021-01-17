use opengl_graphics::{GlGraphics, Texture};
use piston_window::Context;
use crate::util::text_renderer::TextRenderer;

use crate::util::context::GameContext;
use crate::gui::gui::{Activatable, GuiComponent};

use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;

use crate::util::{file::asset_as_pathbuf, texture_util::texture_from_path, render_util::draw};

use super::move_panel::MovePanel;
pub struct FightPanel {

    active: bool,
    focus: bool,

    x: isize,
    y: isize,
    panel_x: isize,
    panel_y: isize,

    background: Texture,
    move_panel: MovePanel,

    pub cursor_position: u8,
    next: u8,

    move_names: Vec<String>,

}

impl FightPanel {

    pub fn new(panel_x: isize, panel_y: isize) -> FightPanel {

        let x = 0;
        let y = 0;

        FightPanel {

            active: false,
            focus: false,

            x: x,
            y: y,
            panel_x: panel_x,
            panel_y: panel_y,

            background: texture_from_path(asset_as_pathbuf("gui/battle/move_panel.png")),
            move_panel: MovePanel::new(x + panel_x, y + panel_y),

            cursor_position: 0,
            next: 0,

            move_names: Vec::new(),

        }

    }

    pub fn update_names(&mut self, instance: &PokemonInstance) {
        self.move_names = Vec::new();
        for moves in &instance.moves {        
            self.move_names.push(moves.move_instance.name.clone());
        }
    }

    fn reset_vars(&mut self) {
        self.next = 0;
        self.cursor_position = 0;
    }

}

impl GuiComponent for FightPanel {

    fn enable(&mut self) {
        self.active = true;
        self.move_panel.enable();
        self.reset_vars();
    }

    fn disable(&mut self) {
        self.active = false;
        self.move_panel.disable();
        self.unfocus();
    }

    fn is_active(& self) -> bool {
        self.active
    }

    fn update(&mut self, _context: &mut GameContext) {
        if self.is_active() {

        }
    }

    fn update_position(&mut self, panel_x: isize, panel_y: isize) {
        self.panel_x = panel_x;
        self.panel_y = panel_y;
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        if self.is_active() {

            draw(ctx, g, &self.background, self.x + self.panel_x, self.y + self.panel_y);
            self.move_panel.render(ctx, g, tr);
        
            for string_id in 0..self.move_names.len() {
                let mut x_offset = 0;
                let mut y_offset = 0;
                if string_id % 2 == 1 {
                    x_offset = 72;
                }
                if string_id / 2 == 1 {
                    y_offset = 17;
                }
                tr.render_text_from_left(ctx, g, 0, self.move_names[string_id].to_uppercase().as_str(), self.panel_x + self.x + 16 + x_offset, self.panel_y + self.y + 8 + y_offset);
                if string_id == self.cursor_position as usize {
                    tr.render_cursor(ctx, g, self.panel_x + self.x + 10 + x_offset, self.panel_y + self.y + 10 + y_offset);
                }
            }
        }        
    }

}

impl Activatable for FightPanel {

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
        if context.keys[1] == 1 {
            self.next = 1;
        }
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
        if self.cursor_position >= self.move_names.len() as u8 {
            if self.cursor_position == 0 {
                self.cursor_position = 0;
            } else {
                self.cursor_position = self.move_names.len() as u8 - 1;
            }
            
        }
    }

    fn next(&self) -> u8 {
        self.next
    }

}