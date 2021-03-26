
use macroquad::prelude::Vec2;
use crate::util::graphics::Texture;
use firecore_util::text::TextColor;
use crate::gui::text::StaticText;
use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;

pub struct MovePanel {

    pub has_move: bool,

    pos: Vec2,
    panel: Vec2,

    background: Texture,
    pp: StaticText,
    remaining_pp: StaticText,
    move_type: StaticText,

}

impl MovePanel {

    pub fn new(panel: Vec2) -> Self {

        let pos = Vec2::new(160.0, 0.0);

        Self {

            has_move: false,

            background: byte_texture(include_bytes!("../../../../build/assets/gui/battle/move_info_panel.png")),
            pp: StaticText::new(vec![String::from("PP")], TextColor::Black, 0, false, Vec2::new(8.0, 11.0), pos + panel),
            move_type: StaticText::new(vec![String::from("TYPE/")], TextColor::Black, 0, false, Vec2::new(8.0, 27.0), pos + panel),
            remaining_pp: StaticText::new(vec![String::from("x/y")], TextColor::Black, 0, true, Vec2::new(72.0, 11.0), pos + panel),
            
            pos,
            panel,

        }

    }

    pub fn update_with_move(&mut self, pmove: &firecore_pokedex::moves::instance::MoveInstance) {
        self.remaining_pp.text = vec![pmove.remaining_pp.to_string() + "/" + &pmove.pokemon_move.pp.to_string()];
        self.move_type.text = vec![format!("TYPE/{:?}", pmove.pokemon_move.pokemon_type.unwrap_or_default())];
    }

    pub fn render(&self) {
        draw(self.background, self.pos.x + self.panel.x, self.pos.y + self.panel.y);
        self.pp.render();
        self.remaining_pp.render();
        self.move_type.render();
    }

}

impl firecore_util::Reset for MovePanel {
    fn reset(&mut self) {
        self.has_move = false;
    }
}