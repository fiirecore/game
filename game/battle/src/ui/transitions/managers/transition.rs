use game::{
    util::{
        Reset,
        Completable,
        battle::{
            BattleType,
            BattleScreenTransitions,
        },
    },
    play_music_named,
};

use crate::ui::transitions::{
    BattleTransition,
    transitions::{
        flash::FlashBattleTransition,
        trainer::TrainerBattleTransition,
    },
};

#[derive(Default)]
pub struct BattleScreenTransitionManager {

    alive: bool,
    current: BattleScreenTransitions,

    flash: FlashBattleTransition,
    trainer: TrainerBattleTransition,

}

impl BattleScreenTransitionManager {

    pub fn spawn(&mut self, battle_type: BattleType) {
        self.alive = true;
        self.set_type(battle_type);
        self.reset();
    }

    pub fn despawn(&mut self) {
        self.alive = false;
        self.reset();
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    fn set_type(&mut self, battle_type: BattleType) {
        match battle_type {
            BattleType::Wild => play_music_named("BattleWild"),
            BattleType::Trainer => play_music_named("BattleTrainer"),
            BattleType::GymLeader => play_music_named("BattleGymLeader"),
        }
    }

    fn get(&self) -> &dyn BattleTransition {
        match self.current {
            BattleScreenTransitions::Flash => &self.flash,
            BattleScreenTransitions::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleTransition {
        match self.current {
            BattleScreenTransitions::Flash => &mut self.flash,
            BattleScreenTransitions::Trainer => &mut self.trainer,
        }
    }

}

impl BattleTransition for BattleScreenTransitionManager {

    fn update(&mut self, delta: f32) {
        self.get_mut().update(delta);       
    }

    fn render(&self) {
        self.get().render();
    }

    // fn render_below_player(&self) {
    //     self.get().render_below_player();
    // }
}

impl Reset for BattleScreenTransitionManager {
    fn reset(&mut self) {
        self.get_mut().reset();
    }
}

impl Completable for BattleScreenTransitionManager {
    fn is_finished(&self) -> bool {
        self.get().is_finished()
    }
}