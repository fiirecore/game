use game::{
    util::{Reset, Completable, Timer, WIDTH},
    graphics::byte_texture,
    macroquad::prelude::{Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect, draw_rectangle, BLACK},
};

use crate::ui::transitions::BattleOpener;
use super::introductions::Introductions;

pub mod trainer;
pub mod wild;

pub enum Openers {
    Wild,
    Trainer,
}

impl Default for Openers {
    fn default() -> Self {
        Self::Wild
    }
}

impl Openers {

    pub fn intro(&self) -> Introductions {
        match self {
            Openers::Wild => Introductions::Basic,
            Openers::Trainer => Introductions::Trainer,
        }
    }

}

static mut PLAYER: Option<Texture2D> = None;

pub fn player_texture() -> Texture2D {
    unsafe { *PLAYER.get_or_insert(byte_texture(include_bytes!("../../../../assets/player.png"))) }
}

pub struct DefaultBattleOpener {

    start_timer: Timer,

    offset: f32,

    rect_size: f32,
    shrink_by: f32,

    player: Texture2D,

}


impl DefaultBattleOpener {
    const RECT_SIZE: f32 = 80.0;
    const SHRINK_BY_DEF: f32 = 1.0;
    const SHRINK_BY_FAST: f32 = 4.0;
    const OFFSET: f32 = 153.0 * 2.0;
}

impl Default for DefaultBattleOpener {
    fn default() -> Self {
        Self {
            start_timer: Timer::new(true, 0.5),
            rect_size: Self::RECT_SIZE,
            shrink_by: Self::SHRINK_BY_DEF,
            offset: Self::OFFSET,
            player: player_texture(),
        }
    }
}

impl BattleOpener for DefaultBattleOpener {

    fn spawn(&mut self, _battle: &crate::Battle) {}

    fn update(&mut self, delta: f32) {
        if self.start_timer.is_finished() {
            if self.offset > 0.0 {
                self.offset -= 120.0 * delta;
                if self.offset < 0.0 {
                    self.offset = 0.0;
                }
                
            }
            if self.rect_size > 0.0 {
                if self.rect_size > 0.0 {
                    self.rect_size -= self.shrink_by * 60.0 * delta;
                    if self.rect_size < 0.0 {
                        self.rect_size = 0.0;
                    }
                } else {
                    self.rect_size = 0.0;
                }
                if self.rect_size <= 58.0 && self.shrink_by != Self::SHRINK_BY_FAST {
                    self.shrink_by = Self::SHRINK_BY_FAST;
                }
            }
        } else {
            self.start_timer.update(delta);
        }
    }

    fn render_below_panel(&self, _battle: &crate::Battle) {
        draw_texture_ex(self.player, 41.0 + self.offset, 49.0, WHITE, DrawTextureParams {
            source: Some(
                Rect::new(
                    0.0,
                    0.0, 
                    64.0, 
                    64.0
                )
            ),
            ..Default::default()
        });
    }

    fn render(&self) {
        draw_rectangle(
            0.0,
            0.0,
            WIDTH,
            self.rect_size,
            BLACK,
        );
        draw_rectangle(
            0.0,
            160.0 - self.rect_size,
            WIDTH,
            self.rect_size,
            BLACK,
        );
    }

    fn offset(&self) -> f32 {
        self.offset
    }

}

impl Reset for DefaultBattleOpener {

    fn reset(&mut self) {
        self.offset = Self::OFFSET;
        self.rect_size = Self::RECT_SIZE;
        self.shrink_by = Self::SHRINK_BY_DEF;
        self.start_timer.hard_reset();
    }

}

impl Completable for DefaultBattleOpener {
    fn is_finished(&self) -> bool {
        self.offset <= 0.0
    }
}