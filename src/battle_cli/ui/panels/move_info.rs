use crate::{
    pokedex::moves::instance::MoveInstance,
    gui::Panel,
    text::TextColor,
    graphics::{draw_text_left, draw_text_right},
    tetra::Context,
};

pub struct MoveInfoPanel {

    background: Panel,
    pp: String,
    move_type: String,

}

impl MoveInfoPanel {

    // const ORIGIN: Vec2 = const_vec2!([160.0, 113.0]);

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            background: Panel::new(ctx),
            pp: String::from("x/y"),
            move_type: String::from("TYPE/"),
        }
    }

    pub fn update_move(&mut self, instance: &MoveInstance) {
        let move_ref = instance.move_ref.value();
        self.pp = format!("{}/{}", instance.pp, move_ref.pp);
        self.move_type = format!("TYPE/{:?}", move_ref.pokemon_type);
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.background.draw(ctx, 160.0, 113.0, 80.0, 47.0);
        draw_text_left(ctx, &0, "PP", &TextColor::Black, 168.0, 124.0);
        draw_text_left(ctx, &0, &self.move_type, &TextColor::Black, 168.0, 140.0);
        draw_text_right(ctx, &0, &self.pp, &TextColor::Black, 232.0, 124.0);
    }

}