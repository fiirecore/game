use crate::{
    util::Reset,
    pokedex::{
        pokemon::instance::PokemonInstance,
        moves::MoveRef
    },
    input::{pressed, Control},
    text::TextColor,
    gui::Panel,
    graphics::{draw_text_left, draw_cursor},
    tetra::{
        Context,
    },
    deps::vec::ArrayVec,
};

pub struct MovePanel {

    // origin: Vec2,
    panel: Panel,
    pub cursor: usize,
    pub names: ArrayVec<[(MoveRef, TextColor); 4]>,

}

impl MovePanel {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            panel: Panel::new(ctx),
            cursor: 0,
            names: ArrayVec::new(),
        }
    }    

    pub fn update_names(&mut self, instance: &PokemonInstance) {
        self.names = instance.moves.iter().map(|instance| {
            (instance.move_ref, if instance.empty() { TextColor::Red } else { TextColor::Black })
        }).collect();
    }

    pub fn input(&mut self, ctx: &Context) -> bool {
        if {
            if pressed(ctx, Control::Up) {
                if self.cursor >= 2 {
                    self.cursor -= 2;
                    true
                } else {
                    false
                }
            } else if pressed(ctx, Control::Down) {
                if self.cursor <= 2 {
                    self.cursor += 2;
                    true
                } else {
                    false
                }
            } else if pressed(ctx, Control::Left) {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    true
                } else {
                    false
                }
            } else if pressed(ctx, Control::Right) {
                if self.cursor < 3 {
                    self.cursor += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } {
            if self.cursor >= self.names.len() {
                self.cursor = self.names.len() - 1;
            }
            true
        } else {
            false
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.panel.draw(ctx, 0.0, 113.0, 160.0, 47.0);
        for (index, (pokemon_move, color)) in self.names.iter().enumerate() {
            let x_offset = if index % 2 == 1 {
                72.0
            } else {
                0.0
            };
            let y_offset = if index >> 1 == 1 {
                17.0
            } else {
                0.0
            };
            draw_text_left(ctx, &0, &pokemon_move.value().name, *color, 16.0 + x_offset, 121.0 + y_offset);
            if index == self.cursor {
                draw_cursor(ctx, 10.0 + x_offset, 123.0 + y_offset);
            }
        }
    }

}

impl Reset for MovePanel {
    fn reset(&mut self) {
        self.cursor = 0;
    }    
}