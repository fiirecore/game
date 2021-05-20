use game::{
    util::Entity,
    gui::DynamicText,
};

use crate::{
    Battle,
    state::TransitionState,
    ui::transitions::{
        BattleIntroduction,
        introductions::{
            Introductions,
            basic::BasicBattleIntroduction, 
            trainer::TrainerBattleIntroduction
        },
    }
};

#[derive(Default)]
pub struct BattleIntroductionManager {

    pub state: TransitionState,
    
    current: Introductions,
    basic: BasicBattleIntroduction,
    trainer: TrainerBattleIntroduction,

}

impl BattleIntroductionManager {

    pub fn begin(&mut self, battle: &Battle, text: &mut DynamicText) {
        self.state = TransitionState::Run;
        match battle.data.battle_type {
            crate::BattleType::Wild => self.current = Introductions::Basic,
            _ => self.current = Introductions::Trainer,
        }
        let current = self.get_mut();
        current.reset();
        current.spawn(battle, text);
        text.spawn();
    }

    pub fn end(&mut self) {
        self.state = TransitionState::Begin;
    }

    pub fn update(&mut self, delta: f32, battle: &mut Battle, text: &mut DynamicText) {
        let current = self.get_mut();
        current.update(delta, battle, text);
        if current.is_finished() {
            self.state = TransitionState::End;
        }
    }

    pub fn render(&self, battle: &Battle) {
        self.get().render(battle);
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