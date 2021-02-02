use crate::util::battle_data::TrainerData;
use crate::util::texture::Texture;
use crate::battle::battle::Battle;
use crate::battle::transitions::battle_transition_traits::BattleIntroduction;
use crate::battle::transitions::battle_transition_traits::BattleTransition;

use crate::entity::Entity;
use crate::util::{Update, Render};
use crate::gui::Activatable;
use crate::util::render::draw_bottom;
use crate::util::{Reset, Completable};
use crate::util::Load;
use crate::world::npc::NPC;
use super::basic_battle_introduction::BasicBattleIntroduction;

static FINAL_TRAINER_OFFSET: f32 = 126.0;

pub struct TrainerBattleIntroduction {

    basic_battle_introduction: BasicBattleIntroduction,
    trainer_texture: Option<Texture>,
    trainer_offset: f32,
    trainer_leaving: bool,

}

impl TrainerBattleIntroduction {

    pub fn new(panel_x: f32, panel_y: f32) -> Self {

        Self {

            basic_battle_introduction: BasicBattleIntroduction::new(panel_x, panel_y),
            trainer_texture: None,
            trainer_offset: 0.0,
            trainer_leaving: false,

        }
    }

}

impl BattleIntroduction for TrainerBattleIntroduction {

    fn update_gui(&mut self, battle_gui: &mut crate::gui::battle::battle_gui::BattleGui, delta: f32) {
        self.basic_battle_introduction.update_gui(battle_gui, delta);
        if self.basic_battle_introduction.intro_text.can_continue {
            if self.basic_battle_introduction.intro_text.next() == self.basic_battle_introduction.intro_text.text.len() as u8 - 2 {
                self.trainer_leaving = true;
            }
        }
    }

    fn input(&mut self, delta: f32) {
        self.basic_battle_introduction.input(delta);
    }

    fn setup(&mut self, battle: &Battle, trainer_data: Option<&TrainerData>) {

        let mut player_string = String::from("Go! ");
                player_string.push_str(battle.player().data.name.to_uppercase().as_str());
                player_string.push_str("!");

        match trainer_data {
            Some(trainer_data) => {

                self.trainer_texture = Some(NPC::battle_sprite(trainer_data.sprite_id));

                let mut opponent_string0 = trainer_data.name.clone();
                opponent_string0.push_str(" sent");
                let mut opponent_string1 = String::from("out "); 
                opponent_string1.push_str(battle.opponent().data.name.to_uppercase().as_str());

                self.basic_battle_introduction.intro_text.text = vec![vec![trainer_data.name.clone(), String::from("would like to battle!")], vec![opponent_string0, opponent_string1]];
            }
            None => {
                self.basic_battle_introduction.intro_text.text = vec![vec![String::from("Missing trainer data")]];
            }
        }
        self.basic_battle_introduction.intro_text.text.push(vec![player_string]);
        
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        if self.trainer_offset < FINAL_TRAINER_OFFSET {
            if let Some(trainer_texture) = self.trainer_texture {
                draw_bottom(trainer_texture, 144.0 - offset + self.trainer_offset, 74.0);
            }
        } else {
            draw_bottom(battle.opponent_textures[battle.opponent_active], 144.0 - offset, 74.0);
        }
        if self.basic_battle_introduction.player_intro.should_update() {
            self.basic_battle_introduction.player_intro.draw(offset);
        } else {
		    draw_bottom(battle.player_textures[battle.player_active], 40.0 + offset, 113.0);
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

impl Reset for TrainerBattleIntroduction {

    fn reset(&mut self) {
        self.basic_battle_introduction.reset();
        self.trainer_offset = 0.0;
        self.trainer_leaving = false;
    }

}

impl Load for TrainerBattleIntroduction {

    fn load(&mut self) {
        self.basic_battle_introduction.load();
    }

    fn on_start(&mut self) {
        self.basic_battle_introduction.on_start();
    }

}

impl Completable for TrainerBattleIntroduction {

    fn is_finished(&self) -> bool {
        self.basic_battle_introduction.is_finished()
    }

}

impl Update for TrainerBattleIntroduction {

    fn update(&mut self, delta: f32) {
        self.basic_battle_introduction.update(delta);
        if self.trainer_leaving && self.trainer_offset < FINAL_TRAINER_OFFSET {
            self.trainer_offset += 300.0 * delta;
        }
    }

}

impl Render for TrainerBattleIntroduction {

    fn render(&self) {
        self.basic_battle_introduction.render();
    }

}

impl BattleTransition for TrainerBattleIntroduction {}