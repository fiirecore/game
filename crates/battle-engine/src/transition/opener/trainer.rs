use core::ops::Deref;
use pokedex::{engine::utils::HashMap, item::Item, moves::Move, pokemon::Pokemon};

use pokedex::{
    engine::{
        graphics::Texture,
        utils::{Completable, Reset},
        Context,
    },
    PokedexClientData,
};

use crate::{
    context::BattleGuiData,
    ui::view::{ActivePokemonRenderer, GuiLocalPlayer, GuiRemotePlayer},
};

use super::{BattleOpener, DefaultBattleOpener};

pub struct TrainerBattleOpener {
    opener: DefaultBattleOpener,
    trainer: Option<Texture>,
}

impl TrainerBattleOpener {
    pub fn new(ctx: &BattleGuiData) -> Self {
        Self {
            opener: DefaultBattleOpener::new(ctx),
            trainer: None,
        }
    }
}

impl<ID, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>
    BattleOpener<ID, P, M, I> for TrainerBattleOpener
{
    fn spawn(
        &mut self,
        ctx: &PokedexClientData,
        _local: &GuiLocalPlayer<ID, P, M, I>,
        opponents: &HashMap<ID, GuiRemotePlayer<ID, P>>,
    ) {
        if let Some(id) = &opponents.values().next().unwrap().npc {
            self.trainer = ctx.npc_group_textures.get(id).cloned();
        }
    }

    fn update(&mut self, delta: f32) {
        self.opener.update(delta);
    }

    fn draw_below_panel(
        &self,
        ctx: &mut Context,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    ) {
        if let Some(texture) = self.trainer.as_ref() {
            texture.draw(
                ctx,
                144.0 - self.opener.offset,
                74.0 - texture.height(),
                Default::default(),
            );
        }
        self.opener.draw_below_panel(ctx, player, opponent);
    }

    fn draw(&self, ctx: &mut Context) {
        self.opener.draw(ctx);
    }

    fn offset(&self) -> f32 {
        self.opener.offset
    }
}

impl Reset for TrainerBattleOpener {
    fn reset(&mut self) {
        self.opener.reset();
        self.trainer = None;
    }
}

impl Completable for TrainerBattleOpener {
    fn finished(&self) -> bool {
        self.opener.finished()
    }
}
