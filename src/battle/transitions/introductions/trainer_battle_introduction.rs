use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::battle::battle::Battle;
use crate::util::context::battle_context::TrainerData;
use crate::battle::transitions::battle_transition_traits::BattleIntroduction;
use crate::battle::transitions::battle_transition_traits::BattleTransition;
use crate::util::context::GameContext;
use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;
use crate::entity::Ticking;
use crate::gui::gui::Activatable;
use crate::util::render_util::draw_bottom;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;
use crate::world::npc::NPC;

use super::basic_battle_introduction::BasicBattleIntroduction;

static FINAL_TRAINER_OFFSET: u8 = 126;

pub struct TrainerBattleIntroduction {

    basic_battle_introduction: BasicBattleIntroduction,
    trainer_texture: Option<Texture>,
    trainer_offset: u8,
    trainer_leaving: bool,

}

impl TrainerBattleIntroduction {

    pub fn new(panel_x: isize, panel_y: isize) -> Self {

        Self {

            basic_battle_introduction: BasicBattleIntroduction::new(panel_x, panel_y),
            trainer_texture: None,
            trainer_offset: 0,
            trainer_leaving: false,

        }
    }

}

impl BattleIntroduction for TrainerBattleIntroduction {

    fn update_gui(&mut self, battle_gui: &mut crate::gui::battle::battle_gui::BattleGui) {
        self.basic_battle_introduction.update_gui(battle_gui);
        if self.basic_battle_introduction.intro_text.can_continue {
            if self.basic_battle_introduction.intro_text.next() == self.basic_battle_introduction.intro_text.text.len() as u8 - 2 {
                self.trainer_leaving = true;
            }
        }
    }

    fn input(&mut self, context: &mut GameContext) {
        self.basic_battle_introduction.input(context);
    }

    fn setup(&mut self, battle: &Battle, trainer_data: Option<&TrainerData>) {

        let mut player_string = String::from("Go! ");
                player_string.push_str(battle.player().pokemon.data.name.to_uppercase().as_str());
                player_string.push_str("!");

        match trainer_data {
            Some(trainer_data) => {

                self.trainer_texture = Some(NPC::battle_sprite(trainer_data.sprite_id));

                let mut opponent_string0 = trainer_data.name.clone();
                opponent_string0.push_str(" sent");
                let mut opponent_string1 = String::from("out "); 
                opponent_string1.push_str(battle.opponent().pokemon.data.name.to_uppercase().as_str());

                self.basic_battle_introduction.intro_text.text = vec![vec![trainer_data.name.clone(), String::from("would like to battle!")], vec![opponent_string0, opponent_string1]];
            }
            None => {
                self.basic_battle_introduction.intro_text.text = vec![vec![String::from("Missing trainer data")]];
            }
        }
        self.basic_battle_introduction.intro_text.text.push(vec![player_string]);
        
    }

    fn render_offset(&self, ctx: &mut Context, g: &mut GlGraphics, battle: &Battle, offset: u16) {
        if self.trainer_offset < FINAL_TRAINER_OFFSET {
            draw_bottom(ctx, g, self.trainer_texture.as_ref().unwrap(), 144 - offset as isize + self.trainer_offset as isize, 74);
        } else {
            draw_bottom(ctx, g, &battle.opponent_textures[battle.opponent_active], 144 - offset as isize, 74);
        }
        if self.basic_battle_introduction.player_intro.should_update() {
            self.basic_battle_introduction.player_intro.draw(ctx, g, offset);
        } else {
		    draw_bottom(ctx, g, &battle.player_textures[battle.player_active], 40 + offset as isize, 113);
        }  
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
        self.trainer_offset = 0;
        self.trainer_leaving = false;
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
        if self.trainer_leaving && self.trainer_offset < FINAL_TRAINER_OFFSET {
            self.trainer_offset += 5;
        }
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.basic_battle_introduction.render(ctx, g, tr);
    }

}