use firecore_util::Entity;
use crate::battle::transitions::BattleOpener;
use crate::battle::transitions::BattleTransition;
use crate::util::graphics::draw_rect;
use firecore_util::Timer;
use firecore_util::{Reset, Completable};

pub struct TrainerBattleOpener {

    alive: bool,

    start_timer: Timer,

    offset: f32,

    rect_size: f32,
    shrink_by: u8,

}

const RECT_SIZE: f32 = 80.0;
const OFFSET: f32 = 153.0 * 2.0;

impl TrainerBattleOpener {

    pub fn new() -> Self {

        Self {

            alive: false,

            start_timer: Timer::new(0.5),

            rect_size: RECT_SIZE,
            shrink_by: 1,
            offset: OFFSET,
        }

    }

}

impl BattleTransition for TrainerBattleOpener {

    fn on_start(&mut self) {
        self.start_timer.spawn();
    }

    fn update(&mut self, delta: f32) {
        if self.start_timer.is_finished() {
            if self.offset - delta <= 0.0 {
                self.offset = 0.0;
            } else {
                self.offset -= 120.0 * delta
                //  * if macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::Space) {
				// 	8.0
				// } else {
				// 	1.0
				// }
                ;
            }
            if self.rect_size > 0.0 {
                if self.rect_size as isize - self.shrink_by as isize > 0 {
                    self.rect_size -= self.shrink_by as f32 * 60.0 * delta;
                } else {
                    self.rect_size = 0.0;
                }
                if self.rect_size <= 58.0 && self.shrink_by != 4 {
                    self.shrink_by = 4;
                }
            }
        } else {
            self.start_timer.update(delta);
        }
    }

    fn render(&self) {
        draw_rect(
            [0.0, 0.0, 0.0, 1.0],
            0.0,
            0.0,
            240.0,
            self.rect_size,
        );
        draw_rect(
            [0.0, 0.0, 0.0, 1.0],
            0.0,
            160.0 - self.rect_size,
            240.0,
            self.rect_size,
        );
    }

}

impl Reset for TrainerBattleOpener {

    fn reset(&mut self) {
        self.offset = OFFSET;
        self.rect_size = RECT_SIZE;
        self.shrink_by = 1;
        self.start_timer.reset();
    }

}

impl Completable for TrainerBattleOpener {

    fn is_finished(&self) -> bool {
        return self.offset <= 0.0;
    }

}

impl BattleOpener for TrainerBattleOpener {

    fn offset(&self) -> f32 {
        return self.offset;
    }

}

impl Entity for TrainerBattleOpener {

    fn spawn(&mut self) {
        self.reset();
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.start_timer.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}