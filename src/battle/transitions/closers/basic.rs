use firecore_util::Entity;
use crate::battle::transitions::BattleCloser;
use crate::battle::transitions::BattleTransition;
use crate::util::graphics::draw_rect;
use firecore_util::{Reset, Completable};

#[derive(Default)]
pub struct BasicBattleCloser {

    alive: bool,
    finished: bool,

    alpha: f32,
    world_active: bool,

}

impl BattleTransition for BasicBattleCloser {
    
    fn on_start(&mut self) {
        
    }

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

    fn render(&self) {
        draw_rect([0.0, 0.0, 0.0, (self.alpha as f32) / 32.0], 0.0, 0.0, crate::WIDTH, crate::HEIGHT);
    }

}

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

impl Completable for BasicBattleCloser {

    fn is_finished(&self) -> bool {
        return self.alpha <= 0.0 && self.world_active;
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