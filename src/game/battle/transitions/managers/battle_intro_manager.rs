use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::audio::music::Music;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::game::battle::transitions::transitions::{flash_battle_transition::FlashBattleTransition, trainer_battle_transition::TrainerBattleTransition, vertical_close_battle_transition::VerticalCloseBattleTransition};
use crate::game::battle::transitions::traits::battle_intro::BattleIntro;
use crate::game::battle::transitions::traits::battle_transition_manager::BattleTransitionManager;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::traits::Loadable;
use music::Repeat::Forever;

pub struct BattleIntroManager {

    pub alive: bool,

    pub intros: Vec<Box<dyn BattleIntro>>,
    pub current_intro_id: usize,

}

impl BattleIntroManager {

    pub fn new() -> Self {

        Self {

            alive: false,

            intros: Vec::new(),
            current_intro_id: 0,

        }

    }

    pub fn load_intros(&mut self) {
        self.intros.push(Box::new(TrainerBattleTransition::new()));
        self.intros.push(Box::new(FlashBattleTransition::new()));
        self.intros.push(Box::new(VerticalCloseBattleTransition::new()));
    }

    pub fn render_below_player(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.intros[self.current_intro_id].render_below_player(ctx, g, tr);
    }

}

impl Ticking for BattleIntroManager {

    fn update(&mut self, context: &mut GameContext) {
        self.intros[self.current_intro_id].update(context);
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.intros[self.current_intro_id].render(ctx, g, tr);
    }

}

impl BattleTransitionManager for BattleIntroManager {

    fn is_finished(&self) -> bool {
        return self.intros[self.current_intro_id].is_finished();
    }

}

impl Loadable for BattleIntroManager {

    fn load(&mut self) {

        music::bind_music_file(Music::BattleWild, asset_as_pathbuf("audio/music/mus_vs_wild.mid"));
        music::bind_music_file(Music::BattleTrainer, asset_as_pathbuf("audio/music/mus_vs_trainer.mid"));
        music::bind_music_file(Music::BattleGym, asset_as_pathbuf("audio/music/mus_vs_gym_leader.mid"));
        music::bind_music_file(Music::BattleChampion, asset_as_pathbuf("audio/music/mus_vs_champion.mid"));

    }

    fn on_start(&mut self, context: &mut GameContext) {

        match context.random.rand_range(0..4) {
            0 => {
                music::play_music(&Music::BattleWild, Forever);
            },
            1 => {
                music::play_music(&Music::BattleTrainer, Forever);
            },
            2 => {
                music::play_music(&Music::BattleGym, Forever);
            }
            3 => {
                music::play_music(&Music::BattleChampion, Forever);
            },
            _ => {
                music::play_music(&Music::BattleWild, Forever);
            }
        } 

        self.current_intro_id = context.random.rand_range(0..self.intros.len() as u32) as usize;

        self.intros[self.current_intro_id].spawn();
        self.intros[self.current_intro_id].on_start(context);
    }
}

impl Entity for BattleIntroManager {

    fn spawn(&mut self) {
        self.alive = true;
        //self.intros[self.current_intro_id].spawn();
    }    

    fn despawn(&mut self) {
        self.alive = false;
        self.intros[self.current_intro_id].despawn();
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }

}