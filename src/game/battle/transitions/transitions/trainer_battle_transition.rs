use crate::engine::game_context::GameContext;
use crate::game::battle::transitions::traits::battle_intro::BattleIntro;
use crate::game::battle::transitions::traits::battle_transition::BattleTransition;
use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;

use crate::util::render_util::draw_rect;

use crate::util::render_util::VIEW_WIDTH;

pub struct TrainerBattleTransition {

    pub active: bool,
    pub finished: bool,

    rect0width: u8,

    //rect1width: u8,

    //rect2width: u8,

    //rect3width: u8,

    //rect4width: u8,

}

impl TrainerBattleTransition {

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

impl BattleIntro for TrainerBattleTransition {}

impl BattleTransition for TrainerBattleTransition {

    fn reset(&mut self) {
        self.rect0width = 0;
        //self.rect1width = 0;
        //self.rect2width = 0;
        //self.rect3width = 0;
        //self.rect4width = 0;
        self.finished = false;
    }

    fn on_start(&mut self, _context: &mut GameContext) {
        
    }

    fn update(&mut self, _context: &mut GameContext) {
        if self.is_alive() {
            self.rect0width += 4;
            if self.rect0width == VIEW_WIDTH as u8 {
                self.finished = true;
            }
        }
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
        if self.is_alive() {
            draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], -(VIEW_WIDTH as isize - self.rect0width as isize), 0  /* 32*0 */, VIEW_WIDTH, 32);
            draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], VIEW_WIDTH as isize - self.rect0width as isize, 32, /* 32*1 */ VIEW_WIDTH, 32);
            draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], -(VIEW_WIDTH as isize - self.rect0width as isize), 64  /* 32*0 */, VIEW_WIDTH, 32);
            draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], VIEW_WIDTH as isize - self.rect0width as isize, 96  /* 32*3 */, VIEW_WIDTH, 32);
            draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], -(VIEW_WIDTH as isize - self.rect0width as isize), 128  /* 32*0 */, VIEW_WIDTH, 32);
        }        
    }

    fn is_finished(&self) -> bool {
        return self.finished;
    }    

}

impl Entity for TrainerBattleTransition {

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