
use firecore_util::Entity;
use crate::util::graphics::Texture;
use firecore_util::text::TextColor;
use crate::gui::text::StaticText;
use crate::gui::GuiComponent;
use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;

pub struct MovePanel {

    active: bool,

    pub has_move: bool,

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

            has_move: false,
    
            x: x,
            y: y,
            panel_x: panel_x,
            panel_y: panel_y,

            background: byte_texture(include_bytes!("../../../../build/assets/gui/battle/move_info_panel.png")),
            pp: StaticText::new(vec![String::from("PP")], TextColor::Black, 0, false, 8.0, 11.0, x + panel_x, y + panel_y),
            move_type: StaticText::new(vec![String::from("TYPE/")], TextColor::Black, 0, false, 8.0, 27.0, x + panel_x, y + panel_y),
            remaining_pp: StaticText::new(vec![String::from("x/y")], TextColor::Black, 0, true, 72.0, 11.0, x + panel_x, y + panel_y),

        }

    }

    pub fn update_with_move(&mut self, pmove: &firecore_pokedex::moves::instance::MoveInstance) {
        self.remaining_pp.text = vec![pmove.remaining_pp.to_string() + "/" + &pmove.pokemon_move.pp.to_string()];
        self.move_type.text = vec![format!("TYPE/{:?}", pmove.pokemon_move.pokemon_type.unwrap_or_default())];
    }

}

impl firecore_util::Reset for MovePanel {
    fn reset(&mut self) {
        self.has_move = false;
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