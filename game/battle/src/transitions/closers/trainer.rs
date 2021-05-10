use game::{
    util::{
        Entity,
        Reset,
        Completable,
        WIDTH,
    },
    macroquad::prelude::{Vec2, Texture2D},
    graphics::draw_o_bottom,
    gui::text::DynamicText,
    text::MessagePage,
    battle::BattleTeam,
};

use crate::{
    Battle,
    transitions::{
        BattleTransition,
        BattleTransitionGui,
        BattleCloser,
    }
};

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
            text: DynamicText::new(Vec2::new(11.0, 11.0), Vec2::new(0.0, 113.0), 1, game::text::TextColor::White, 2, "btltinro"),
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

    fn setup(&mut self, battle: &Battle) {
        match battle.winner {
            Some(winner) => match winner {
                BattleTeam::Player => {
                    if let Some(trainer) = &battle.trainer {
                        self.trainer = Some(trainer.texture);

                        let mut message_set = Vec::with_capacity(trainer.victory_message.len() + 2);

                        message_set.push(MessagePage::new(
                            vec![
                                String::from("Player defeated"), 
                                format!("{} {}!", trainer.npc_type, trainer.name),
                            ],
                            None
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
                        
                        self.text.set(message_set);
                        
                        if let Some(saves) = game::storage::get::<game::storage::player::PlayerSaves>() {
                            self.text.process_messages(saves.get());
                        }
                    }
                }
                BattleTeam::Opponent => {
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
        self.text.render(#[cfg(debug_assertions)] "render");
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
            self.text.update(delta, #[cfg(debug_assertions)] "update");
            if self.text.current() == 1 && self.offset > XPOS {
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