
use crate::util::texture::Texture;
use crate::util::text_renderer::TextRenderer;
use crate::io::data::Direction;
use crate::gui::gui::BasicText;
use crate::gui::gui::GuiComponent;
use crate::util::render::draw;
use crate::util::texture::byte_texture;

pub struct MovePanel {

    active: bool,

    x: f32,
    y: f32,
    panel_x: f32,
    panel_y: f32,

    background: Texture,
    pp: BasicText,
    remaining_pp: BasicText,
    move_type: BasicText,

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
            pp: BasicText::new(vec![String::from("PP")], 0, Direction::Left, 8.0, 11.0, x + panel_x, y + panel_y),
            move_type: BasicText::new(vec![String::from("TYPE/")], 0, Direction::Left, 8.0, 27.0, x + panel_x, y + panel_y),
            remaining_pp: BasicText::new(vec![String::from("x/y")], 0, Direction::Right, 72.0, 11.0, x + panel_x, y + panel_y),

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
    
    fn update(&mut self, _delta: f32) {}

    fn render(&self, tr: &TextRenderer) {
        if self.is_active() {
            draw(self.background, (self.x + self.panel_x) as f32, (self.y + self.panel_y) as f32);
            self.pp.render(tr);
            self.remaining_pp.render(tr);
            self.move_type.render(tr);
        }
    }

    fn update_position(&mut self, x: f32, y: f32) {
        self.panel_x = x;
        self.panel_y = y;
    }

}