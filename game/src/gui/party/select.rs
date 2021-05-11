use firecore_input::{pressed, Control};
use firecore_util::Reset;
use crate::text::TextColor;
use macroquad::prelude::Texture2D;

use crate::graphics::{byte_texture, draw, draw_text_left, draw_cursor};

pub struct PartySelectMenu {

    pub alive: bool,

    background: Texture2D,
    cursor: usize,

    world: [&'static str; 4],
    battle: [&'static str; 3],

    pub is_world: Option<bool>,

}

pub enum PartySelectAction {

    Select,
    Summary,
    // Item,
    // Cancel,

}

impl PartySelectMenu {

    pub fn new() -> Self {
        Self {
            alive: false,
            background: byte_texture(include_bytes!("../../../assets/gui/party/select.png")),
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
            is_world: None,
        }
    }

    pub fn input(&mut self) -> Option<PartySelectAction> {
        if let Some(is_world) = self.is_world {
            if pressed(Control::Up) && self.cursor > 0 {
                self.cursor -= 1;
            }
            if pressed(Control::Down) && self.cursor < if is_world { self.world.len() } else { self.battle.len() } {
                self.cursor += 1;
            }
            if pressed(Control::B) {
                self.alive = false;
            }
            if pressed(Control::A) {
                match is_world {
                    true => {
                        match self.cursor {
                            0 => Some(PartySelectAction::Summary),
                            1 => Some(PartySelectAction::Select),
                            2 => None,
                            3 => {
                                self.alive = false;
                                None
                            },
                            _ => unreachable!(),
                        }
                    },
                    false => {
                        match self.cursor {
                            0 => Some(PartySelectAction::Select),
                            1 => Some(PartySelectAction::Summary),
                            2 => {
                                self.alive = false;
                                None
                            },
                            _ => unreachable!(),
                        }
                    }
                }
            } else {
                None
            }
        } else {
            None
        }        
    }

    pub fn render(&self) {
        if self.alive {
            if let Some(is_world) = self.is_world {
                draw(self.background, 146.0, 83.0);
                draw_cursor(154.0, 94.0 + (self.cursor << 4) as f32);
                if is_world {
                    self.world.iter()
                } else {
                    self.battle.iter()
                }.enumerate().for_each(|(index, line)| draw_text_left(1, line, TextColor::Black, 161.0, 93.0 + (index << 4) as f32));
            }
        }        
    }

    pub fn toggle(&mut self) {
        self.alive = !self.alive;
        self.reset();
    }

}

impl Reset for PartySelectMenu {
    fn reset(&mut self) {
        self.cursor = 0;
    }
}