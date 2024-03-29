use crate::{WIDTH, HEIGHT};
use crate::utils::{Update, Render};
use crate::transitions::BattleScreenTransition;
use crate::transitions::BattleTransition;
use firecore_utils::{Reset, Completable};
use crate::utils::Load;
use firecore_utils::Entity;

use crate::utils::graphics::draw_rect;

pub struct VerticalCloseBattleScreenTransition {

    active: bool,
    finished: bool,

    offset: f32,
    speed: u16,

}

impl VerticalCloseBattleScreenTransition {

    pub fn new() -> Self {

        Self {

            active: false,
            finished: false,

            offset: 0.0,
            speed: 2 * 60,

        }

    }

    

}

impl BattleScreenTransition for VerticalCloseBattleScreenTransition {}
impl BattleTransition for VerticalCloseBattleScreenTransition {}

impl Reset for VerticalCloseBattleScreenTransition {

    fn reset(&mut self) {
        self.offset = 0.0;
        self.speed = 2 * 60;
    }  

}

impl Load for VerticalCloseBattleScreenTransition {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self) {
        
    } 

}

impl Completable for VerticalCloseBattleScreenTransition {

    fn finished(&self) -> bool {
        self.finished
    }   

}

impl Update for VerticalCloseBattleScreenTransition {

    fn update(&mut self, delta: f32) {
        if self.offset >= 24.0 {
            self.speed*=2;
        }
        self.offset += self.speed as f32 * delta;
        if self.offset >= BASE_HEIGHT as f32 / 2.0 {
            self.finished = true;
        }     
    }

}

impl Render for VerticalCloseBattleScreenTransition {

    fn render(&self) {
        draw_rect([0.0, 0.0, 0.0, 1.0], 0f32, 0f32, WIDTH, self.offset);
        draw_rect([0.0, 0.0, 0.0, 1.0], 0f32, HEIGHT - self.offset, WIDTH, self.offset.ceil());    
    }

}