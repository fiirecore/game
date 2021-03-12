use firecore_util::Entity;
use crate::battle::transitions::BattleOpener;
use crate::battle::transitions::BattleTransition;
use crate::util::graphics::draw_rect;
use firecore_util::Timer;
use crate::util::{Reset, Completable};

pub struct TrainerBattleOpener {

    active: bool,
    finished: bool,

    start_timer: Timer,

    offset: f32,

    rect_size: f32,
    shrink_by: u8,

}

static RECT_SIZE: f32 = 80.0;
static OFFSET: f32 = 153.0 * 2.0;

impl TrainerBattleOpener {

    pub fn new() -> Self {

        Self {

            active: false,
            finished: false,

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
                self.finished = true;
            } else {
                self.offset -= 120.0 * delta;
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
            240,
            self.rect_size as u32,
        );
        draw_rect(
            [0.0, 0.0, 0.0, 1.0],
            0.0,
            160.0 - self.rect_size,
            240,
            self.rect_size as u32,
        );
    }

}

impl Reset for TrainerBattleOpener {

    fn reset(&mut self) {
        self.offset = OFFSET;
        self.rect_size = RECT_SIZE;
        self.shrink_by = 1;
        self.start_timer.reset();
        self.finished = false;
    }

}

impl Completable for TrainerBattleOpener {

    fn is_finished(&self) -> bool {
        return self.finished;
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
        self.active = true;
    }

    fn despawn(&mut self) {
        self.finished = false;
        self.active = false;
    }

    fn is_alive(&self) -> bool {
        return self.active;
    }
}