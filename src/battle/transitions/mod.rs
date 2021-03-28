use firecore_util::{Entity, Completable};

use crate::battle::{Battle, gui::BattleGui};

use super::manager::TrainerTextures;

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

pub trait BattleIntroduction: BattleTransition {

    fn setup(&mut self, battle: &Battle, trainer_sprites: &TrainerTextures);

    fn update_gui(&mut self, battle: &Battle, battle_gui: &mut BattleGui, delta: f32);

    fn input(&mut self);

    fn render_offset(&self, battle: &Battle, offset: f32);

}

pub trait BattleCloser: BattleTransition {

    fn world_active(&self) -> bool;

}