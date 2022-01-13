use core::ops::Deref;

use pokedex::{
    engine::{
        graphics::{draw_cursor, draw_text_left, DrawParams},
        gui::Panel,
        controls::{pressed, Control},
        Context, EngineContext, text::TextColor,
    },
    pokemon::{owned::OwnablePokemon, Pokemon},
};

pub struct BattleOptions {
    buttons: [&'static str; 4],
    pokemon_do: String,
    pub cursor: usize,
}

impl BattleOptions {
    pub fn new() -> Self {
        Self {
            buttons: ["FIGHT", "BAG", "POKEMON", "RUN"],
            pokemon_do: String::new(),
            cursor: 0,
        }
    }

    pub fn setup<P: Deref<Target = Pokemon>, M, I, G, N, H>(
        &mut self,
        instance: &OwnablePokemon<P, M, I, G, N, H>,
    ) {
        self.pokemon_do = format!("{} do?", instance.name());
    }

    pub fn input(&mut self, ctx: &Context, eng: &EngineContext) {
        if pressed(ctx, eng, Control::Up) && self.cursor >= 2 {
            self.cursor -= 2;
        } else if pressed(ctx, eng, Control::Down) && self.cursor <= 2 {
            self.cursor += 2;
        } else if pressed(ctx, eng, Control::Left) && self.cursor > 0 {
            self.cursor -= 1;
        } else if pressed(ctx, eng, Control::Right) && self.cursor < 3 {
            self.cursor += 1;
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        Panel::draw(ctx, eng, 120.0, 113.0, 120.0, 47.0);

        draw_text_left(
            ctx,
            eng,
            &1,
            "What will",
            11.0,
            123.0,
            DrawParams::color(TextColor::WHITE),
        );
        draw_text_left(
            ctx,
            eng,
            &1,
            &self.pokemon_do,
            11.0,
            139.0,
            DrawParams::color(TextColor::WHITE),
        );

        for (index, string) in self.buttons.iter().enumerate() {
            draw_text_left(
                ctx,
                eng,
                &0,
                string,
                138.0 + if index % 2 == 0 { 0.0 } else { 56.0 },
                123.0 + if index >> 1 == 0 { 0.0 } else { 16.0 },
                DrawParams::color(TextColor::BLACK),
            )
        }

        draw_cursor(
            ctx,
            eng,
            131.0 + if self.cursor % 2 == 0 { 0.0 } else { 56.0 },
            126.0 + if (self.cursor >> 1) == 0 { 0.0 } else { 16.0 },
            Default::default(),
        );
    }
}
