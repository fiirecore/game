
use crate::entity::Entity;
use crate::util::{Update, Render};
use crate::battle::transitions::BattleCloser;
use crate::battle::transitions::BattleTransitionManager;
use crate::battle::transitions::closers::basic_battle_closer::BasicBattleCloser;
use crate::util::{Reset, Completable};
use crate::util::Load;

#[derive(Default)]
pub struct BattleCloserManager {
    
    alive: bool,

    pub closers: Vec<Box<dyn BattleCloser>>,
    pub current_closer_id: usize,

}

impl BattleCloserManager { // return player data

    pub fn load_closers(&mut self) {
        self.closers.push(Box::new(BasicBattleCloser::new()));
    }

    pub fn world_active(&self) -> bool {
        self.closers[self.current_closer_id].world_active()
    }

}

impl Update for BattleCloserManager {

    fn update(&mut self, delta: f32) {
        if self.is_alive() {
            self.closers[self.current_closer_id].update(delta);
        }
    }

}

impl Render for BattleCloserManager {

    fn render(&self) {
        self.closers[self.current_closer_id].render();
    }

}

impl BattleTransitionManager for BattleCloserManager {}

impl Reset for BattleCloserManager {
    
    fn reset(&mut self) {
        self.closers[self.current_closer_id].reset();
    }
    
}

impl Load for BattleCloserManager {

    fn load(&mut self) {
        self.closers[self.current_closer_id].load();
    }

    fn on_start(&mut self) {
        self.closers[self.current_closer_id].on_start();
    }

}

impl Completable for BattleCloserManager {

    fn is_finished(&self) -> bool {
        return self.closers[self.current_closer_id].is_finished();
    }

}

impl Entity for BattleCloserManager {

    fn spawn(&mut self) {
        self.alive = true;
        self.closers[self.current_closer_id].spawn();
        self.reset();
    }    

    fn despawn(&mut self) {
        self.alive = false;
        self.closers[self.current_closer_id].despawn();
        self.reset();
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }

}