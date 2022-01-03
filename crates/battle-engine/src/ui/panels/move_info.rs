use core::ops::Deref;

use pokedex::{
    engine::{
        graphics::{draw_text_left, draw_text_right, DrawParams},
        gui::Panel,
        text::MessagePage,
        Context, EngineContext,
    },
    moves::{owned::OwnedMove, Move},
};

pub struct MoveInfoPanel {
    pp: String,
    move_type: String,
}

impl MoveInfoPanel {
    // const ORIGIN: Vec2 = const_vec2!([160.0, 113.0]);

    pub fn new() -> Self {
        Self {
            pp: String::from("x/y"),
            move_type: String::from("TYPE/"),
        }
    }

    pub fn update_move<M: Deref<Target = Move>>(&mut self, instance: &OwnedMove<M>) {
        let move_ref = &instance.0;
        self.pp = format!("{}/{}", instance.pp(), move_ref.pp);
        self.move_type = format!("TYPE/{:?}", move_ref.pokemon_type);
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        Panel::draw(ctx, eng, 160.0, 113.0, 80.0, 47.0);
        let p = DrawParams::color(MessagePage::BLACK);
        draw_text_left(ctx, eng, &0, "PP", 168.0, 124.0, p);
        draw_text_left(ctx, eng, &0, &self.move_type, 168.0, 140.0, p);
        draw_text_right(ctx, eng, &0, &self.pp, 232.0, 124.0, p);
    }
}
