use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::game::battle::transitions::battle_transition_traits::BattleCloser;
use crate::game::battle::transitions::battle_transition_traits::BattleTransitionManager;
use crate::game::battle::transitions::closers::basic_battle_closer::BasicBattleCloser;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

pub struct BattleCloserManager {
    
    alive: bool,

    pub closers: Vec<Box<dyn BattleCloser>>,
    pub current_closer_id: usize,

}

impl BattleCloserManager {

    pub fn new() -> Self {

        Self {

            alive: false,

            closers: Vec::new(),

            current_closer_id: 0,

        }

    }

    pub fn load_closers(&mut self) {
        self.closers.push(Box::new(BasicBattleCloser::new()));
    }

}

impl Ticking for BattleCloserManager {

    fn update(&mut self, context: &mut GameContext) {
        if self.is_alive() {
            self.closers[self.current_closer_id].update(context);
        }
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.closers[self.current_closer_id].render(ctx, g, tr);
    }

}

impl BattleTransitionManager for BattleCloserManager {

}

impl Loadable for BattleCloserManager {

    fn load(&mut self) {
        self.closers[self.current_closer_id].load();
    }

    fn on_start(&mut self, context: &mut GameContext) {
        self.closers[self.current_closer_id].on_start(context);
    }

}

impl Completable for BattleCloserManager {

    fn is_finished(&self) -> bool {
        return self.closers[self.current_closer_id].is_finished();
    }

}

impl Entity for BattleCloserManager {

    fn spawn(&mut self) {
        self.alive = true;
        self.closers[self.current_closer_id].spawn();
    }    

    fn despawn(&mut self) {
        self.alive = false;
        self.closers[self.current_closer_id].despawn();
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }

}