use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::battle::battle::Battle;
use crate::battle::battle_context::TrainerData;
use crate::battle::transitions::battle_transition_traits::BattleIntroduction;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::engine::game_context::GameContext;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::scene::scene::TextRenderer;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

use super::basic_battle_introduction::BasicBattleIntroduction;

pub struct TrainerBattleIntroduction {

    basic_battle_introduction: BasicBattleIntroduction,

}

impl TrainerBattleIntroduction {

    pub fn new(panel_x: isize, panel_y: isize) -> Self {

        Self {

            basic_battle_introduction: BasicBattleIntroduction::new(panel_x, panel_y),

        }
    }

}

impl BattleIntroduction for TrainerBattleIntroduction {

    fn update_gui(&mut self, battle_gui: &mut crate::gui::battle::battle_gui::BattleGui) {
        self.basic_battle_introduction.update_gui(battle_gui);
    }

    fn input(&mut self, context: &mut GameContext) {
        self.basic_battle_introduction.input(context);
    }

    fn setup_text(&mut self, battle: &Battle, trainer_data: Option<&TrainerData>) {

        let mut player_string = String::from("Go! ");
                player_string.push_str(battle.player().pokemon.name.to_uppercase().as_str());
                player_string.push_str("!");

        match trainer_data {
            Some(trainer_data) => {

                let mut opponent_string0 = trainer_data.name.clone();
                opponent_string0.push_str(" would like to battle!");

                let mut opponent_string1 = trainer_data.name.clone();
                opponent_string1.push_str(" sent out ");
                opponent_string1.push_str(battle.opponent().pokemon.name.to_uppercase().as_str());

                self.basic_battle_introduction.intro_text.text = vec![opponent_string0, opponent_string1, player_string];
            }
            None => {
                self.basic_battle_introduction.intro_text.text = vec![String::from("Missing trainer data"), player_string];
            }
        }
        
    }

    fn render_offset(&self, ctx: &mut Context, g: &mut GlGraphics, battle: &Battle, offset: u16) {
        self.basic_battle_introduction.render_offset(ctx, g, battle, offset);
    }
}

impl Entity for TrainerBattleIntroduction {

    fn spawn(&mut self) {
        self.basic_battle_introduction.spawn();
    }

    fn despawn(&mut self) {
        self.basic_battle_introduction.despawn();
    }

    fn is_alive(&self) -> bool {
        self.basic_battle_introduction.is_alive()
    }

}

impl BattleTransition for TrainerBattleIntroduction {

    fn reset(&mut self) {
        self.basic_battle_introduction.reset();
    }

}

impl Loadable for TrainerBattleIntroduction {

    fn load(&mut self) {
        self.basic_battle_introduction.load();
    }

    fn on_start(&mut self, context: &mut GameContext) {
        self.basic_battle_introduction.on_start(context);
    }

}

impl Completable for TrainerBattleIntroduction {

    fn is_finished(&self) -> bool {
        self.basic_battle_introduction.is_finished()
    }

}

impl Ticking for TrainerBattleIntroduction {

    fn update(&mut self, context: &mut GameContext) {
        self.basic_battle_introduction.update(context);
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.basic_battle_introduction.render(ctx, g, tr);
    }

}