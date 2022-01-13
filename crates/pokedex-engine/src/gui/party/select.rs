use engine::{
    graphics::{draw_cursor, draw_text_left, DrawParams, Texture},
    controls::{pressed, Control},
    text::TextColor,
    Context,
};
use firecore_engine::EngineContext;

use crate::data::PokedexClientData;

pub struct PartySelectMenu {
    pub alive: bool,

    background: Texture,
    cursor: usize,

    world: &'static [&'static str; 4],
    battle: &'static [&'static str; 3],

    pub is_world: Option<bool>,
}

pub enum PartySelectAction {
    Select,
    Summary,
    // Item,
    // Cancel,
}

impl PartySelectMenu {
    pub fn new(ctx: &PokedexClientData) -> Self {
        Self {
            alive: Default::default(),
            background: ctx.party.select.clone(),
            cursor: Default::default(),
            world: &["Summary", "Switch", "Item", "Cancel"],
            battle: &["Shift", "Summary", "Cancel"],
            is_world: Default::default(),
        }
    }

    pub fn input(&mut self, ctx: &Context, eng: &EngineContext) -> Option<PartySelectAction> {
        if let Some(is_world) = self.is_world {
            let cursor = self.cursor;
            if pressed(ctx, eng, Control::Up) && cursor > 0 {
                self.cursor -= 1;
            }
            if pressed(ctx, eng, Control::Down)
                && cursor
                    < if is_world {
                        self.world.len()
                    } else {
                        self.battle.len()
                    }
            {
                self.cursor += 1;
            }
            if pressed(ctx, eng, Control::B) {
                self.alive = false;
            }
            if pressed(ctx, eng, Control::A) {
                let cursor = self.cursor;
                match is_world {
                    true => match cursor {
                        0 => Some(PartySelectAction::Summary),
                        1 => Some(PartySelectAction::Select),
                        2 => None,
                        3 => {
                            self.alive = false;
                            None
                        }
                        _ => unreachable!(),
                    },
                    false => match cursor {
                        0 => Some(PartySelectAction::Select),
                        1 => Some(PartySelectAction::Summary),
                        2 => {
                            self.alive = false;
                            None
                        }
                        _ => unreachable!(),
                    },
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        if self.alive {
            if let Some(is_world) = self.is_world {
                self.background.draw(ctx, 146.0, 83.0, Default::default());
                draw_cursor(
                    ctx,
                    eng,
                    154.0,
                    94.0 + (self.cursor << 4) as f32,
                    Default::default(),
                );
                if is_world {
                    self.world.iter()
                } else {
                    self.battle.iter()
                }
                .enumerate()
                .for_each(|(index, line)| {
                    draw_text_left(
                        ctx,
                        eng,
                        &1,
                        line,
                        161.0,
                        93.0 + (index << 4) as f32,
                        DrawParams::color(TextColor::BLACK),
                    )
                });
            }
        }
    }

    pub fn toggle(&mut self) {
        self.alive = !self.alive;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
    }
}
