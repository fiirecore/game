use game::{
    util::{Entity, Reset},
    pokedex::pokemon::instance::PokemonInstance,
    input::{pressed, Control},
    text::TextColor,
    macroquad::prelude::{Vec2, Texture2D},
    graphics::{byte_texture, draw, draw_text_left, draw_cursor},
    deps::smallvec::SmallVec,
};

pub struct MovePanel {

    alive: bool,
    panel: Vec2,
    background: Texture2D,
    pub cursor: usize,
    pub move_names: SmallVec<[String; 4]>,

}

impl MovePanel {

    pub fn new(panel: Vec2) -> Self {
        Self {
            alive: false,
            panel,
            background: byte_texture(include_bytes!("../../../assets/gui/move_panel.png")),
            cursor: 0,
            move_names: SmallVec::new(),
        }
    }    

    pub fn update_names(&mut self, instance: &PokemonInstance) {
        self.move_names.clear();
        for (index, move_instance) in instance.moves.iter().enumerate() {
            if index == 4 {
                break;
            }
            self.move_names.push(move_instance.pokemon_move.name.to_ascii_uppercase());
        }
    }

    pub fn input(&mut self) -> bool {

        let mut update_cursor = false;

        if pressed(Control::Up) {
            if self.cursor >= 2 {
                self.cursor -= 2;
                update_cursor = true;
            } 
        } else if pressed(Control::Down) {
            if self.cursor <= 2 {
                self.cursor += 2;
                update_cursor = true;
            } 
        } else if pressed(Control::Left) {
            if self.cursor > 0 {
                self.cursor -= 1;
                update_cursor = true;
            }
        } else if pressed(Control::Right) {
            if self.cursor < 3 {
                self.cursor += 1;
                update_cursor = true;
            }
        }
        
        if update_cursor {
            if self.cursor >= self.move_names.len() {
                self.cursor = self.move_names.len() - 1;
            }
        }

        update_cursor
    }

    pub fn render(&self) {
        draw(self.background, self.panel.x, self.panel.y);
        for (index, string) in self.move_names.iter().enumerate() {
            let x_offset = if index % 2 == 1 {
                72.0
            } else {
                0.0
            };
            let y_offset = if index / 2 == 1 {
                17.0
            } else {
                0.0
            };
            draw_text_left(0, string, TextColor::Black, self.panel.x + 16.0 + x_offset, self.panel.y + 8.0 + y_offset);
            if index == self.cursor {
                draw_cursor(self.panel.x + 10.0 + x_offset, self.panel.y + 10.0 + y_offset);
            }
        }
    }

}

impl Entity for MovePanel {

    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(& self) -> bool {
        self.alive
    }

}

impl Reset for MovePanel {
    fn reset(&mut self) {
        self.cursor = 0;
    }    
}