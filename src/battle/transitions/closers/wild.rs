use firecore_util::Entity;
use crate::battle::transitions::{BattleTransition, BattleTransitionGui, BattleCloser};
use crate::util::graphics::draw_rect;
use firecore_util::{Reset, Completable};

#[derive(Default)]
pub struct WildBattleCloser {

    alive: bool,

    alpha: f32,
    world_active: bool,

}

impl BattleTransitionGui for WildBattleCloser {}

impl BattleTransition for WildBattleCloser {
    
    fn on_start(&mut self) {
        
    }

    fn update(&mut self, delta: f32) {
        if self.world_active {
            self.alpha -= 45.0 * delta;
        } else {
            self.alpha += 45.0 * delta;
        }
        if self.alpha >= 32.0 {
            self.world_active = true;
        }
    }

    fn render(&self) {
        draw_rect([0.0, 0.0, 0.0, (self.alpha as f32) / 32.0], 0.0, 0.0, crate::WIDTH, crate::HEIGHT);
    }

}

impl BattleCloser for WildBattleCloser {

    fn world_active(&self) -> bool {
        self.world_active
    }
}

impl Reset for WildBattleCloser {

    fn reset(&mut self) {
        self.alpha = 0.0;
        self.world_active = false;
    }    

}

impl Completable for WildBattleCloser {

    fn is_finished(&self) -> bool {
        return self.alpha <= 0.0 && self.world_active;
    }

}

impl Entity for WildBattleCloser {

    fn spawn(&mut self) {
        self.reset();
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.world_active = false;
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }
}