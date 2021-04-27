use firecore_input::{pressed, Control};
use firecore_util::text::TextColor;
use crate::graphics::draw_cursor;
use crate::graphics::draw_text_left;
use crate::gui::panel::Panel;

pub struct BagSelectMenu {

    pub alive: bool,

    // background: Texture2D,
    background: Panel,

    pub world: [&'static str; Self::WORLD_LEN],
    pub battle: [&'static str; Self::BATTLE_LEN],

    cursor: usize,

    pub is_world: bool,

}

pub enum BagSelectAction {
    Use,
    Give,
    Toss,
}

impl BagSelectMenu {

    const WORLD_LEN: usize = 3;
    const BATTLE_LEN: usize = 1;

    pub fn new() -> Self {
        Self {
            alive: false,
            background: Panel::new(),
            // background: byte_texture(include_bytes!("../../../assets/gui/party/select.png")),
            world: [
                "Use",
                "Give",
                "Toss",
            ],
            battle: [
                "Use",
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
        if pressed(Control::Down) && self.cursor < if self.is_world { Self::WORLD_LEN } else { Self::BATTLE_LEN } {
            self.cursor += 1;
        }
        if pressed(Control::A) {
            if self.cursor == if self.is_world { Self::WORLD_LEN } else { Self::BATTLE_LEN } {
                self.alive = false;
                None
            } else if self.is_world {
                match self.cursor {
                    0 => Some(BagSelectAction::Use),
                    1 => Some(BagSelectAction::Give),
                    _ => None,
                }
            } else {
                match self.cursor {
                    0 => Some(BagSelectAction::Use),
                    _ => None,
                }
            }            
        } else {
            None
        }
    }

    pub fn render(&self) {
        if self.alive {
            let h = if self.is_world { 78.0 } else { 46.0 };
            self.background.render(146.0, util::HEIGHT - h, 94.0, h);
            let len = if self.is_world { self.world.len() } else { self.battle.len() };
            for (index, option) in if self.is_world { self.world.iter() } else { self.battle.iter() }.enumerate() {
                let index = len - index;
                draw_text_left(1, option, TextColor::Black, 161.0, 140.0 - (index << 4) as f32);
            }
            draw_text_left(1, "Cancel", TextColor::Black, 161.0, 140.0);
            draw_cursor(154.0, ((util::HEIGHT + 12.0) - h) + (self.cursor << 4) as f32);
        }
    }

}