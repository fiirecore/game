use firecore_util::text::Message;
use firecore_util::text::MessageSet;
use firecore_util::text::TextColor;
use crate::util::battle_data::TrainerData;
use crate::util::graphics::Texture;
use crate::battle::battle::Battle;
use crate::battle::transitions::BattleIntroduction;
use crate::battle::transitions::BattleTransition;
use firecore_util::Entity;
use crate::util::graphics::draw_bottom;
use crate::util::{Reset, Completable};
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
            if self.basic_battle_introduction.intro_text.current_phrase() == self.basic_battle_introduction.intro_text.text.len() as u8 - 2 {
                self.trainer_leaving = true;
            }
        }
    }

    fn input(&mut self, delta: f32) {
        self.basic_battle_introduction.input(delta);
    }

    fn setup(&mut self, battle: &Battle, trainer_data: Option<&TrainerData>) {

        if let Some(trainer_data) = trainer_data {

            self.trainer_texture = Some(crate::io::data::map::npc_texture::battle_sprite(&trainer_data.npc_type));

            self.basic_battle_introduction.intro_text.text = MessageSet {
                messages: vec![
                    Message::with_color(vec![trainer_data.name.clone(), String::from("would like to battle!")], false, TextColor::White), 
                    Message::with_color(vec![trainer_data.name.clone() + " sent", String::from("out ") + &battle.opponent().pokemon.data.name.to_ascii_uppercase()], true, TextColor::White),
                ]
            };
            
        } else {
            self.basic_battle_introduction.intro_text.text = MessageSet { messages: vec![Message::with_color(vec![String::from("No trainer data found!")], false, TextColor::White)] };
        }        

        self.basic_battle_introduction.intro_text.text.messages.push(
            Message::with_color(vec![String::from("Go! ") + battle.player().pokemon.data.name.to_ascii_uppercase().as_str() + "!"], true, TextColor::White),
        );

        self.basic_battle_introduction.common_setup(battle);
        
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        if self.trainer_offset < FINAL_TRAINER_OFFSET {
            if let Some(trainer_texture) = self.trainer_texture {
                draw_bottom(trainer_texture, 144.0 - offset + self.trainer_offset, 74.0);
            }
        } else {
            draw_bottom(battle.opponent_textures[battle.opponent_active], 144.0 - offset, 74.0);
        }
        if !self.basic_battle_introduction.player_intro.is_finished() {
            self.basic_battle_introduction.player_intro.draw(offset);
        } else {
		    draw_bottom(battle.player_textures[battle.player_active], 40.0 + offset, 113.0);
        }  
    }
}

impl BattleTransition for TrainerBattleIntroduction {

    fn on_start(&mut self) {
        self.basic_battle_introduction.on_start();
    }

    fn update(&mut self, delta: f32) {
        self.basic_battle_introduction.update(delta);
        if self.trainer_leaving && self.trainer_offset < FINAL_TRAINER_OFFSET {
            self.trainer_offset += 300.0 * delta;
        }
    }

    fn render(&self) {
        self.basic_battle_introduction.render();
    }

}

impl Completable for TrainerBattleIntroduction {

    fn is_finished(&self) -> bool {
        self.basic_battle_introduction.is_finished()
    }

}

impl Reset for TrainerBattleIntroduction {

    fn reset(&mut self) {
        self.basic_battle_introduction.reset();
        self.trainer_offset = 0.0;
        self.trainer_leaving = false;
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