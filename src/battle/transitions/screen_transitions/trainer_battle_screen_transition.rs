use crate::BASE_WIDTH;
use crate::util::{Update, Render};
use crate::battle::transitions::battle_transition_traits::BattleScreenTransition;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::{Reset, Completable};
use crate::util::Load;
use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;
use crate::util::render::draw_rect;

pub struct TrainerBattleScreenTransition {

    pub active: bool,
    pub finished: bool,

    rect_width: f32,

}

impl TrainerBattleScreenTransition {

    pub fn new() -> Self {

        Self {
            active: false,
            finished: false,
            rect_width: 0.0,
        }

    }

    

}

impl BattleScreenTransition for TrainerBattleScreenTransition {}
impl BattleTransition for TrainerBattleScreenTransition {}

impl Reset for TrainerBattleScreenTransition {

    fn reset(&mut self) {
        self.rect_width = 0.0;
        self.finished = false;
    }

}

impl Load for TrainerBattleScreenTransition {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self) {
        
    } 

}

impl Completable for TrainerBattleScreenTransition {

    fn is_finished(&self) -> bool {
        return self.finished;
    } 

}

impl Update for TrainerBattleScreenTransition {

    fn update(&mut self, delta: f32) {
        self.rect_width += 240.0 * delta;
        if self.rect_width >= BASE_WIDTH as f32 {
            self.finished = true;
        }
    }

}

impl Render for TrainerBattleScreenTransition {

    fn render(&self, _tr: &TextRenderer) {
        draw_rect([0.0, 0.0, 0.0, 1.0], -(BASE_WIDTH as f32) - self.rect_width, 0.0, BASE_WIDTH, 32);
        draw_rect([0.0, 0.0, 0.0, 1.0], BASE_WIDTH as f32 - self.rect_width, 32.0, BASE_WIDTH, 32);
        draw_rect([0.0, 0.0, 0.0, 1.0], -(BASE_WIDTH as f32) - self.rect_width, 64.0, BASE_WIDTH, 32);
        draw_rect([0.0, 0.0, 0.0, 1.0], BASE_WIDTH as f32 - self.rect_width, 96.0, BASE_WIDTH, 32);
        draw_rect([0.0, 0.0, 0.0, 1.0], -(BASE_WIDTH as f32) - self.rect_width, 128.0, BASE_WIDTH, 32);      
    }

}

impl Entity for TrainerBattleScreenTransition {

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