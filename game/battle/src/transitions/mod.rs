use game::util::{Entity, Completable};

use crate::{Battle, gui::BattleGui};

pub mod managers;

pub mod screen_transitions;
pub mod openers;
pub mod introductions;
pub mod closers;

pub trait BattleTransition: Entity + Completable {

    fn on_start(&mut self);

    fn update(&mut self, delta: f32);

    fn render(&self);

}

pub trait BattleScreenTransition: BattleTransition {

    fn render_below_player(&self) {}

}

pub trait BattleOpener: BattleTransition  {

    fn offset(&self) -> f32;

    fn render_below_panel(&self) {}

}

pub trait BattleIntroduction: BattleTransition + BattleTransitionGui {

    fn setup(&mut self, battle: &Battle);

    fn update_gui(&mut self, battle: &Battle, battle_gui: &mut BattleGui, delta: f32);

    fn render_offset(&self, battle: &Battle, offset: f32);

}

pub trait BattleCloser: BattleTransition + BattleTransitionGui {

    #[allow(unused_variables)]
    fn setup(&mut self, battle: &Battle) {}

    fn world_active(&self) -> bool;

    fn render_battle(&self) {}

}

pub trait BattleTransitionGui: BattleTransition {

    fn input(&mut self) {}

}