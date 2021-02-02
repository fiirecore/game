use crate::entity::Entity;
use crate::util::{Update, Render};
use crate::battle::transitions::battle_transition_traits::BattleOpener;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::render::draw_rect;
use crate::util::timer::Timer;
use crate::util::{Reset, Completable};
use crate::util::Load;

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

impl Reset for TrainerBattleOpener {

    fn reset(&mut self) {
        self.offset = OFFSET;
        self.rect_size = RECT_SIZE;
        self.shrink_by = 1;
    }

}

impl Load for TrainerBattleOpener {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self) {
        self.start_timer.spawn();
    }

}

impl Completable for TrainerBattleOpener {

    fn is_finished(&self) -> bool {
        return self.finished;
    }

}

impl Update for TrainerBattleOpener {

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

}

impl Render for TrainerBattleOpener {

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

impl BattleTransition for TrainerBattleOpener {}