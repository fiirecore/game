use firecore_util::{Entity, Reset, Completable, text::{Message, TextColor}};

use macroquad::prelude::{Vec2, Texture2D};

use crate::battle::{
    Battle,
    BattleWinner,
    manager::TrainerTextures,
    transitions::{
        BattleTransition,
        BattleTransitionGui,
        BattleCloser,
    }
};

use crate::WIDTH;
use crate::gui::text::DynamicText;
use crate::util::graphics::{draw_o_bottom};

use super::wild::WildBattleCloser;

const XPOS: f32 = 172.0; // 144 = pokemon

pub struct TrainerBattleCloser {

    alive: bool,

    wild: WildBattleCloser,

    text: DynamicText,

    trainer: Option<Texture2D>,
    offset: f32,

}

impl TrainerBattleCloser {
    pub fn new() -> Self {
        Self {
            alive: false,
            wild: WildBattleCloser::default(),
            text: DynamicText::new(Vec2::new(11.0, 11.0), Vec2::new(0.0, 113.0)),
            trainer: None,
            offset: WIDTH,
        }
    }
}

impl BattleTransitionGui for TrainerBattleCloser {

    fn input(&mut self) {
        self.text.input();
    }
}

impl BattleCloser for TrainerBattleCloser {

    fn setup(&mut self, battle: &Battle, textures: &TrainerTextures) {
        match battle.winner {
            Some(winner) => match winner {
                BattleWinner::Player => {
                    if let Some(trainer) = &battle.trainer {
                        self.trainer = textures.get(&trainer.identifier.npc_type).map(|texture| *texture);
                        self.text.messages = Some(vec![
                            Message::new(
                                vec![
                                    String::from("Player defeated"), 
                                    format!("{} {}!", trainer.npc_type.identifier, trainer.identifier.name),
                                ], 
                                TextColor::White,
                                None, 
                            ),
                            
                        ]);
                        let messages = self.text.messages.as_mut().unwrap();
                        for message in trainer.victory_message.iter() {
                            messages.push(Message::new(
                                message.clone(),
                                TextColor::White,
                                None,
                            ));
                        }
                        messages.push(
                            Message::new(
                                vec![
                                    format!("%p got ${}", trainer.worth),
                                    String::from("for winning!")
                                ],
                                TextColor::White,
                                None
                            )
                        );
                        crate::util::text::process_messages(firecore_data::get::<firecore_data::player::PlayerSaves>().unwrap().get(), messages);
                    }
                }
                BattleWinner::Opponent => {
                    self.wild.spawn();
                }
            }
            None => {
                self.wild.spawn();
            }
        }
        
        
    }

    fn world_active(&self) -> bool {
        self.wild.world_active()
    }

    fn render_battle(&self) {
        draw_o_bottom(self.trainer, self.offset, 74.0);
        self.text.render();
    }

}

impl BattleTransition for TrainerBattleCloser {
    fn on_start(&mut self) {
        
    }

    fn update(&mut self, delta: f32) {
        if self.wild.is_alive() {
            self.wild.update(delta);
        } else if self.text.is_finished() {
            self.wild.spawn();
        } else {
            self.text.update(delta);
            if self.text.current_message() == 1 && self.offset > XPOS {
                self.offset -= 300.0 * delta;
                if self.offset < XPOS {
                    self.offset = XPOS;
                }
            }
        }
    }

    fn render(&self) {       
        self.wild.render();
    }
}

impl Entity for TrainerBattleCloser {
    fn spawn(&mut self) {
        self.alive = true;
        self.text.spawn();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.text.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl Completable for TrainerBattleCloser {
    fn is_finished(&self) -> bool {
        self.wild.is_finished()
    }
}

impl Reset for TrainerBattleCloser {
    fn reset(&mut self) {
        self.text.reset();
        self.trainer = None;
        self.wild.reset();
        self.offset = WIDTH;
    }
}