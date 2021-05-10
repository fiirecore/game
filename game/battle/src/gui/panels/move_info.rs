use game::{
    pokedex::moves::instance::MoveInstance,
    macroquad::prelude::Texture2D,
    text::TextColor,
    graphics::{byte_texture, draw, draw_text_left, draw_text_right},
};

static mut BACKGROUND: Option<Texture2D> = None;

pub struct MoveInfoPanel {

    background: Texture2D,
    pp: String,
    move_type: String,

}

impl MoveInfoPanel {

    // const ORIGIN: Vec2 = const_vec2!([160.0, 113.0]);

    pub fn new() -> Self {

        Self {

            background: unsafe { *BACKGROUND.get_or_insert(byte_texture(include_bytes!("../../../assets/gui/move_info_panel.png"))) },
            pp: String::from("x/y"),
            move_type: String::from("TYPE/"),

        }

    }

    pub fn update_move(&mut self, instance: &MoveInstance) {
        self.pp = format!("{}/{}", instance.pp, instance.pokemon_move.pp);
        self.move_type = format!("TYPE/{:?}", instance.pokemon_move.pokemon_type);
    }

    pub fn render(&self) {
        draw(self.background, 160.0, 113.0);
        draw_text_left(0, "PP", TextColor::Black, 168.0, 124.0);
        draw_text_left(0, &self.move_type, TextColor::Black, 168.0, 140.0);
        draw_text_right(0, &self.pp, TextColor::Black, 232.0, 124.0);
    }

}