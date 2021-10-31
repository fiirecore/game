use pokedex::context::PokedexClientContext;
use worldlib::TrainerId;

use crate::{engine::{
        graphics::{draw_o_bottom, TextureManager},
        gui::MessageBox,
        tetra::graphics::Texture,
        text::MessagePage,
        util::{Completable, Entity, Reset, WIDTH},
        EngineContext,
    }, game::battle_glue::{BattleId, BattleTrainerEntry}};

use crate::battle::manager::transitions::BattleCloser;

use super::wild::WildBattleCloser;

pub struct TrainerBattleCloser {
    wild: WildBattleCloser,
    trainer: Option<Texture>,
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
    fn spawn<'d>(
        &mut self,
        ctx: &PokedexClientContext<'d>,
        player: &BattleId,
        player_name: &str,
        winner: Option<&BattleId>,
        trainer_entry: Option<&BattleTrainerEntry>,
        text: &mut MessageBox,
    ) {
        match winner {
            Some(winner) => match winner == player {
                true => {
                    if let Some(trainer) = trainer_entry {

                        log::debug!("todo set trainer textures and name in intro");

                        // self.trainer =
                        //     Some(ctx.trainer_textures.get(&trainer_data.npc_type).clone());

                        // text.reset();
                        // text.clear();

                        // text.push(MessagePage {
                        //     lines: vec![
                        //         String::from("Player defeated"),
                        //         format!("{} {}!", trainer_data.prefix, trainer_data.name),
                        //     ],
                        //     wait: None,
                        // });

                        for message in trainer.victory_message.iter() {
                            text.push(MessagePage {
                                lines: message.clone(),
                                wait: None,
                            });
                        }

                        text.push(MessagePage {
                            lines: vec![
                                format!("{} got ${}", player_name, trainer.worth),
                                String::from("for winning!"),
                            ],
                            wait: None,
                        });

                        text.spawn();
                    }
                }
                false => {
                    text.despawn();
                }
            },
            None => {
                text.despawn();
            }
        }
    }

    fn update(&mut self, ctx: &mut EngineContext, delta: f32, text: &mut MessageBox) {
        if text.alive() {
            text.update(ctx, delta);
            if text.page() == 1 && self.offset > Self::XPOS {
                self.offset -= 300.0 * delta;
                if self.offset < Self::XPOS {
                    self.offset = Self::XPOS;
                }
            }
            if text.finished() {
                text.despawn();
            }
        } else {
            self.wild.update(ctx, delta, text);
        }
    }

    fn world_active(&self) -> bool {
        self.wild.world_active()
    }

    fn draw(&self, ctx: &mut EngineContext) {
        self.wild.draw(ctx);
    }

    fn draw_battle(&self, ctx: &mut EngineContext) {
        draw_o_bottom(ctx, self.trainer.as_ref(), self.offset, 74.0);
    }
}

impl Completable for TrainerBattleCloser {
    fn finished(&self) -> bool {
        self.wild.finished()
    }
}

impl Reset for TrainerBattleCloser {
    fn reset(&mut self) {
        self.trainer = None;
        self.wild.reset();
        self.offset = WIDTH;
    }
}
