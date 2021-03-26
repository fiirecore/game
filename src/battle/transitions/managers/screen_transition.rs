use crate::battle::transitions::BattleTransition;
use firecore_util::Entity;
use macroquad::prelude::warn;
use crate::battle::transitions::BattleScreenTransition;
use crate::battle::transitions::screen_transitions::flash::FlashBattleScreenTransition;
use crate::battle::transitions::screen_transitions::trainer::TrainerBattleScreenTransition;
use firecore_util::{Reset, Completable};
use firecore_util::battle::{BattleType, BattleScreenTransitions};
use firecore_audio::play_music_named as play_music;

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
        if let Err(err) = match battle_type {
            BattleType::Wild => {
                play_music("BattleWild")
            }
            BattleType::Trainer => {
                play_music("BattleTrainer")
            }
            BattleType::GymLeader => {
                play_music("BattleGymLeader")
            }
        } {
            warn!("Could not play battle music with error {}", err);
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