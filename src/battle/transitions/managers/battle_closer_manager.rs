
use firecore_util::Entity;
use crate::battle::transitions::BattleCloser;
use crate::battle::transitions::BattleTransition;
use crate::battle::transitions::closers::basic_battle_closer::BasicBattleCloser;
use crate::util::{Reset, Completable};

#[derive(Default)]
pub struct BattleCloserManager {
    
    alive: bool,

    current_closer: Closers,
    basic: BasicBattleCloser,

}

pub enum Closers {

    Basic,

}

impl Default for Closers {
    fn default() -> Self {
        Self::Basic
    }
}

impl BattleCloserManager { // return player data

    fn get(&self) -> &dyn BattleCloser {
        match self.current_closer {
            Closers::Basic => &self.basic,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleCloser {
        match self.current_closer {
            Closers::Basic => &mut self.basic,
        }
    }

}

impl BattleTransition for BattleCloserManager {

    fn on_start(&mut self) {
        self.get_mut().on_start();
    }

    fn update(&mut self, delta: f32) {
        self.get_mut().update(delta);
    }

    fn render(&self) {
        self.get().render();
    }

}

impl BattleCloser for BattleCloserManager {
    fn world_active(&self) -> bool {
        self.get().world_active()
    }
}

impl Reset for BattleCloserManager {
    
    fn reset(&mut self) {
        self.get_mut().reset();
    }
    
}

impl Completable for BattleCloserManager {

    fn is_finished(&self) -> bool {
        return self.get().is_finished();
    }

}

impl Entity for BattleCloserManager {

    fn spawn(&mut self) {
        self.alive = true;
        self.get_mut().spawn();
        self.reset();
    }    

    fn despawn(&mut self) {
        self.alive = false;
        self.get_mut().despawn();
        self.reset();
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }

}