use crate::battle::battle_info::BattleType;

use crate::util::Entity;
use crate::battle::battle::Battle;
use crate::battle::transitions::BattleOpener;
use crate::battle::transitions::BattleTransition;
use crate::battle::transitions::openers::trainer_battle_opener::TrainerBattleOpener;
use crate::battle::transitions::openers::wild_battle_opener::WildBattleOpener;
use crate::util::{Reset, Completable};

use super::battle_introduction_manager::BattleIntroductionManager;
use super::battle_introduction_manager::Introductions;

pub struct BattleOpenerManager {

    alive: bool,

    current_opener: Openers,
    wild: WildBattleOpener,
    trainer: TrainerBattleOpener,

    pub battle_introduction_manager: BattleIntroductionManager,
}

pub enum Openers {

    Wild,
    Trainer,

}

impl Default for Openers {
    fn default() -> Self {
        Self::Wild
    }
}

impl Openers {

    pub fn intro(&self) -> Introductions {
        match self {
            Openers::Wild => Introductions::Basic,
            Openers::Trainer => Introductions::Trainer,
        }
    }

}

impl BattleOpenerManager {
    pub fn new() -> Self {
        Self {

            alive: false,

            current_opener: Openers::default(),
            wild: WildBattleOpener::new(),
            trainer: TrainerBattleOpener::new(),

            battle_introduction_manager: BattleIntroductionManager::new(),

        }
    }

    pub fn render_below_panel(&self, battle: &Battle) {
        self.battle_introduction_manager.render_with_offset(battle, self.offset());
        self.get().render_below_panel();
    }

    pub fn spawn_type(&mut self, battle_type: BattleType) {
        self.current_opener = match battle_type {
            BattleType::Wild => Openers::Wild,
            BattleType::Trainer => Openers::Trainer,
            BattleType::GymLeader => Openers::Trainer,
        };
        self.battle_introduction_manager.spawn_type(&self.current_opener);
        self.spawn();
    }

    fn get(&self) -> &dyn BattleOpener {
        match self.current_opener {
            Openers::Wild => &self.wild,
            Openers::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleOpener {
        match self.current_opener {
            Openers::Wild => &mut self.wild,
            Openers::Trainer => &mut self.trainer,
        }
    }

}

impl BattleTransition for BattleOpenerManager {

    fn on_start(&mut self) {
        self.get_mut().on_start();
    }

    fn update(&mut self, delta: f32) {
        if self.is_alive() {
            let opener = self.get_mut();
            if opener.is_alive() {
                if opener.is_finished() {
                    opener.despawn();
                    self.battle_introduction_manager.spawn();
                    self.battle_introduction_manager.on_start();
                } else {
                    opener.update(delta);
                }
            } else if self.battle_introduction_manager.is_alive() {
                self.battle_introduction_manager.update(delta);
            }
        }
    }

    fn render(&self) {
        if self.is_alive() {
            if self.battle_introduction_manager.is_alive() {
                self.battle_introduction_manager.render();
            } else {
                self.get().render();
            }
        }
    }

}

impl BattleOpener for BattleOpenerManager {

    fn offset(&self) -> f32 {
        return self.get().offset();
    }

    fn render_below_panel(&self) {
        macroquad::prelude::warn!("Using wrong render below panel method!");
    }
}

impl Reset for BattleOpenerManager {

    fn reset(&mut self) {
        self.get_mut().reset();
        self.battle_introduction_manager.reset();
    }

}

impl Completable for BattleOpenerManager {

    fn is_finished(&self) -> bool {
        return self.battle_introduction_manager.is_finished();
    }
    
}

impl Entity for BattleOpenerManager {
    fn spawn(&mut self) {
        self.alive = true;
        self.get_mut().spawn();
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.get_mut().despawn();
        self.battle_introduction_manager.despawn();
        self.reset();
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }
}
