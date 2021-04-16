use game::{
    util::text::TextColor,
    pokedex::moves::instance::MoveInstance,
    macroquad::prelude::{Vec2, Texture2D},
    graphics::{byte_texture, draw, draw_text_left, draw_text_right}
};

pub struct MoveInfoPanel {

    pos: Vec2,
    panel: Vec2,

    background: Texture2D,
    pp: String,
    remaining_pp: String,
    move_type: String,

}

impl MoveInfoPanel {

    pub fn new(panel: Vec2) -> Self {

        let pos = Vec2::new(160.0, 0.0);

        Self {

            background: byte_texture(include_bytes!("../../../assets/gui/move_info_panel.png")),
            pp: String::from("PP"),
            move_type: String::from("TYPE/"),
            remaining_pp: String::from("x/y"),
            
            pos,
            panel,

        }

    }

    pub fn update_with_move(&mut self, move_instance: &MoveInstance) {
        self.remaining_pp = format!("{}/{}", move_instance.pp, move_instance.pokemon_move.pp);
        self.move_type = format!("TYPE/{:?}", move_instance.pokemon_move.pokemon_type);
    }

    pub fn render(&self) {
        let x = self.pos.x + self.panel.x;
        let y = self.pos.y + self.panel.y;
        draw(self.background, x, y);
        draw_text_left(0, &self.pp, TextColor::Black, x + 8.0, y + 11.0);
        draw_text_left(0, &self.move_type, TextColor::Black, x + 8.0, y + 27.0);
        draw_text_right(0, &self.remaining_pp, TextColor::Black, x + 72.0, y + 11.0);
    }

}

// impl firecore_util::Reset for MoveInfoPanel {
//     fn reset(&mut self) {
//         self.has_move = false;
//     }
// }

// pub struct MoveInfo {

//     move_type: String,
//     pp: String,
//     remaining_pp: String,

// }