use firecore_input::{pressed, Control};
use firecore_util::text::TextColor;
use macroquad::prelude::Texture2D;

use crate::graphics::byte_texture;
use crate::graphics::draw;
use crate::graphics::draw_cursor;
use crate::graphics::draw_text_left;

pub struct BagSelectMenu {

    pub alive: bool,

    background: Texture2D,

    pub world: [&'static str; 4],
    pub battle: [&'static str; 2],

    cursor: usize,

    pub is_world: bool,

}

pub enum BagSelectAction {
    Use,
    Give,
    Toss,
}

impl BagSelectMenu {

    pub fn new() -> Self {
        Self {
            alive: false,
            background: byte_texture(include_bytes!("../../../assets/gui/party/select.png")),
            world: [
                "Use",
                "Give",
                "Toss",
                "Cancel",
            ],
            battle: [
                "Use",
                "Cancel",
            ],
            cursor: 0,
            is_world: true,
        }
    }

    pub fn spawn(&mut self) {
        self.alive = true;
        self.cursor = 0;
    }

    pub fn input(&mut self) -> Option<BagSelectAction> {
        if pressed(Control::B) {
            self.alive = false;
        }
        if pressed(Control::Up) && self.cursor > 0 {
            self.cursor -= 1;
        }
        if pressed(Control::Down) && self.cursor < 3 {
            self.cursor += 1;
        }
        if pressed(Control::A) {
            if self.is_world {
                match self.cursor {
                    0 => Some(BagSelectAction::Use),
                    1 => Some(BagSelectAction::Give),
                    2 => Some(BagSelectAction::Toss),
                    3 => {
                        self.alive = false;
                        None
                    },
                    _ => None,
                }
            } else {
                match self.cursor {
                    0 => Some(BagSelectAction::Use),
                    1 => {
                        self.alive = false;
                        None
                    },
                    _ => None,
                }
            }            
        } else {
            None
        }
    }

    pub fn render(&self) {
        if self.alive {
            draw(self.background, 146.0, 83.0);
            for (index, option) in if self.is_world { self.world.iter() } else { self.battle.iter() }.enumerate() {
                draw_text_left(1, option, TextColor::Black, 161.0, 94.0 + (index << 4) as f32);
            }
            draw_cursor(154.0, 94.0 + (self.cursor << 4) as f32);
        }
    }

}