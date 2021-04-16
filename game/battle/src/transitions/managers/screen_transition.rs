use game::{
    util::{
        Entity,
        Reset,
        Completable,
        battle::{
            BattleType,
            BattleScreenTransitions,
        },
    },
    play_music_named,
};

use crate::transitions::{
    BattleTransition,
    BattleScreenTransition,
    screen_transitions::{
        flash::FlashBattleScreenTransition,
        trainer::TrainerBattleScreenTransition,
    },
};

pub struct BattleScreenTransitionManager {

    
    current_transition: BattleScreenTransitions,

    flash: FlashBattleScreenTransition,
    trainer: TrainerBattleScreenTransition,

}

impl BattleScreenTransitionManager {

    pub fn new() -> Self {
        Self {
            current_transition: BattleScreenTransitions::default(),
            flash: FlashBattleScreenTransition::new(),
            trainer: TrainerBattleScreenTransition::new(),
        }
    }

    pub fn spawn_with_type(&mut self, battle_type: BattleType) {
        self.spawn();
        self.set_type(battle_type);
    }

    pub fn set_type(&mut self, battle_type: BattleType) {
        match battle_type {
            BattleType::Wild => {
                play_music_named("BattleWild")
            }
            BattleType::Trainer => {
                play_music_named("BattleTrainer")
            }
            BattleType::GymLeader => {
                play_music_named("BattleGymLeader")
            }
        }
    }

    fn get(&self) -> &dyn BattleScreenTransition {
        match self.current_transition {
            BattleScreenTransitions::Flash => &self.flash,
            BattleScreenTransitions::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleScreenTransition {
        match self.current_transition {
            BattleScreenTransitions::Flash => &mut self.flash,
            BattleScreenTransitions::Trainer => &mut self.trainer,
        }
    }

}

impl BattleTransition for BattleScreenTransitionManager {

    fn on_start(&mut self) {
        self.spawn();
        self.get_mut().on_start();
    }

    fn update(&mut self, delta: f32) {
        if self.is_alive() {
            self.get_mut().update(delta
                //  * if macroquad::prelude::is_key_down(macroquad::prelude::KeyCode::Space) {
                //     8.0
                // } else {
                //     1.0
                // }
            );
        }        
    }

    fn render(&self) {
        if self.is_alive() {
            self.get().render();
        }
    }

}

impl BattleScreenTransition for BattleScreenTransitionManager {
    fn render_below_player(&self) {
        self.get().render_below_player();
    }
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

impl Entity for BattleScreenTransitionManager {

    fn spawn(&mut self) {
        self.get_mut().spawn();
        self.reset();
    }    

    fn despawn(&mut self) {
        self.get_mut().despawn();
        self.reset();
    }

    fn is_alive(&self) -> bool {
        self.get().is_alive()
    }

}