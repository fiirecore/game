use core::ops::Deref;
use pokengine::{
    engine::{
        graphics::{Draw, DrawImages, Texture},
        text::{MessagePage, MessageState},
        App, Plugins,
    },
    pokedex::{item::Item, moves::Move, pokemon::Pokemon},
    PokedexClientData,
};

use crate::{
    context::BattleGuiData,
    players::{GuiLocalPlayer, GuiRemotePlayers},
    ui::{pokemon::PokemonRenderer, text::BattleMessageState},
};

use super::{basic::BasicBattleIntroduction, BattleIntroduction};

pub struct TrainerBattleIntroduction {
    introduction: BasicBattleIntroduction,

    texture: Option<Texture>,
    offset: f32,
    leaving: bool,
}

impl TrainerBattleIntroduction {

    pub fn new(ctx: &BattleGuiData) -> Self {
        Self {
            introduction: BasicBattleIntroduction::new(ctx),
            texture: None,
            offset: 0.0,
            leaving: false,
        }
    }
}

impl<
        ID,
        D: Deref<Target = PokedexClientData>,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > BattleIntroduction<ID, D, P, M, I> for TrainerBattleIntroduction
{
    fn spawn(
        &mut self,
        ctx: &PokedexClientData,
        local: &GuiLocalPlayer<ID, P, M, I>,
        opponents: &GuiRemotePlayers<ID, P>,
        text: &mut Option<BattleMessageState>,
    ) {

        self.introduction.common_setup(local, text);
    }

    fn update(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        pokemon: &mut PokemonRenderer<D>,
        local: &mut GuiLocalPlayer<ID, P, M, I>,
        remotes: &mut GuiRemotePlayers<ID, P>,
        text: &mut Option<BattleMessageState>,
    ) {
        BattleIntroduction::<ID, D, P, M, I>::update(
            &mut self.introduction,
            app,
            plugins,
            pokemon,
            local,
            remotes,
            text,
        );
        let text = text.get_or_insert_with(MessageState::default);
    }

    fn draw(
        &self,
        draw: &mut Draw,
        pokemonr: &mut PokemonRenderer<D>,
        local: Option<&GuiLocalPlayer<ID, P, M, I>>,
        remotes: &GuiRemotePlayers<ID, P>,
    ) {
        if self.offset < Self::FINAL_TRAINER_OFFSET {
            if let Some(texture) = &self.texture {
                draw.image(texture)
                    .position(144.0 + self.offset, 74.0 - texture.height());
            }
        } else {
        }
        self.introduction.draw_local(draw, pokemonr, local);
    }

    fn finished(&self) -> bool {
        BattleIntroduction::<ID, D, P, M, I>::finished(&self.introduction)
    }

    fn reset(&mut self) {
        BattleIntroduction::<ID, D, P, M, I>::reset(&mut self.introduction);
        self.offset = 0.0;
        self.leaving = false;
    }
}
