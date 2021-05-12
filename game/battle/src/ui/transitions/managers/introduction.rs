use game::{
    util::{Entity, Reset, Completable},
    gui::DynamicText,
};

use crate::{
    Battle,
    ui::transitions::{
        BattleIntroduction,
        openers::Openers,
        introductions::{
            Introductions,
            basic::BasicBattleIntroduction, 
            trainer::TrainerBattleIntroduction
        },
    }
};

#[derive(Default)]
pub struct BattleIntroductionManager {

    alive: bool,
    
    current: Introductions,
    basic: BasicBattleIntroduction,
    trainer: TrainerBattleIntroduction,

}

impl BattleIntroductionManager {

    //

    pub fn update(&mut self, delta: f32, battle: &mut Battle, text: &mut DynamicText) {
        self.get_mut().update(delta, battle, text)
    }

    pub fn render(&self, battle: &Battle) {
        self.get().render(battle);
    }

    //

    pub fn spawn(&mut self, opener: &Openers, battle: &Battle, text: &mut DynamicText) {
        self.current = opener.intro();
        self.alive = true;
        self.reset();
        self.get_mut().spawn(battle, text);
        text.spawn();
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        self.reset();
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    fn get(&self) -> &dyn BattleIntroduction {
        match self.current {
            Introductions::Basic => &self.basic,
            Introductions::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleIntroduction {
        match self.current {
            Introductions::Basic => &mut self.basic,
            Introductions::Trainer => &mut self.trainer,
        }
    }

}

impl Reset for BattleIntroductionManager {
    fn reset(&mut self) {
        self.get_mut().reset();
    }
}

impl Completable for BattleIntroductionManager {
    fn is_finished(&self) -> bool {
        self.get().is_finished()
    }
}