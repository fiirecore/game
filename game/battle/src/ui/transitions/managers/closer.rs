use game::{
    util::{
        Reset, 
        Completable,
        battle::BattleType,
    },
    gui::DynamicText,
};

use crate::{
    Battle,
    ui::transitions::{
        BattleCloser,
        closers::{
            Closers,
            wild::WildBattleCloser,
            trainer::TrainerBattleCloser,
        }
    }
};

#[derive(Default)]
pub struct BattleCloserManager {
    
    alive: bool,

    current: Closers,

    wild: WildBattleCloser,
    trainer: TrainerBattleCloser,

}

impl BattleCloserManager { // return player data

    pub fn despawn(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    fn get(&self) -> &dyn BattleCloser {
        match self.current {
            Closers::Wild => &self.wild,
            Closers::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleCloser {
        match self.current {
            Closers::Wild => &mut self.wild,
            Closers::Trainer => &mut self.trainer,
        }
    }

}

impl BattleCloser for BattleCloserManager {

    fn spawn(&mut self, battle: &Battle, text: &mut DynamicText) {
        match battle.data.battle_type {
            BattleType::Wild => self.current = Closers::Wild,
            _ => self.current = Closers::Trainer,
        }
        self.alive = true;
        self.reset();
        self.get_mut().spawn(battle, text);
    }

    fn update(&mut self, delta: f32, text: &mut DynamicText) {
        self.get_mut().update(delta, text);
    }

    fn render(&self) {
        self.get().render();
    }

    fn render_battle(&self) {
        self.get().render_battle();
    }

    fn world_active(&self) -> bool {
        self.alive && self.get().world_active()
    }
}

impl Reset for BattleCloserManager {
    fn reset(&mut self) {
        self.get_mut().reset();
    } 
}

impl Completable for BattleCloserManager {
    fn is_finished(&self) -> bool {
        self.get().is_finished()
    }
}