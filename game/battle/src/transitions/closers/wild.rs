use game::{
    util::{Entity, Reset, Completable, WIDTH, HEIGHT},
    macroquad::prelude::{Color, draw_rectangle},
};

use crate::transitions::{BattleTransition, BattleTransitionGui, BattleCloser};

#[derive(Default)]
pub struct WildBattleCloser {

    alive: bool,

    color: Color,
    world_active: bool,

}

impl BattleTransitionGui for WildBattleCloser {}

impl BattleTransition for WildBattleCloser {
    
    fn on_start(&mut self) {
        
    }

    fn update(&mut self, delta: f32) {
        if self.world_active {
            self.color.a -= 4.5 * delta;
        } else {
            self.color.a += 4.5 * delta;
        }
        if self.color.a >= 1.0 {
            self.world_active = true;
        }
    }

    fn render(&self) {
        draw_rectangle(0.0, 0.0, WIDTH, HEIGHT, self.color);
    }

}

impl BattleCloser for WildBattleCloser {

    fn world_active(&self) -> bool {
        self.world_active
    }
}

impl Reset for WildBattleCloser {

    fn reset(&mut self) {
        self.color.a = 0.0;
        self.world_active = false;
    }    

}

impl Completable for WildBattleCloser {

    fn is_finished(&self) -> bool {
        self.color.a <= 0.0 && self.world_active
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
        self.alive
    }
}