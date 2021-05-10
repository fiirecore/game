use game::{
    input::{pressed, Control},
    graphics::{draw, draw_text_left, draw_cursor},
    macroquad::prelude::Texture2D,
};

use crate::pokemon::ActivePokemon;

pub struct TargetPanel {

    // pub cols: usize,

    background: Texture2D,

    pub names: Vec<Option<String>>,
    pub cursor: usize,

}

impl TargetPanel {

    pub fn new() -> Self {
        Self {
            background: super::moves::move_panel_texture(),
            names: Vec::with_capacity(4),
            cursor: 0,
        }
    }

    pub fn update_names(&mut self, targets: &Box<[ActivePokemon]>) {
        self.names.clear();
        for active in targets.iter() {
            self.names.push(active.pokemon.as_ref().map(|instance| instance.name()));
        }
    }

    pub fn input(&mut self) {
        if pressed(Control::Up) && self.cursor >= 2 {
            self.cursor -= 2;
        } else if pressed(Control::Down) && self.cursor <= 2 {
            self.cursor += 2;
        } else if pressed(Control::Left) && self.cursor > 0 {
            self.cursor -= 1;
        } else if pressed(Control::Right) && self.cursor < 3 {
            self.cursor += 1;
        }
        if self.cursor >= self.names.len() {
            self.cursor = self.names.len() - 1;
        }
    }

    pub fn render(&self) {
        draw(self.background, 0.0, 113.0);
        for (index, name) in self.names.iter().enumerate() {
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
            draw_text_left(0, name.as_ref().map(|name| name.as_str()).unwrap_or("None"), game::text::TextColor::Black, 16.0 + x_offset, 121.0 + y_offset);
            if index == self.cursor {
                draw_cursor(10.0 + x_offset, 123.0 + y_offset);
            }
        }
    }

}