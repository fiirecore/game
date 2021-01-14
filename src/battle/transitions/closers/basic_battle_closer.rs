use crate::entity::entity::Entity;
use crate::engine::game_context::GameContext;
use crate::entity::entity::Ticking;
use crate::battle::transitions::battle_transition_traits::BattleCloser;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

pub struct BasicBattleCloser {

    alive: bool,
    finished: bool,

}

impl BasicBattleCloser {

    pub fn new() -> Self {

        Self {

            alive: false,
            finished: false,

        }

    }

}

impl BattleCloser for BasicBattleCloser {
    
}

impl BattleTransition for BasicBattleCloser {

    fn reset(&mut self) {
        self.finished = true;
    }    

}

impl Loadable for BasicBattleCloser {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self, context: &mut GameContext) {

    }

}

impl Completable for BasicBattleCloser {

    fn is_finished(&self) -> bool {
        return self.finished;
    }

}

impl Ticking for BasicBattleCloser {

    fn update(&mut self, context: &mut GameContext) {
    }

    fn render(&self, ctx: &mut piston_window::Context, g: &mut opengl_graphics::GlGraphics, tr: &mut crate::engine::text::TextRenderer) {
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