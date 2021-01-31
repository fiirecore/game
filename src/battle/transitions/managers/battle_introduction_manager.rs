use crate::battle::transitions::introductions::trainer_battle_introduction::TrainerBattleIntroduction;
use crate::util::battle_data::TrainerData;
use crate::entity::Entity;
use crate::util::{Update, Render};
use crate::battle::battle::Battle;
use crate::battle::transitions::battle_transition_traits::BattleIntroduction;
use crate::battle::transitions::battle_transition_traits::BattleTransitionManager;
use crate::battle::transitions::introductions::basic_battle_introduction::BasicBattleIntroduction;
use crate::gui::battle::battle_gui::BattleGui;
use crate::util::{Reset, Completable};
use crate::util::Load;

pub struct BattleIntroductionManager {

    alive: bool,
    
    introductions: Vec<Box<dyn BattleIntroduction>>,
    pub current_introduction_index: usize,

}

impl BattleIntroductionManager {

    pub fn new() -> Self {
        let mut this = Self {

            alive: false,
            
            introductions: Vec::new(),
            current_introduction_index: 0,

        };
        this.load_introductions();
        this
    }

    fn load_introductions(&mut self) {
        self.introductions.push(Box::new(BasicBattleIntroduction::new(0.0, 113.0)));
        self.introductions.push(Box::new(TrainerBattleIntroduction::new(0.0, 113.0)));
        for introduction in &mut self.introductions {
            introduction.load();
        }
    }

    pub fn input(&mut self, delta: f32) {
		if self.is_alive() {
            self.introductions[self.current_introduction_index].input(delta);
        }
    }
    
    pub fn setup_text(&mut self, battle: &Battle, trainer_data: Option<&TrainerData>) {
        self.introductions[self.current_introduction_index].setup(battle, trainer_data);
    }

    pub fn render_with_offset(&self, battle: &Battle, offset: f32) {
        self.introductions[self.current_introduction_index].render_offset(battle, offset);
    }

    pub fn update_gui(&mut self, battle_gui: &mut BattleGui, delta: f32) {
        self.introductions[self.current_introduction_index].update_gui(battle_gui, delta);
    }

    pub fn spawn_type(&mut self, index: usize) {
        self.current_introduction_index = index;
    }

}


impl Update for BattleIntroductionManager {

    fn update(&mut self, delta: f32) {
        self.introductions[self.current_introduction_index].update(delta);     
	}
}

impl Render for BattleIntroductionManager {

    fn render(&self, tr: &crate::util::text_renderer::TextRenderer) {
        self.introductions[self.current_introduction_index].render(tr);
	}

}

impl BattleTransitionManager for BattleIntroductionManager {}

impl Reset for BattleIntroductionManager {

    fn reset(&mut self) {
        self.introductions[self.current_introduction_index].reset();
    }

}

impl Completable for BattleIntroductionManager {

    fn is_finished(&self) -> bool {
        return self.introductions[self.current_introduction_index].is_finished();
    }
}

impl Entity for BattleIntroductionManager {

    fn spawn(&mut self) {
        self.alive = true;
        self.introductions[self.current_introduction_index].spawn();
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.introductions[self.current_introduction_index as usize].despawn();
        self.reset();
    }

    fn is_alive(&self) -> bool {
        return self.introductions[self.current_introduction_index as usize].is_alive();
    }
}

impl Load for BattleIntroductionManager {

    fn load(&mut self) {
        self.introductions[self.current_introduction_index].load();
    }

    fn on_start(&mut self) {
        self.introductions[self.current_introduction_index].on_start();
    }

}