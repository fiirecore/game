use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::game::battle::battle::Battle;
use crate::gui::battle::battle_gui::BattleGui;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

pub trait BattleTransition: Loadable + Entity + Ticking + Completable {

    fn reset(&mut self);

}

pub trait BattleScreenTransition: BattleTransition {

    fn render_below_player(&mut self, _ctx: &mut Context, _g: &mut GlGraphics, _tr: &mut TextRenderer) {
        
    }

}

pub trait BattleOpener: BattleTransition  {

    fn offset(&self) -> u16;

    fn render_below_panel(&self, _ctx: &mut Context, _g: &mut GlGraphics, _tr: &mut TextRenderer) {

    }

}

pub trait BattleIntroduction: BattleTransition {

    fn update_gui(&mut self, battle_gui: &mut BattleGui);

    fn input(&mut self, context: &mut GameContext);

    fn setup_text(&mut self, battle: &Battle);

    fn render_offset(&self, ctx: &mut Context, g: &mut GlGraphics, battle: &Battle, offset: u16);

}

pub trait BattleCloser: BattleTransition {

}

pub trait BattleTransitionManager: Loadable + Entity + Ticking + Completable {

}

