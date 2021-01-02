use crate::entity::entity::Entity;
use crate::game::battle::transitions::traits::battle_transition::BattleTransition;
use crate::engine::game_context::GameContext;

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

impl BattleTransition for BasicBattleCloser {

    fn reset(&mut self) {
        self.finished = true;
    }

    fn on_start(&mut self, context: &mut GameContext) {
    }

    fn update(&mut self, context: &mut GameContext) {
    }

    fn render(&self, ctx: &mut piston_window::Context, g: &mut opengl_graphics::GlGraphics, tr: &mut crate::engine::text::TextRenderer) {
    }

    fn is_finished(&self) -> bool {
        return self.finished;
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