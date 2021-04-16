use game::{
    util::{
        Entity,
        Reset, 
        Completable,
        text::{
            Message, 
            TextColor
        }
    },
    macroquad::prelude::{Vec2, Texture2D},
    graphics::{draw_bottom, draw_o_bottom},
};

use crate::{
    Battle,
    gui::BattleGui,
    transitions::{
        BattleTransition,
        BattleTransitionGui,
        BattleIntroduction,
        introductions::basic::BasicBattleIntroduction,
    }
};

pub const FINAL_TRAINER_OFFSET: f32 = 126.0;

pub struct TrainerBattleIntroduction {

    introduction: BasicBattleIntroduction,

    texture: Option<Texture2D>,
    offset: f32,
    leaving: bool,

}

impl TrainerBattleIntroduction {

    pub fn new(panel: Vec2) -> Self {
        Self {
            introduction: BasicBattleIntroduction::new(panel),
            texture: None,
            offset: 0.0,
            leaving: false,
        }
    }

}

impl BattleIntroduction for TrainerBattleIntroduction {

    fn setup(&mut self, battle: &Battle) {

        if let Some(trainer) = battle.trainer.as_ref() {

            self.texture = Some(trainer.texture);

            let name = format!("{} {}", trainer.npc_type, trainer.name);

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

        self.introduction.common_setup(battle);
        
    }

    fn update_gui(&mut self, battle: &Battle, battle_gui: &mut BattleGui, delta: f32) {
        self.introduction.update_gui(battle, battle_gui, delta);
        if self.introduction.text.can_continue {
            if let Some(messages) = self.introduction.text.messages.as_ref() {
                if self.introduction.text.current_message() == messages.len() - 2 {
                    self.leaving = true;
                }
            } else {
                self.leaving = true;
            }
            
        }
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        if self.offset < FINAL_TRAINER_OFFSET {
            draw_o_bottom(self.texture, 144.0 - offset + self.offset, 74.0);
        } else {
            draw_bottom(battle.opponent.active_texture(), 144.0 - offset, 74.0);
        }
        self.introduction.render_player(battle, offset);  
    }
}

impl BattleTransitionGui for TrainerBattleIntroduction {
    fn input(&mut self) {
        self.introduction.input();
    }
}

impl BattleTransition for TrainerBattleIntroduction {

    fn on_start(&mut self) {
        self.introduction.on_start();
    }

    fn update(&mut self, delta: f32) {
        self.introduction.update(delta);
        if self.leaving && self.offset < FINAL_TRAINER_OFFSET {
            self.offset += 300.0 * delta;
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
        self.offset = 0.0;
        self.leaving = false;
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