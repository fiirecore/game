use game::{
    util::{Entity, Reset, Completable},
    macroquad::prelude::Vec2,
};

use crate::{
    Battle,
    gui::BattleGui,
    transitions::{
        BattleIntroduction, BattleTransition,
        introductions::{
            Introductions,
            basic::BasicBattleIntroduction, 
            trainer::TrainerBattleIntroduction
        },
        openers::Openers,
    }
};

pub struct BattleIntroductionManager {

    alive: bool,
    
    current_introduction: Introductions,
    basic: BasicBattleIntroduction,
    trainer: TrainerBattleIntroduction,

}

impl BattleIntroductionManager {

    pub fn new() -> Self {
        let panel = Vec2::new(0.0, 113.0);
        Self {
            alive: false,
            
            current_introduction: Introductions::default(),
            basic: BasicBattleIntroduction::new(panel),
            trainer: TrainerBattleIntroduction::new(panel),
        }
    }

    pub fn input(&mut self) {
		if self.is_alive() {
            self.get_mut().input();
        }
    }
    
    pub fn setup_text(&mut self, battle: &Battle) {
        self.get_mut().setup(battle);
    }

    pub fn render_with_offset(&self, battle: &Battle, offset: f32) {
        self.get().render_offset(battle, offset);
    }

    pub fn update_gui(&mut self, battle: &Battle, battle_gui: &mut BattleGui, delta: f32) {
        self.get_mut().update_gui(battle, battle_gui, delta);
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