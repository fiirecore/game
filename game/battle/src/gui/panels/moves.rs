use game::{
    util::Reset,
    pokedex::pokemon::instance::PokemonInstance,
    input::{pressed, Control},
    text::TextColor,
    gui::panel::Panel,
    macroquad::prelude::Vec2,
    graphics::{draw_text_left, draw_cursor},
    deps::vec::ArrayVec,
};

pub struct MovePanel {

    origin: Vec2,
    panel: Panel,
    pub cursor: usize,
    pub names: ArrayVec<[(String, TextColor); 4]>,

}

impl MovePanel {

    pub fn new(origin: Vec2) -> Self {
        Self {
            origin,
            panel: Panel::new(),
            cursor: 0,
            names: ArrayVec::new(),
        }
    }    

    pub fn update_names(&mut self, instance: &PokemonInstance) {
        self.names.clear();
        for (index, move_instance) in instance.moves.iter().enumerate() {
            if index == 4 {
                break;
            }
            if !self.names.get(index).map(|(name, _)| name.eq_ignore_ascii_case(&move_instance.pokemon_move.name)).unwrap_or_default() {
                self.names.push((move_instance.pokemon_move.name.to_ascii_uppercase(), if move_instance.pp == 0 { TextColor::Red } else { TextColor::Black }));
            }
            if let Some((_, color)) = self.names.get_mut(index) {
                *color = if move_instance.pp == 0 { TextColor::Red } else { TextColor::Black };
            }
        }
    }

    pub fn input(&mut self) -> bool {
        if {
            if pressed(Control::Up) {
                if self.cursor >= 2 {
                    self.cursor -= 2;
                    true
                } else {
                    false
                }
            } else if pressed(Control::Down) {
                if self.cursor <= 2 {
                    self.cursor += 2;
                    true
                } else {
                    false
                }
            } else if pressed(Control::Left) {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    true
                } else {
                    false
                }
            } else if pressed(Control::Right) {
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

    pub fn render(&self) {
        self.panel.render(self.origin.x, self.origin.y, 160.0, 47.0);
        for (index, (string, color)) in self.names.iter().enumerate() {
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
            draw_text_left(0, string, *color, self.origin.x + 16.0 + x_offset, self.origin.y + 8.0 + y_offset);
            if index == self.cursor {
                draw_cursor(self.origin.x + 10.0 + x_offset, self.origin.y + 10.0 + y_offset);
            }
        }
    }

}

impl Reset for MovePanel {
    fn reset(&mut self) {
        self.cursor = 0;
    }    
}