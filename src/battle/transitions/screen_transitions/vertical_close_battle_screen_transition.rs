use crate::BASE_HEIGHT;
use crate::BASE_WIDTH;
use crate::util::{Update, Render};
use crate::battle::transitions::battle_transition_traits::BattleScreenTransition;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::{Reset, Completable};
use crate::util::Load;



use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;

use crate::util::render::draw_rect;

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

    fn is_finished(&self) -> bool {
        return self.finished;
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

    fn render(&self, _tr: &TextRenderer) {
        draw_rect([0.0, 0.0, 0.0, 1.0], 0f32, 0f32, BASE_WIDTH, self.offset as u32);
        draw_rect([0.0, 0.0, 0.0, 1.0], 0f32, BASE_HEIGHT as f32 - self.offset, BASE_WIDTH, self.offset.ceil() as u32);    
    }

}

impl Entity for VerticalCloseBattleScreenTransition {

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