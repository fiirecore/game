use crate::engine::game_context::GameContext;
use crate::entity::entity::Ticking;
use crate::battle::transitions::battle_transition_traits::BattleScreenTransition;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::render_util::{VIEW_WIDTH, VIEW_HEIGHT};
use crate::util::traits::Completable;
use crate::util::traits::Loadable;
use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;

use crate::util::render_util::draw_rect;

pub struct VerticalCloseBattleScreenTransition {

    active: bool,
    finished: bool,

    offset: u8,
    speed: u8,

}

impl VerticalCloseBattleScreenTransition {

    pub fn new() -> Self {

        Self {

            active: false,
            finished: false,

            offset: 0,
            speed: 2,

        }

    }

    

}

impl BattleScreenTransition for VerticalCloseBattleScreenTransition {
    
}

impl BattleTransition for VerticalCloseBattleScreenTransition {

    fn reset(&mut self) {
        self.offset = 0;
        self.speed = 2;
    }  

}

impl Loadable for VerticalCloseBattleScreenTransition {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self, _context: &mut GameContext) {
        
    } 

}

impl Completable for VerticalCloseBattleScreenTransition {

    fn is_finished(&self) -> bool {
        return self.finished;
    }   

}

impl Ticking for VerticalCloseBattleScreenTransition {

    fn update(&mut self, _context: &mut GameContext) {
        if self.offset == 24 {
            self.speed*=2;
        }
        self.offset += self.speed;
        if self.offset >= VIEW_HEIGHT as u8 / 2 {
            self.finished = true;
        }     
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
        draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], 0, 0, VIEW_WIDTH, self.offset as usize);
        draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], 0, VIEW_HEIGHT as isize - self.offset as isize, VIEW_WIDTH, self.offset as usize);    
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