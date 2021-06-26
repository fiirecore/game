use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};

use crate::input::{pressed, Control};
use atomic::Atomic;
use crate::text::TextColor;
use crate::tetra::{Context, graphics::Texture};

use crate::graphics::{byte_texture, draw_text_left, draw_cursor};

pub struct PartySelectMenu {

    pub alive: AtomicBool,

    background: Texture,
    cursor: AtomicUsize,

    world: [&'static str; 4],
    battle: [&'static str; 3],

    pub is_world: Atomic<Option<bool>>,

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
            alive: AtomicBool::new(false),
            background: byte_texture(ctx, include_bytes!("../../../assets/gui/party/select.png")),
            cursor: AtomicUsize::new(0),
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
            is_world: Atomic::new(None),
        }
    }

    pub fn input(&self, ctx: &Context) -> Option<PartySelectAction> {
        if let Some(is_world) = self.is_world.load(Relaxed) {
            let cursor = self.cursor.load(Relaxed);
            if pressed(ctx, Control::Up) && cursor > 0 {
                self.cursor.store(cursor - 1, Relaxed);
            }
            if pressed(ctx, Control::Down) && cursor < if is_world { self.world.len() } else { self.battle.len() } {
                self.cursor.store(cursor + 1, Relaxed);
            }
            if pressed(ctx, Control::B) {
                self.alive.store(false, Relaxed);
            }
            if pressed(ctx, Control::A) {
                let cursor = self.cursor.load(Relaxed);
                match is_world {
                    true => {
                        match cursor {
                            0 => Some(PartySelectAction::Summary),
                            1 => Some(PartySelectAction::Select),
                            2 => None,
                            3 => {
                                self.alive.store(false, Relaxed);
                                None
                            },
                            _ => unreachable!(),
                        }
                    },
                    false => {
                        match cursor {
                            0 => Some(PartySelectAction::Select),
                            1 => Some(PartySelectAction::Summary),
                            2 => {
                                self.alive.store(false, Relaxed);
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
        if self.alive.load(Relaxed) {
            if let Some(is_world) = self.is_world.load(Relaxed) {
                self.background.draw(ctx, crate::graphics::position(146.0, 83.0));
                draw_cursor(ctx, 154.0, 94.0 + (self.cursor.load(Relaxed) << 4) as f32);
                if is_world {
                    self.world.iter()
                } else {
                    self.battle.iter()
                }.enumerate().for_each(|(index, line)| draw_text_left(ctx, &1, line, &TextColor::Black, 161.0, 93.0 + (index << 4) as f32));
            }
        }        
    }

    pub fn toggle(&self) {
        self.alive.store(!self.alive.load(Relaxed), Relaxed);
        self.reset();
    }

    pub fn reset(&self) {
        self.cursor.store(0, Relaxed);
    }

}
