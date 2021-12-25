use crate::pokedex::PokedexClientData;

use crate::{
    engine::{
        graphics::Texture,
        gui::MessageBox,
        text::MessagePage,
        utils::{Completable, Entity, Reset, WIDTH},
        Context,
    },
    game::battle_glue::{BattleId, BattleTrainerEntry},
};

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
        _: &PokedexClientData,
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
                        crate::engine::log::debug!("todo set trainer textures and name in intro");

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

                        for message in trainer.defeat.iter() {
                            text.pages.push(message.clone());
                        }

                        text.pages.push(MessagePage {
                            lines: vec![
                                format!("{} got ${}", player_name, trainer.worth),
                                String::from("for winning!"),
                            ],
                            wait: None,
                            color: MessagePage::WHITE,
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

    fn update(&mut self, ctx: &mut Context, delta: f32, text: &mut MessageBox) {
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

    fn draw(&self, ctx: &mut Context) {
        self.wild.draw(ctx);
    }

    fn draw_battle(&self, ctx: &mut Context) {
        if let Some(texture) = self.trainer.as_ref() {
            texture.draw(
                ctx,
                self.offset,
                74.0 - texture.height(),
                Default::default(),
            );
        }
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
