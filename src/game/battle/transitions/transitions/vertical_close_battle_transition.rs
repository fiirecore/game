use crate::engine::game_context::GameContext;
use crate::game::battle::transitions::traits::battle_intro::BattleIntro;
use crate::game::battle::transitions::traits::battle_transition::BattleTransition;
use crate::util::render_util::{VIEW_WIDTH, VIEW_HEIGHT};
use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;

use crate::util::render_util::draw_rect;

pub struct VerticalCloseBattleTransition {

    active: bool,
    finished: bool,

    offset: u8,
    speed: u8,

}

impl VerticalCloseBattleTransition {

    pub fn new() -> Self {

        Self {

            active: false,
            finished: false,

            offset: 0,
            speed: 2,

        }

    }

    

}

impl BattleIntro for VerticalCloseBattleTransition {}

impl BattleTransition for VerticalCloseBattleTransition {

    fn reset(&mut self) {
        self.offset = 0;
        self.speed = 2;
    }

    fn on_start(&mut self, _context: &mut GameContext) {
        
    }

    fn update(&mut self, _context: &mut GameContext) {
        if self.is_alive() {
            if self.offset == 24 {
                self.speed*=2;
            }
            self.offset += self.speed;
            if self.offset >= VIEW_HEIGHT as u8 / 2 {
                self.finished = true;
            }
        }        
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
        if self.is_alive() {
            draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], 0, 0, VIEW_WIDTH, self.offset as usize);
            draw_rect(ctx, g, [0.0, 0.0, 0.0, 1.0], 0, VIEW_HEIGHT as isize - self.offset as isize, VIEW_WIDTH, self.offset as usize);
        }        
    }

    fn is_finished(&self) -> bool {
        return self.finished;
    }    

}

impl Entity for VerticalCloseBattleTransition {

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