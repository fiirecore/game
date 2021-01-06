use crate::engine::game_context::GameContext;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::game::battle::transitions::battle_transition_traits::BattleOpener;
use crate::game::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::render_util::draw_rect;
use crate::util::timer::Timer;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

pub struct TrainerBattleOpener {

    active: bool,
    finished: bool,

    start_timer: Timer,

    offset: u16,

    rect_size: u8,
    shrink_by: u8,

}

static RECT_SIZE: u8 = 80;
static OFFSET: u16 = 153 * 2;

impl TrainerBattleOpener {

    pub fn new() -> Self {

        Self {

            active: false,
            finished: false,

            start_timer: Timer::new(30),

            rect_size: RECT_SIZE,
            shrink_by: 1,
            offset: OFFSET,
        }

    }

}

impl BattleTransition for TrainerBattleOpener {

    fn reset(&mut self) {
        self.offset = OFFSET;
        self.rect_size = RECT_SIZE;
        self.shrink_by = 1;
    }

}

impl Loadable for TrainerBattleOpener {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self, _context: &mut GameContext) {
        self.start_timer.spawn();
    }

}

impl Completable for TrainerBattleOpener {

    fn is_finished(&self) -> bool {
        return self.finished;
    }

}

impl Ticking for TrainerBattleOpener {

    fn update(&mut self, _context: &mut GameContext) {
        if self.start_timer.is_finished() {
            self.offset -= 2;
            if self.rect_size > 0 {
                if self.rect_size as isize - self.shrink_by as isize > 0 {
                    self.rect_size -= self.shrink_by;
                } else {
                    self.rect_size = 0;
                }
                if self.rect_size == 58 {
                    self.shrink_by = 4;
                }
            }
            if self.offset <= 0 {
                self.finished = true;
            }
        } else {
            self.start_timer.update();
        }
    }

    fn render(&self, ctx: &mut piston_window::Context, g: &mut opengl_graphics::GlGraphics, _tr: &mut crate::engine::text::TextRenderer) {
        draw_rect(
            ctx,
            g,
            [0.0, 0.0, 0.0, 1.0].into(),
            0,
            0,
            240,
            self.rect_size as usize,
        );
        draw_rect(
            ctx,
            g,
            [0.0, 0.0, 0.0, 1.0].into(),
            0,
            160 - self.rect_size as isize,
            240,
            self.rect_size as usize,
        );
    }

}

impl BattleOpener for TrainerBattleOpener {

    fn offset(&self) -> u16 {
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