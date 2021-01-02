use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;

pub trait BattleTransition: Entity {

    fn reset(&mut self);

    fn on_start(&mut self, context: &mut GameContext);

    fn update(&mut self, context: &mut GameContext);

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer);

    fn is_finished(&self) -> bool;

}