use crate::engine::game_context::GameContext;
use crate::game::battle::transitions::traits::battle_intro::BattleIntro;
use crate::game::battle::transitions::traits::battle_transition::BattleTransition;
use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;

use crate::util::render_util::draw_rect;

static FINAL_INDEX: u8 = 4;

pub struct FlashBattleTransition {

    active: bool,
    finished: bool,
    screen: [f32; 4],
    waning: bool,
    index: u8,
    fade: f32,
    zoom: bool,

}

impl FlashBattleTransition {

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

impl BattleIntro for FlashBattleTransition {}

impl BattleTransition for FlashBattleTransition {

    fn reset(&mut self) {
        self.screen = [1.0, 1.0, 1.0, 0.0];
        self.waning = false;
        self.index = 0;
        self.fade = 1.0 / 8.0;
        self.zoom = false;
    }

    fn on_start(&mut self, _context: &mut GameContext) {
        
    }

    fn update(&mut self, _context: &mut GameContext) {
        if self.is_alive() {
            if self.waning {
                self.screen[3] -= self.fade;
            } else {
                self.screen[3] += self.fade;
            }
            if self.screen[3] == 0.0 {
                self.waning = false;
                self.index+=1;
            } else if self.screen[3] == 1.0 {
                self.waning = true;
            }
            if self.index == FINAL_INDEX && self.screen[3] == 0.0 {
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

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
        if self.is_alive() {
            draw_rect(ctx, g, self.screen.into(), 0, 0, crate::util::render_util::VIEW_WIDTH, crate::util::render_util::VIEW_HEIGHT);
        }        
    }

    fn is_finished(&self) -> bool {
        return self.finished;
    }    

}

impl Entity for FlashBattleTransition {

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