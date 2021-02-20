
use crate::entity::Entity;
use crate::util::graphics::Texture;

use crate::util::Direction;
use crate::gui::text::StaticText;
use crate::gui::GuiComponent;
use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;

pub struct MovePanel {

    active: bool,

    x: f32,
    y: f32,
    panel_x: f32,
    panel_y: f32,

    background: Texture,
    pp: StaticText,
    remaining_pp: StaticText,
    move_type: StaticText,

}

impl MovePanel {

    pub fn new(panel_x: f32, panel_y: f32) -> Self {

        let x = 160.0;
        let y = 0.0;

        Self {

            active: false,
    
            x: x,
            y: y,
            panel_x: panel_x,
            panel_y: panel_y,

            background: byte_texture(include_bytes!("../../../../build/assets/gui/battle/move_info_panel.png")),
            pp: StaticText::new(vec![String::from("PP")], 0, Direction::Left, 8.0, 11.0, x + panel_x, y + panel_y),
            move_type: StaticText::new(vec![String::from("TYPE/")], 0, Direction::Left, 8.0, 27.0, x + panel_x, y + panel_y),
            remaining_pp: StaticText::new(vec![String::from("x/y")], 0, Direction::Right, 72.0, 11.0, x + panel_x, y + panel_y),

        }

    }

}

impl GuiComponent for MovePanel {

    fn render(&self) {
        if self.is_alive() {
            draw(self.background, (self.x + self.panel_x) as f32, (self.y + self.panel_y) as f32);
            self.pp.render();
            self.remaining_pp.render();
            self.move_type.render();
        }
    }

    fn update_position(&mut self, x: f32, y: f32) {
        self.panel_x = x;
        self.panel_y = y;
    }

}

impl Entity for MovePanel {

    fn spawn(&mut self) {
        self.active = true;
    }

    fn despawn(&mut self) {
        self.active = false;
    }

    fn is_alive(&self) -> bool {
        return self.active;
    }

}