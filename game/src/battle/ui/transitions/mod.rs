use crate::{
    util::Completable,
    gui::DynamicText,
};

use crate::battle::Battle;

pub mod managers;

pub mod transitions;
pub mod openers;
pub mod introductions;
pub mod closers;


pub(crate) trait BattleTransition: Completable {

    fn update(&mut self, delta: f32);

    fn render(&self);

    // fn render_below_player(&self);

}

pub(crate) trait BattleOpener: Completable  {

    fn spawn(&mut self, battle: &Battle);
    
    fn update(&mut self, delta: f32);

    fn render_below_panel(&self, battle: &Battle);

    fn render(&self);

    fn offset(&self) -> f32;

}

pub(crate) trait BattleIntroduction: Completable {

    fn spawn(&mut self, battle: &Battle, text: &mut DynamicText);

    fn update(&mut self, delta: f32, battle: &mut Battle, text: &mut DynamicText);

    fn render(&self, battle: &Battle);

}

pub(crate) trait BattleCloser: Completable {

    fn spawn(&mut self, battle: &Battle, text: &mut DynamicText);
    
    fn update(&mut self, delta: f32, text: &mut DynamicText);

    fn render(&self);

    fn render_battle(&self);

    fn world_active(&self) -> bool;

}