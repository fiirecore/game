use firecore_util::{
    Entity,
    Reset, 
    Completable,
    text::{
        Message, 
        TextColor
    }
};

use macroquad::prelude::Vec2;

use crate::battle::{
    Battle,
    gui::BattleGui,
    transitions::{
        BattleTransition,
        BattleIntroduction,
        introductions::basic::BasicBattleIntroduction,
    }
};

use crate::util::graphics::{Texture, draw_bottom};
use crate::util::battle_data::TrainerData;

const FINAL_TRAINER_OFFSET: f32 = 126.0;

pub struct TrainerBattleIntroduction {

    introduction: BasicBattleIntroduction,
    trainer_texture: Option<Texture>,
    trainer_offset: f32,
    trainer_leaving: bool,

}

impl TrainerBattleIntroduction {

    pub fn new(panel: Vec2) -> Self {
        Self {
            introduction: BasicBattleIntroduction::new(panel),
            trainer_texture: None,
            trainer_offset: 0.0,
            trainer_leaving: false,
        }
    }

}

impl BattleIntroduction for TrainerBattleIntroduction {

    fn update_gui(&mut self, battle_gui: &mut BattleGui, delta: f32) {
        self.introduction.update_gui(battle_gui, delta);
        if self.introduction.text.can_continue {
            if let Some(messages) = self.introduction.text.messages.as_ref() {
                if self.introduction.text.current_message() == messages.len() - 2 {
                    self.trainer_leaving = true;
                }
            } else {
                self.trainer_leaving = true;
            }
            
        }
    }

    fn input(&mut self) {
        self.introduction.input();
    }

    fn setup(&mut self, battle: &Battle, trainer_data: Option<&TrainerData>) {

        if let Some(trainer_data) = trainer_data {

            self.trainer_texture = Some(crate::data::map::npc_texture::battle_sprite(trainer_data.npc_data.key()));

            let name = trainer_data.npc_data.identifier.clone() + " " + trainer_data.npc_name.as_str();

            self.introduction.text.messages = Some(vec![
                Message::new(
                    vec![
                        name.clone(), 
                        String::from("would like to battle!")
                    ], 
                    TextColor::White,
                    None, 
                ), 
                Message::new(
                    vec![
                        name + " sent", 
                        format!("out {}", battle.opponent.active().name())
                    ],
                    TextColor::White,
                    Some(0.5),
                ),
            ]);
            
        } else {
            self.introduction.text.messages = Some(vec![
                Message::new(
                    vec![String::from("No trainer data found!")],
                    TextColor::White,
                    None,
                )
            ]);
        }        

        if let Some(messages) = self.introduction.text.messages.as_mut() {
            messages.push(
                Message::new(
                    vec![
                        format!("Go! {}!", battle.player.active().name())
                    ],
                    TextColor::White,
                    Some(0.5),
                ),
            );
        }        

        self.introduction.common_setup(battle);
        
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        if self.trainer_offset < FINAL_TRAINER_OFFSET {
            if let Some(trainer_texture) = self.trainer_texture {
                draw_bottom(trainer_texture, 144.0 - offset + self.trainer_offset, 74.0);
            }
        } else {
            draw_bottom(battle.opponent.active_texture(), 144.0 - offset, 74.0);
        }
        if !self.introduction.player_intro.is_finished() {
            self.introduction.player_intro.draw(offset);
        } else {
		    draw_bottom(battle.player.active_texture(), 40.0 + offset, 113.0);
        }  
    }
}

impl BattleTransition for TrainerBattleIntroduction {

    fn on_start(&mut self) {
        self.introduction.on_start();
    }

    fn update(&mut self, delta: f32) {
        self.introduction.update(delta);
        if self.trainer_leaving && self.trainer_offset < FINAL_TRAINER_OFFSET {
            self.trainer_offset += 300.0 * delta;
        }
    }

    fn render(&self) {
        self.introduction.render();
    }

}

impl Completable for TrainerBattleIntroduction {

    fn is_finished(&self) -> bool {
        self.introduction.is_finished()
    }

}

impl Reset for TrainerBattleIntroduction {

    fn reset(&mut self) {
        self.introduction.reset();
        self.trainer_offset = 0.0;
        self.trainer_leaving = false;
    }

}

impl Entity for TrainerBattleIntroduction {

    fn spawn(&mut self) {
        self.introduction.spawn();
    }

    fn despawn(&mut self) {
        self.introduction.despawn();
    }

    fn is_alive(&self) -> bool {
        self.introduction.is_alive()
    }

}