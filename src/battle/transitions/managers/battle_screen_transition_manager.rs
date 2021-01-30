use crate::audio::Music;
use crate::audio::play_music;
use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;
use crate::util::{Update, Render};
use crate::battle::transitions::battle_transition_traits::BattleScreenTransition;
use crate::battle::transitions::battle_transition_traits::BattleTransitionManager;
use crate::battle::transitions::screen_transitions::flash_battle_screen_transition::FlashBattleScreenTransition;
use crate::battle::transitions::screen_transitions::trainer_battle_screen_transition::TrainerBattleScreenTransition;
use crate::battle::transitions::screen_transitions::vertical_close_battle_screen_transition::VerticalCloseBattleScreenTransition;
use crate::util::{Reset, Completable};
use crate::util::Load;
use crate::battle::battle_info::BattleType;

pub struct BattleScreenTransitionManager {

    pub transitions: Vec<Box<dyn BattleScreenTransition>>,
    pub current_transition_id: usize,

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
        self.transitions.push(Box::new(VerticalCloseBattleScreenTransition::new()));
    }

    pub fn on_start(&mut self, battle_type: BattleType) {

        self.transitions[self.current_transition_id].spawn();
        self.transitions[self.current_transition_id].on_start();

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

    pub fn render_below_player(&mut self, tr: &TextRenderer) {
        self.transitions[self.current_transition_id].render_below_player(tr);
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

    fn render(&self, tr: &TextRenderer) {
        if self.is_alive() {
            self.transitions[self.current_transition_id].render(tr);
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