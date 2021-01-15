use crate::entity::entity::Entity;
use crate::engine::game_context::GameContext;
use crate::entity::entity::Ticking;
use crate::battle::transitions::battle_transition_traits::BattleCloser;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::render_util::VIEW_HEIGHT;
use crate::util::render_util::VIEW_WIDTH;
use crate::util::render_util::draw_rect;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

pub struct BasicBattleCloser {

    alive: bool,
    finished: bool,

    alpha: u8,
    world_active: bool,

}

impl BasicBattleCloser {

    pub fn new() -> Self {

        Self {

            alive: false,
            finished: false,

            alpha: 0,
            world_active: false,

        }

    }

}

impl BattleCloser for BasicBattleCloser {
    
    fn world_active(&self) -> bool {
        self.world_active
    }

}

impl BattleTransition for BasicBattleCloser {

    fn reset(&mut self) {
        self.alpha = 0;
        self.world_active = false;
    }    

}

impl Loadable for BasicBattleCloser {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self, _context: &mut GameContext) {

    }

}

impl Completable for BasicBattleCloser {

    fn is_finished(&self) -> bool {
        return self.alpha == 0 && self.world_active;
    }

}

impl Ticking for BasicBattleCloser {

    fn update(&mut self, _context: &mut GameContext) {
        if self.world_active {
            self.alpha -= 1;
        } else {
            self.alpha += 1;
        }
        if self.alpha == 32 {
            self.world_active = true;
        }
    }

    fn render(&self, ctx: &mut piston_window::Context, g: &mut opengl_graphics::GlGraphics, _tr: &mut crate::engine::text::TextRenderer) {
        draw_rect(ctx, g, [0.0, 0.0, 0.0, (self.alpha as f32) / 32.0], 0, 0, VIEW_WIDTH, VIEW_HEIGHT);
    }

}

impl Entity for BasicBattleCloser {

    fn spawn(&mut self) {
        self.reset();
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.finished = false;
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }
}