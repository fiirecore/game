use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::audio::music::Music;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::game::battle::transitions::battle_transition_traits::BattleScreenTransition;
use crate::game::battle::transitions::battle_transition_traits::BattleTransitionManager;
use crate::game::battle::transitions::screen_transitions::flash_battle_screen_transition::FlashBattleScreenTransition;
use crate::game::battle::transitions::screen_transitions::trainer_battle_screen_transition::TrainerBattleScreenTransition;
use crate::game::battle::transitions::screen_transitions::vertical_close_battle_screen_transition::VerticalCloseBattleScreenTransition;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;
use music::Repeat::Forever;
use crate::game::battle::battle_info::BattleType;

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

    pub fn on_start(&mut self, context: &mut GameContext) {

        self.transitions[self.current_transition_id].spawn();
        self.transitions[self.current_transition_id].on_start(context);

        match context.battle_context.battle_data.as_ref().unwrap().battle_type {
            BattleType::Wild => {
                music::play_music(&Music::BattleWild, Forever);
            }
            BattleType::Trainer => {
                music::play_music(&Music::BattleTrainer, Forever);
            }
            BattleType::GymLeader => {
                music::play_music(&Music::BattleGym, Forever);
            }
        }
    }

    pub fn render_below_player(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.transitions[self.current_transition_id].render_below_player(ctx, g, tr);
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

        music::bind_music_file(Music::BattleWild, asset_as_pathbuf("audio/music/mus_vs_wild.mid"));
        music::bind_music_file(Music::BattleTrainer, asset_as_pathbuf("audio/music/mus_vs_trainer.mid"));
        music::bind_music_file(Music::BattleGym, asset_as_pathbuf("audio/music/mus_vs_gym_leader.mid"));
        music::bind_music_file(Music::BattleChampion, asset_as_pathbuf("audio/music/mus_vs_champion.mid"));

    }

}

impl Entity for BattleScreenTransitionManager {

    fn spawn(&mut self) {
        self.transitions[self.current_transition_id].spawn();
    }    

    fn despawn(&mut self) {
        self.transitions[self.current_transition_id].despawn();
    }

    fn is_alive(&self) -> bool {
        return self.transitions[self.current_transition_id].is_alive();
    }

}