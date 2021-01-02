use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::text::TextRenderer;

use super::battle_transition::BattleTransition;

pub trait BattleOpener: BattleTransition {

    fn offset(&self) -> u16;

    fn render_below_panel(&self, _ctx: &mut Context, _g: &mut GlGraphics, _tr: &mut TextRenderer) {

    }

}