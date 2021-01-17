use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::audio::music::Music;
use crate::util::context::GameContext;
use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;
use crate::entity::Ticking;
use crate::battle::transitions::battle_transition_traits::BattleScreenTransition;
use crate::battle::transitions::battle_transition_traits::BattleTransitionManager;
use crate::battle::transitions::screen_transitions::flash_battle_screen_transition::FlashBattleScreenTransition;
use crate::battle::transitions::screen_transitions::trainer_battle_screen_transition::TrainerBattleScreenTransition;
use crate::battle::transitions::screen_transitions::vertical_close_battle_screen_transition::VerticalCloseBattleScreenTransition;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;
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

    pub fn on_start(&mut self, context: &mut GameContext, battle_type: BattleType) {

        self.transitions[self.current_transition_id].spawn();
        self.transitions[self.current_transition_id].on_start(context);

        match battle_type {
            BattleType::Wild => {
                context.play_music(Music::BattleWild);
            }
            BattleType::Trainer => {
                context.play_music(Music::BattleTrainer);
            }
            BattleType::GymLeader => {
                context.play_music(Music::BattleGym);
            }
        }
    }

    pub fn render_below_player(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.transitions[self.current_transition_id].render_below_player(ctx, g, tr);
    }

    fn reset(&mut self) {
        self.transitions[self.current_transition_id].reset();
    }

}

impl Ticking for BattleScreenTransitionManager {

    fn update(&mut self, context: &mut GameContext) {
        if self.is_alive() {
            self.transitions[self.current_transition_id].update(context);
        }        
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        if self.is_alive() {
            self.transitions[self.current_transition_id].render(ctx, g, tr);
        }
    }

}

impl BattleTransitionManager for BattleScreenTransitionManager {

}

impl Completable for BattleScreenTransitionManager {

    fn is_finished(&self) -> bool {
        return self.transitions[self.current_transition_id].is_finished();
    }

}

impl Loadable for BattleScreenTransitionManager {

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