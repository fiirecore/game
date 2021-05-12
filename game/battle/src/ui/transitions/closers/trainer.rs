use game::{
    util::{
        Entity,
        Reset,
        Completable,
        WIDTH,
    },
    macroquad::prelude::Texture2D,
    graphics::draw_o_bottom,
    gui::DynamicText,
    text::MessagePage,
    battle::BattleTeam,
};

use crate::{
    Battle,
    ui::transitions::BattleCloser,
};

use super::wild::WildBattleCloser;

pub struct TrainerBattleCloser {
    wild: WildBattleCloser,
    trainer: Option<Texture2D>,
    offset: f32,
}

impl TrainerBattleCloser {
    const XPOS: f32 = 172.0; // 144 = pokemon
}

impl Default for TrainerBattleCloser {
    fn default() -> Self {
        Self {
            wild: WildBattleCloser::default(),
            trainer: None,
            offset: WIDTH,
        }
    }
}

impl BattleCloser for TrainerBattleCloser {

    fn spawn(&mut self, battle: &Battle, text: &mut DynamicText) {
        match battle.data.winner {
            Some(winner) => match winner {
                BattleTeam::Player => {
                    if let Some(trainer) = &battle.data.trainer {
                        self.trainer = Some(trainer.texture);

                        text.reset();

                        let mut message_set = Vec::with_capacity(trainer.victory_message.len() + 2);

                        message_set.push(MessagePage::new(
                            vec![
                                String::from("Player defeated"), 
                                format!("{} {}!", trainer.npc_type, trainer.name),
                            ],
                            None,
                        ));

                        for message in trainer.victory_message.iter() {
                            message_set.push(MessagePage::new(
                                message.clone(),
                                None,
                            ));
                        }

                        message_set.push(MessagePage::new(
                            vec![
                                format!("%p got ${}", trainer.worth),
                                String::from("for winning!")
                            ],
                            None
                        ));
                        
                        text.set(message_set);
                        
                        if let Some(saves) = game::storage::get::<game::storage::player::PlayerSaves>() {
                            text.process_messages(saves.get());
                        }
                    }
                }
                BattleTeam::Opponent => {
                    text.despawn();
                }
            }
            None => {
                text.despawn();
            }
        }
    }

    fn update(&mut self, delta: f32, text: &mut DynamicText) {
        if text.is_alive() {
            text.update(delta);
            if text.current() == 1 && self.offset > Self::XPOS {
                self.offset -= 300.0 * delta;
                if self.offset < Self::XPOS {
                    self.offset = Self::XPOS;
                }
            }
            if text.is_finished() {
                text.despawn();
            }
        } else {
            self.wild.update(delta, text);
        }
    }

    fn world_active(&self) -> bool {
        self.wild.world_active()
    }

    fn render(&self) {       
        self.wild.render();
    }

    fn render_battle(&self) {
        draw_o_bottom(self.trainer, self.offset, 74.0);
    }

}

impl Completable for TrainerBattleCloser {
    fn is_finished(&self) -> bool {
        self.wild.is_finished()
    }
}

impl Reset for TrainerBattleCloser {
    fn reset(&mut self) {
        self.trainer = None;
        self.wild.reset();
        self.offset = WIDTH;
    }
}