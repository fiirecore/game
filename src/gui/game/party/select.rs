use firecore_input::{pressed, Control};
use firecore_util::Reset;
use firecore_util::text::TextColor;
use macroquad::prelude::Texture2D;

use crate::util::graphics::{byte_texture, draw, draw_text_left, draw_cursor};

pub struct SelectMenu {

    pub alive: bool,

    background: Texture2D,
    cursor: usize,

    world: [&'static str; 4],
    battle: [&'static str; 3],

    pub is_world: bool,

}

pub enum SelectAction {

    Select,
    Summary,
    // Item,
    // Cancel,

}

impl SelectMenu {

    pub fn new() -> Self {
        Self {
            alive: false,
            background: byte_texture(include_bytes!("../../../../build/assets/gui/party/select.png")),
            cursor: 0,
            world: [
                "Summary",
                "Switch",
                "Item",
                "Cancel",
            ],
            battle: [
                "Shift",
                "Summary",
                "Cancel",
            ],
            is_world: true,
        }
    }

    pub fn input(&mut self) -> Option<SelectAction> {
        if pressed(Control::Up) && self.cursor > 0 {
            self.cursor -= 1;
        }
        if pressed(Control::Down) && self.cursor < if self.is_world { self.world.len() } else { self.battle.len() } {
            self.cursor += 1;
        }
        if pressed(Control::A) {
            match self.is_world {
                true => {
                    match self.cursor {
                        0 => Some(SelectAction::Summary),
                        1 => Some(SelectAction::Select),
                        2 => None,
                        3 => {
                            self.alive = false;
                            None
                        },
                        _ => None,
                    }
                },
                false => {
                    match self.cursor {
                        0 => Some(SelectAction::Select),
                        1 => Some(SelectAction::Summary),
                        2 => {
                            self.alive = false;
                            None
                        },
                        _ => None,
                    }
                }
            }
            
        } else {
            None
        }
    }

    pub fn render(&self) {
        if self.alive {
            draw(self.background, 146.0, 83.0);
            draw_cursor(154.0, 94.0 + (self.cursor << 4) as f32);
            if self.is_world {
                self.world.iter()
            } else {
                self.battle.iter()
            }.enumerate().for_each(|(index, line)| draw_text_left(1, line, TextColor::Black, 161.0, 93.0 + (index << 4) as f32));
        }        
    }

    pub fn toggle(&mut self) {
        self.alive = !self.alive;
        self.reset();
    }

}

impl Reset for SelectMenu {
    fn reset(&mut self) {
        self.cursor = 0;
    }
}