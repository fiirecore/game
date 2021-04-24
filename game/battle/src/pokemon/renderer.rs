use game::{
    util::{Reset, Completable},
    macroquad::prelude::{Texture2D, Vec2, draw_texture_ex, DrawTextureParams, Rect, WHITE},
};

use std::collections::VecDeque;
use game::pokedex::moves::battle_script::BattleActionActions;

const MISSING: f32 = 0.0;
const SIZE: f32 = 64.0;




#[derive(Default)]
pub struct ActivePokemonRenderer {

    pub pos: Vec2,
    missing: f32,
    reverse: bool,

    flicker: Flicker,

    leaving: bool,
    offset: Vec2,
    pub move_actions: Option<VecDeque<BattleActionActions>>,

}

#[derive(Default)]
struct Flicker {
    remaining: u8,
    accumulator: f32,
}

impl Flicker {
    const LENGTH: f32 = 0.20;
    const TIMES: u8 = 4;
}

impl ActivePokemonRenderer {

    pub fn new(pos: Vec2, reverse: bool) -> Self {
        Self {
            pos,
            missing: MISSING,
            reverse,
            flicker: Flicker::default(),
            leaving: false,
            offset: Vec2::default(),
            move_actions: None,
        }
    }

    pub fn update_faint(&mut self, delta: f32) {
        self.missing += delta * 128.0;
    }

    pub fn update_other(&mut self, delta: f32) {
        self.update_flicker(delta);
        self.update_script(delta);
    }

    pub fn update_flicker(&mut self, delta: f32) {
        if self.flicker.remaining != 0 {
            self.flicker.accumulator += delta;
            if self.flicker.accumulator > Flicker::LENGTH {
                self.flicker.accumulator -= Flicker::LENGTH;
                self.flicker.remaining -= 1;
            }
            if self.flicker.remaining == 0 {
                self.flicker.accumulator = 0.0;
            }
        }
    }

    pub fn update_script(&mut self, delta: f32) {
        if let Some(actions) = self.move_actions.as_mut() {
            let mut pop = false;
            for action in &*actions {
                match action {
                    BattleActionActions::MoveAndReturn(pos) => {
                        if !self.leaving {
                            self.offset.x += delta * 120.0;
                            if self.offset.x > *pos {
                                self.offset.x = *pos;
                                self.leaving = true;
                            }
                        } else {
                            self.offset.x -= delta * 120.0;
                            if self.offset.x < 0.0 {
                                self.offset.x = 0.0;
                                self.leaving = false;
                                pop = true;
                            }
                        }
                    }
                }
            }
            if pop {
                actions.pop_front();
            }
            if actions.is_empty() {
                self.move_actions = None;
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.move_actions.as_ref().map(|actions| actions.is_empty()).unwrap_or_default()
    }

    pub fn flicker(&mut self) {
        self.flicker.remaining = Flicker::TIMES;
        self.flicker.accumulator = 0.0;
    }

    pub fn render(&self, texture: Texture2D, y_offset: f32) {
        if self.missing < SIZE && self.flicker.accumulator < Flicker::LENGTH / 2.0 {
            draw_texture_ex(
                texture,
                self.pos.x + if self.reverse { -self.offset.x } else { self.offset.x },
                self.pos.y - texture.height() + y_offset + self.missing,
                WHITE,
                DrawTextureParams {
                    source: if self.missing > 0.0 {
                        Some(
                            Rect::new(0.0, 0.0, texture.width(), SIZE - self.missing)
                        )
                    } else {
                        None
                    },
                    ..Default::default()
                }
            );
        }
    }

}

impl Reset for ActivePokemonRenderer {
    fn reset(&mut self) {
        self.missing = MISSING;
    }
}

impl Completable for ActivePokemonRenderer {
    fn is_finished(&self) -> bool {
        self.missing >= SIZE
    }
}