use game::util::{
    Entity, 
    Reset, 
    Completable,
    battle::BattleType,
};

use crate::{
    Battle,
    transitions::{
        BattleTransition,
        BattleTransitionGui,
        BattleCloser,
        closers::{
            Closers,
            wild::WildBattleCloser,
            trainer::TrainerBattleCloser,
        }
    }
};
pub struct BattleCloserManager {
    
    alive: bool,

    current: Closers,

    wild: WildBattleCloser,
    trainer: TrainerBattleCloser,

}

impl BattleCloserManager { // return player data

    pub fn new() -> Self {
        Self {
            alive: false,
            current: Closers::Wild,
            wild: WildBattleCloser::default(),
            trainer: TrainerBattleCloser::new(),
        }
    }

    pub fn spawn_closer(&mut self, battle: &Battle) {
        match battle.battle_type {
            BattleType::Wild => self.current = Closers::Wild,
            _ => self.current = Closers::Trainer,
        }
        self.spawn();
        self.setup(battle);
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

impl BattleTransitionGui for BattleCloserManager {

    fn input(&mut self) {
        self.get_mut().input();
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

    fn setup(&mut self, battle: &Battle) {
        self.get_mut().setup(battle);
    }

    fn world_active(&self) -> bool {
        self.get().world_active()
    }

    fn render_battle(&self) {
        self.get().render_battle();
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
        self.reset();
        self.get_mut().spawn();
    }    

    fn despawn(&mut self) {
        self.alive = false;
        self.get_mut().despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

}