use crate::battle::transitions::introductions::trainer_battle_introduction::TrainerBattleIntroduction;
use crate::util::battle_data::TrainerData;
use crate::entity::Entity;
use crate::battle::battle::Battle;
use crate::battle::transitions::BattleIntroduction;
use crate::battle::transitions::BattleTransition;
use crate::battle::transitions::introductions::basic_battle_introduction::BasicBattleIntroduction;
use crate::gui::battle::battle_gui::BattleGui;
use crate::util::{Reset, Completable};

use super::battle_opener_manager::Openers;

pub struct BattleIntroductionManager {

    alive: bool,
    
    current_introduction: Introductions,
    basic: BasicBattleIntroduction,
    trainer: TrainerBattleIntroduction,

}

pub enum Introductions {

    Basic,
    Trainer,

}

impl Default for Introductions {
    fn default() -> Self {
        Self::Basic
    }
}

impl BattleIntroductionManager {

    pub fn new() -> Self {
        Self {
            alive: false,
            
            current_introduction: Introductions::default(),
            basic: BasicBattleIntroduction::new(0.0, 113.0),
            trainer: TrainerBattleIntroduction::new(0.0, 113.0),
        }
    }

    pub fn input(&mut self, delta: f32) {
		if self.is_alive() {
            self.get_mut().input(delta);
        }
    }
    
    pub fn setup_text(&mut self, battle: &Battle, trainer_data: Option<&TrainerData>) {
        self.get_mut().setup(battle, trainer_data);
    }

    pub fn render_with_offset(&self, battle: &Battle, offset: f32) {
        self.get().render_offset(battle, offset);
    }

    pub fn update_gui(&mut self, battle_gui: &mut BattleGui, delta: f32) {
        self.get_mut().update_gui(battle_gui, delta);
    }

    pub fn spawn_type(&mut self, opener: &Openers) {
        self.current_introduction = opener.intro();
    }

    fn get(&self) -> &dyn BattleIntroduction {
        match self.current_introduction {
            Introductions::Basic => &self.basic,
            Introductions::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleIntroduction {
        match self.current_introduction {
            Introductions::Basic => &mut self.basic,
            Introductions::Trainer => &mut self.trainer,
        }
    }

}

impl BattleTransition for BattleIntroductionManager {

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

impl Reset for BattleIntroductionManager {

    fn reset(&mut self) {
        self.get_mut().reset();
    }

}

impl Completable for BattleIntroductionManager {

    fn is_finished(&self) -> bool {
        return self.get().is_finished();
    }
}

impl Entity for BattleIntroductionManager {

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
        return self.get().is_alive();
    }

}