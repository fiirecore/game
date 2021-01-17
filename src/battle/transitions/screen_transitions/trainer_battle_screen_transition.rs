use crate::BASE_WIDTH;
use crate::util::context::GameContext;
use crate::entity::Ticking;
use crate::battle::transitions::battle_transition_traits::BattleScreenTransition;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;
use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;

use crate::util::render_util::draw_rect;

pub struct TrainerBattleScreenTransition {

    pub active: bool,
    pub finished: bool,

    rect0width: u8,

    //rect1width: u8,

    //rect2width: u8,

    //rect3width: u8,

    //rect4width: u8,

}

impl TrainerBattleScreenTransition {

    pub fn new() -> Self {

        Self {
            active: false,
            finished: false,
            rect0width: 0,
            //rect1width: 0,
            //rect2width: 0,
            //rect3width: 0,
            //rect4width: 0,
        }

    }

    

}

impl BattleScreenTransition for TrainerBattleScreenTransition {
    
    
}

impl BattleTransition for TrainerBattleScreenTransition {

    fn reset(&mut self) {
        self.rect0width = 0;
        //self.rect1width = 0;
        //self.rect2width = 0;
        //self.rect3width = 0;
        //self.rect4width = 0;
        self.finished = false;
    }

}

impl Loadable for TrainerBattleScreenTransition {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self, _context: &mut GameContext) {
        
    } 

}

impl Completable for TrainerBattleScreenTransition {

    fn is_finished(&self) -> bool {
        return self.finished;
    } 

}

impl Ticking for TrainerBattleScreenTransition {

    fn update(&mut self, _context: &mut GameContext) {
        self.rect0width += 4;
        if self.rect0width == BASE_WIDTH as u8 {
            self.finished = true;
        }
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
        draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], -(BASE_WIDTH as isize - self.rect0width as isize), 0  /* 32*0 */, BASE_WIDTH, 32);
        draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], BASE_WIDTH as isize - self.rect0width as isize, 32, /* 32*1 */ BASE_WIDTH, 32);
        draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], -(BASE_WIDTH as isize - self.rect0width as isize), 64  /* 32*0 */, BASE_WIDTH, 32);
        draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], BASE_WIDTH as isize - self.rect0width as isize, 96  /* 32*3 */, BASE_WIDTH, 32);
        draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], -(BASE_WIDTH as isize - self.rect0width as isize), 128  /* 32*0 */, BASE_WIDTH, 32);      
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