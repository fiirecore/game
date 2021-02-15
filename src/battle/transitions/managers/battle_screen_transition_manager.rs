use crate::audio::music::Music;
use crate::audio::play_music;
use crate::entity::Entity;
use crate::util::battle_data::BattleData;
use crate::util::{Update, Render};
use crate::battle::transitions::BattleScreenTransition;
use crate::battle::transitions::BattleTransitionManager;
use crate::battle::transitions::screen_transitions::flash_battle_screen_transition::FlashBattleScreenTransition;
use crate::battle::transitions::screen_transitions::trainer_battle_screen_transition::TrainerBattleScreenTransition;
//use crate::battle::transitions::screen_transitions::vertical_close_battle_screen_transition::VerticalCloseBattleScreenTransition;
use crate::util::{Reset, Completable};
use crate::util::Load;
use crate::battle::battle_info::BattleType;

pub struct BattleScreenTransitionManager {

    pub transitions: Vec<Box<dyn BattleScreenTransition>>,
    pub current_transition_id: usize,

}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum BattleScreenTransitions {

    Flash,
    Trainer,

}

impl BattleScreenTransitions {

    fn id(&self) -> usize {
        match self {
            BattleScreenTransitions::Flash => 0,
            BattleScreenTransitions::Trainer => 1,
        }
    }

}

impl BattleScreenTransitionManager {

    pub fn new() -> Self {

        Self {

            transitions: Vec::new(),
            current_transition_id: 0,

        }

    }

    pub fn load_transitions(&mut self) {
        self.transitions.push(Box::new(FlashBattleScreenTransition::new()));
        self.transitions.push(Box::new(TrainerBattleScreenTransition::new()));
        //self.transitions.push(Box::new(VerticalCloseBattleScreenTransition::new()));
    }

    pub fn on_start(&mut self, battle_data: &BattleData) {

        if let Some(ref trainer) = battle_data.trainer_data {
            self.current_transition_id = trainer.transition.id();
        } else {
            self.current_transition_id = 0;
        }

        self.transitions[self.current_transition_id].spawn();
        self.transitions[self.current_transition_id].on_start();

        match battle_data.battle_type {
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

    pub fn render_below_player(&mut self) {
        self.transitions[self.current_transition_id].render_below_player();
    }

}

impl Update for BattleScreenTransitionManager {

    fn update(&mut self, delta: f32) {
        if self.is_alive() {
            self.transitions[self.current_transition_id].update(delta);
        }        
    }

}

impl Render for BattleScreenTransitionManager {

    fn render(&self) {
        if self.is_alive() {
            self.transitions[self.current_transition_id].render();
        }
    }

}

impl BattleTransitionManager for BattleScreenTransitionManager {}

impl Reset for BattleScreenTransitionManager {

    fn reset(&mut self) {
        self.transitions[self.current_transition_id].reset();
    }

}

impl Completable for BattleScreenTransitionManager {

    fn is_finished(&self) -> bool {
        return self.transitions[self.current_transition_id].is_finished();
    }

}

impl Load for BattleScreenTransitionManager {

    fn load(&mut self) {

    }

}

impl Entity for BattleScreenTransitionManager {

    fn spawn(&mut self) {
        self.transitions[self.current_transition_id].spawn();
        self.reset();
    }    

    fn despawn(&mut self) {
        self.transitions[self.current_transition_id].despawn();
        self.reset();
    }

    fn is_alive(&self) -> bool {
        return self.transitions[self.current_transition_id].is_alive();
    }

}