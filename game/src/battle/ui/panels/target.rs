use crate::{
    input::{pressed, Control},
    gui::Panel,
    text::TextColor,
    graphics::{draw_text_left, draw_cursor},
};

use crate::battle::pokemon::ActivePokemonArray;

pub struct TargetPanel {

    // pub cols: usize,

    panel: Panel,

    pub names: Vec<Option<String>>,
    pub cursor: usize,

}

impl TargetPanel {

    pub fn new() -> Self {
        Self {
            panel: Panel::new(),
            names: Vec::with_capacity(4),
            cursor: 0,
        }
    }

    pub fn update_names(&mut self, targets: &ActivePokemonArray) {
        self.names.clear();
        for active in targets.iter() {
            self.names.push(active.pokemon.as_ref().map(|instance| instance.name().to_string()));
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
        self.panel.render(0.0, 113.0, 160.0, 47.0);
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
            draw_text_left(0, name.as_ref().map(|name| name.as_str()).unwrap_or("None"), TextColor::Black, 16.0 + x_offset, 121.0 + y_offset);
            if index == self.cursor {
                draw_cursor(10.0 + x_offset, 123.0 + y_offset);
            }
        }
    }

}