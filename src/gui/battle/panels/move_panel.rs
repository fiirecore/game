use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::util::direction::Direction;
use crate::gui::gui::BasicText;
use crate::gui::gui::GuiComponent;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::render_util::draw;
use crate::util::texture_util::texture_from_path;

pub struct MovePanel {

    active: bool,

    x: isize,
    y: isize,
    panel_x: isize,
    panel_y: isize,

    background: Texture,
    pp: BasicText,
    remaining_pp: BasicText,
    move_type: BasicText,

}

impl MovePanel {

    pub fn new(panel_x: isize, panel_y: isize) -> Self {

        let x = 160;
        let y = 0;

        Self {

            active: false,
    
            x: x,
            y: y,
            panel_x: panel_x,
            panel_y: panel_y,

            background: texture_from_path(asset_as_pathbuf("gui/battle/move_info_panel.png")),
            pp: BasicText::new("PP", 0, Direction::Left, 8, 13 - 2, x + panel_x, y + panel_y),
            move_type: BasicText::new("TYPE/", 0, Direction::Left, 8, 27, x + panel_x, y + panel_y),
            remaining_pp: BasicText::new("x/y", 0, Direction::Right, 72, 13 - 2, x + panel_x, y + panel_y),

        }

    }

}

impl GuiComponent for MovePanel {

    fn enable(&mut self) {
        self.active = true;
    }

    fn disable(&mut self) {
        self.active = false;
    }

    fn is_active(&self) -> bool {
        return self.active;
    }
    
    fn update(&mut self, _context: &mut GameContext) {
        
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        if self.is_active() {
            draw(ctx, g, &self.background, self.x + self.panel_x, self.y + self.panel_y);
            self.pp.render(ctx, g, tr);
            self.remaining_pp.render(ctx, g, tr);
            self.move_type.render(ctx, g, tr);
        }
    }

    fn update_position(&mut self, x: isize, y: isize) {
        self.panel_x = x;
        self.panel_y = y;
    }

}