use crate::input::{pressed, Control};
use firecore_util::Reset;
use crate::text::TextColor;
use crate::tetra::{Context, graphics::Texture};

use crate::graphics::{byte_texture, draw_text_left, draw_cursor};

pub struct PartySelectMenu {

    pub alive: bool,

    background: Texture,
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

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            alive: false,
            background: byte_texture(ctx, include_bytes!("../../../assets/gui/party/select.png")),
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

    pub fn input(&mut self, ctx: &Context) -> Option<PartySelectAction> {
        if let Some(is_world) = self.is_world {
            if pressed(ctx, Control::Up) && self.cursor > 0 {
                self.cursor -= 1;
            }
            if pressed(ctx, Control::Down) && self.cursor < if is_world { self.world.len() } else { self.battle.len() } {
                self.cursor += 1;
            }
            if pressed(ctx, Control::B) {
                self.alive = false;
            }
            if pressed(ctx, Control::A) {
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

    pub fn draw(&self, ctx: &mut Context) {
        if self.alive {
            if let Some(is_world) = self.is_world {
                self.background.draw(ctx, crate::graphics::position(146.0, 83.0));
                draw_cursor(ctx, 154.0, 94.0 + (self.cursor << 4) as f32);
                if is_world {
                    self.world.iter()
                } else {
                    self.battle.iter()
                }.enumerate().for_each(|(index, line)| draw_text_left(ctx, &1, line, TextColor::Black, 161.0, 93.0 + (index << 4) as f32));
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