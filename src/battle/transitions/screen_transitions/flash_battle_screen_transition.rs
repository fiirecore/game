use crate::util::{Update, Render};
use crate::battle::transitions::battle_transition_traits::BattleScreenTransition;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::{Reset, Completable};
use crate::util::Load;

use crate::entity::Entity;

use crate::util::render::draw_rect;

static FINAL_INDEX: u8 = 4;

pub struct FlashBattleScreenTransition {
    active: bool,
    finished: bool,
    screen: [f32; 4],
    waning: bool,
    index: u8,
    fade: f32,
    zoom: bool,
}

impl FlashBattleScreenTransition {
    pub fn new() -> Self {
        Self {
            active: false,
            finished: false,

            screen: [1.0, 1.0, 1.0, 0.0],
            waning: false,
            index: 0,
            fade: 1.0 / 8.0,
            zoom: false,
        }
    }
}

impl BattleScreenTransition for FlashBattleScreenTransition {}
impl BattleTransition for FlashBattleScreenTransition {}

impl Reset for FlashBattleScreenTransition {
    fn reset(&mut self) {
        self.screen = [1.0, 1.0, 1.0, 0.0];
        self.waning = false;
        self.index = 0;
        self.fade = 1.0 / 8.0;
        self.zoom = false;
    }
}

impl Load for FlashBattleScreenTransition {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self) {
        
    } 

}

impl Completable for FlashBattleScreenTransition {
    fn is_finished(&self) -> bool {
        return self.finished;
    }
}

impl Update for FlashBattleScreenTransition {
    
    fn update(&mut self, delta: f32) {
        if self.waning {
            self.screen[3] -= self.fade * 60.0 * delta;
        } else {
            self.screen[3] += self.fade * 60.0 * delta;
        }
        if self.screen[3] <= 0.0 {
            self.waning = false;
            self.index += 1;
        } else if self.screen[3] >= 1.0 {
            self.waning = true;
        }
        if self.index == FINAL_INDEX && self.screen[3] <= 0.0 {
            self.screen[0] = 0.0;
            self.screen[1] = 0.0;
            self.screen[2] = 0.0;
            self.fade = 1.0 / 16.0;
            self.zoom = true;
        }
        if self.index >= FINAL_INDEX && self.waning {
            self.finished = true;
        }
    }

}

impl Render for FlashBattleScreenTransition {

    fn render(&self) {
        draw_rect(
            self.screen,
            0.0,
            0.0,
            crate::BASE_WIDTH,
            crate::BASE_HEIGHT,
        );
    }
}

impl Entity for FlashBattleScreenTransition {
    fn spawn(&mut self) {
        self.reset();
        self.active = true;
        self.finished = false;
    }

    fn despawn(&mut self) {
        self.active = false;
        self.finished = false;
    }

    fn is_alive(&self) -> bool {
        self.active
    }
}
