use firecore_audio::music::Music;
use firecore_audio::play_music;
use crate::battle::transitions::BattleTransition;
use firecore_util::Entity;
use crate::battle::transitions::BattleScreenTransition;
use crate::battle::transitions::screen_transitions::flash_battle_screen_transition::FlashBattleScreenTransition;
use crate::battle::transitions::screen_transitions::trainer_battle_screen_transition::TrainerBattleScreenTransition;
//use crate::battle::transitions::screen_transitions::vertical_close_battle_screen_transition::VerticalCloseBattleScreenTransition;
use crate::util::{Reset, Completable};
// use crate::battle::battle_info::BattleType;
use firecore_world::{BattleType, BattleScreenTransitions};

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

    pub fn set_type(&mut self, battle_type: BattleType) {
        match battle_type {
            BattleType::Wild => {
                play_music(Music::BattleWild);
            }
            BattleType::Trainer => {
                play_music(Music::BattleTrainer);
            }
            BattleType::GymLeader => {
                play_music(Music::BattleGym);
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
            self.get_mut().update(delta);
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