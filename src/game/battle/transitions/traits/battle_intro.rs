use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::text::TextRenderer;

use super::battle_transition::BattleTransition;

pub trait BattleIntro: BattleTransition {

    fn render_below_player(&mut self, _ctx: &mut Context, _g: &mut GlGraphics, _tr: &mut TextRenderer) {
        
    }

}