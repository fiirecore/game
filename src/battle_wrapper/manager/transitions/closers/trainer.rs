use crate::pokengine::PokedexClientData;

use crate::engine::{
    graphics::Texture,
    text::MessagePage,
    utils::{Completable, Reset, WIDTH},
    Context, EngineContext,
};

use firecore_battle_engine::ui::text::BattleText;
use worldcli::{battle::*, engine::text::{TextColor, MessageState}};

use crate::battle_wrapper::manager::transitions::BattleCloser;

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
        text: &mut BattleText,
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

                        let text = text.state.get_or_insert_with(|| MessageState::new(1, Default::default()));

                        for message in trainer.defeat.iter() {
                            text.pages.push(message.clone());
                        }

                        text.pages.push(MessagePage {
                            lines: vec![
                                format!("{} got ${}", player_name, trainer.worth),
                                String::from("for winning!"),
                            ],
                            wait: None,
                            color: TextColor::WHITE,
                        });
                    }
                }
                false => {}
            },
            None => {}
        }
    }

    fn update(
        &mut self,
        ctx: &mut Context,
        eng: &mut EngineContext,
        delta: f32,
        text: &mut BattleText,
    ) {
        if text.alive() {
            text.update(ctx, eng, delta);
            if text.page() == Some(1) && self.offset > Self::XPOS {
                self.offset -= 300.0 * delta;
                if self.offset < Self::XPOS {
                    self.offset = Self::XPOS;
                }
            }
        } else {
            self.wild.update(ctx, eng, delta, text);
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
