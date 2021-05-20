use game::{
    play_music_named,
    battle::BattleTrainerEntry,
};

use crate::{
    BattleType,
    state::TransitionState,
    ui::transitions::{
        BattleTransition,
        transitions::{
            BattleTransitions,
            FlashBattleTransition,
            TrainerBattleTransition,
        },
    },
};

#[derive(Default)]
pub struct BattleScreenTransitionManager {

    pub state: TransitionState,
    current: BattleTransitions,

    flash: FlashBattleTransition,
    trainer: TrainerBattleTransition,

}

impl BattleScreenTransitionManager {

    pub fn begin(&mut self, battle_type: BattleType, trainer: &Option<BattleTrainerEntry>) {
        self.play_music(battle_type);
        match trainer {
            Some(trainer) => self.current = BattleTransitions::from(trainer.transition),
            None => self.current = BattleTransitions::default(),
        }
        self.get_mut().reset();
        self.state = TransitionState::Run;
    }

    pub fn end(&mut self) {
        self.state = TransitionState::Begin;
    }
    
    pub fn update(&mut self, delta: f32) {
        let current = self.get_mut();
        current.update(delta);
        if current.is_finished() {
            self.state = TransitionState::End;
        }
    }

    pub fn render(&self) {
        self.get().render();
    }


    fn play_music(&mut self, battle_type: BattleType) {
        match battle_type {
            BattleType::Wild => play_music_named("BattleWild"),
            BattleType::Trainer => play_music_named("BattleTrainer"),
            BattleType::GymLeader => play_music_named("BattleGymLeader"),
        }
    }

    fn get(&self) -> &dyn BattleTransition {
        match self.current {
            BattleTransitions::Flash => &self.flash,
            BattleTransitions::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleTransition {
        match self.current {
            BattleTransitions::Flash => &mut self.flash,
            BattleTransitions::Trainer => &mut self.trainer,
        }
    }

}