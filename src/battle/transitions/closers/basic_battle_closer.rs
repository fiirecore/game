use crate::entity::Entity;
use crate::util::{Update, Render};
use crate::battle::transitions::battle_transition_traits::BattleCloser;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::render::draw_rect;
use crate::util::{Reset, Completable};
use crate::util::Load;


pub struct BasicBattleCloser {

    alive: bool,
    finished: bool,

    alpha: f32,
    world_active: bool,

}

impl BasicBattleCloser {

    pub fn new() -> Self {

        Self {

            alive: false,
            finished: false,

            alpha: 0.0,
            world_active: false,

        }

    }

}

impl BattleTransition for BasicBattleCloser {}
impl BattleCloser for BasicBattleCloser {
    
    fn world_active(&self) -> bool {
        self.world_active
    }

}

impl Reset for BasicBattleCloser {

    fn reset(&mut self) {
        self.alpha = 0.0;
        self.world_active = false;
    }    

}

impl Load for BasicBattleCloser {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self) {

    }

}

impl Completable for BasicBattleCloser {

    fn is_finished(&self) -> bool {
        return self.alpha <= 0.0 && self.world_active;
    }

}

impl Update for BasicBattleCloser {

    fn update(&mut self, delta: f32) {
        if self.world_active {
            self.alpha -= 60.0 * delta;
        } else {
            self.alpha += 60.0 * delta;
        }
        if self.alpha >= 32.0 {
            self.world_active = true;
        }
    }

}

impl Render for BasicBattleCloser {

    fn render(&self, _tr: &crate::util::text_renderer::TextRenderer) {
        draw_rect([0.0, 0.0, 0.0, (self.alpha as f32) / 32.0], 0.0, 0.0, crate::BASE_WIDTH, crate::BASE_HEIGHT);
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