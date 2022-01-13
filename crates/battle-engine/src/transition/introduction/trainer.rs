use core::ops::Deref;
use pokedex::{
    engine::{utils::HashMap, EngineContext, text::{TextColor, MessageState}},
    item::Item,
    moves::Move,
    pokemon::Pokemon,
};

use pokedex::{
    engine::{
        graphics::Texture,
        text::MessagePage,
        utils::{Completable, Reset},
        Context,
    },
    PokedexClientData,
};

use crate::{
    context::BattleGuiData,
    ui::{view::{ActivePokemonRenderer, GuiLocalPlayer, GuiRemotePlayer}, text::BattleText},
};

use super::{basic::BasicBattleIntroduction, BattleIntroduction};

pub struct TrainerBattleIntroduction {
    introduction: BasicBattleIntroduction,

    texture: Option<Texture>,
    offset: f32,
    leaving: bool,
}

impl TrainerBattleIntroduction {
    const FINAL_TRAINER_OFFSET: f32 = 126.0;

    pub fn new(ctx: &BattleGuiData) -> Self {
        Self {
            introduction: BasicBattleIntroduction::new(ctx),
            texture: None,
            offset: 0.0,
            leaving: false,
        }
    }
}

impl<ID, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>
    BattleIntroduction<ID, P, M, I> for TrainerBattleIntroduction
{
    fn spawn(
        &mut self,
        ctx: &PokedexClientData,
        local: &GuiLocalPlayer<ID, P, M, I>,
        opponents: &HashMap<ID, GuiRemotePlayer<ID, P>>,
        text: &mut BattleText,
    ) {

        if let Some(opponent) = opponents.values().next() {
            if let Some(id) = opponent.trainer.as_ref() {
                self.texture = ctx.trainer_group_textures.get(id).cloned();
            }
            let text = text.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
            if let Some(name) = &opponent.player.name {
                text.pages.push(MessagePage {
                    lines: vec![name.to_owned(), "would like to battle!".to_owned()],
                    wait: None,
                    color: TextColor::WHITE,
                });
    
                text.pages.push(MessagePage {
                    lines: vec![
                        format!("{} sent", name),
                        format!(
                            "out {}",
                            BasicBattleIntroduction::concatenate(&opponent.player)
                        ),
                    ],
                    wait: Some(0.5),
                    color: TextColor::WHITE,
                });
            } else {
                text.pages.push(MessagePage {
                    lines: vec![String::from("No trainer data found!")],
                    wait: None,
                    color: TextColor::WHITE,
                });
            }
        }

        self.introduction.common_setup(text, local);
    }

    fn update(
        &mut self,
        ctx: &mut Context,
        eng: &mut EngineContext,
        delta: f32,
        player: &mut GuiLocalPlayer<ID, P, M, I>,
        opponent: &mut GuiRemotePlayer<ID, P>,
        text: &mut BattleText,
    ) {
        self.introduction
            .update(ctx, eng, delta, player, opponent, text);
        if text.waiting() && text.page() >= text.pages().map(|s| s - 2) {
            self.leaving = true;
        }
        if self.leaving && self.offset < Self::FINAL_TRAINER_OFFSET {
            self.offset += 300.0 * delta;
        }
    }

    fn draw(
        &self,
        ctx: &mut Context,
        eng: &EngineContext,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    ) {
        if self.offset < Self::FINAL_TRAINER_OFFSET {
            if let Some(texture) = &self.texture {
                texture.draw(
                    ctx,
                    144.0 + self.offset,
                    74.0 - texture.height(),
                    Default::default(),
                );
            }
        } else {
            self.introduction.draw_opponent(ctx, eng, opponent);
        }
        self.introduction.draw_player(ctx, player);
    }
}

impl Completable for TrainerBattleIntroduction {
    fn finished(&self) -> bool {
        self.introduction.finished()
    }
}

impl Reset for TrainerBattleIntroduction {
    fn reset(&mut self) {
        self.introduction.reset();
        self.offset = 0.0;
        self.leaving = false;
    }
}
