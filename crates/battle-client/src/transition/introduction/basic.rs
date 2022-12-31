use core::ops::Deref;

use pokengine::{
    engine::{
        graphics::{Color, Draw, DrawExt, DrawParams, Texture},
        math::{vec2, Rect},
        text::{MessagePage, MessageState},
        utils::Entity,
        App, Plugins,
    },
    pokedex::{item::Item, moves::Move, pokemon::Pokemon},
    PokedexClientData,
};

use battle::party::PlayerParty;

use crate::{
    context::BattleGuiData,
    players::{GuiLocalPlayer, GuiRemotePlayers},
    trainer::BattleTrainer,
    ui::{
        pokemon::{PokemonRenderer, PokemonStatusGui},
        text::BattleMessageState,
    },
    view::BasePokemonView,
};

use super::BattleIntroduction;

pub struct BasicBattleIntroduction {
    offsets: (f32, f32),
}
impl BasicBattleIntroduction {
    const OFFSETS: (f32, f32) = (
        -PokemonStatusGui::BATTLE_OFFSET,
        PokemonStatusGui::BATTLE_OFFSET,
    );

    pub fn new(ctx: &BattleGuiData) -> Self {
        Self {
            player: ctx.player.clone(),
            counter: 0.0,
            offsets: Self::OFFSETS, // opponent, player
        }
    }

    pub(crate) fn common_setup<
        ID,
    >(
        &mut self,
        local: &GuiLocalPlayer<ID, P, M, I>,
        text: &mut Option<BattleMessageState>,
    ) {
        let text = text.get_or_insert_with(MessageState::default);
        text.pages.push(MessagePage {
            lines: vec![format!(
                "Go! {}!",
                Self::concatenate(local.player.active_iter().map(|(.., p)| p.name()))
            )],
            wait: Some(0.5),
            ..Default::default()
        });
    }

    pub(crate) fn draw_local<ID>(
        &self,
        draw: &mut Draw,
        pokemonr: &PokemonRenderer<D>,
        local: Option<&GuiLocalPlayer<ID, P, M, I>>,
    ) {
        if self.counter < Self::PLAYER_DESPAWN {
        } else if let Some(local) = local {
            pokemonr.draw_local(remotes, offset, color)
        }
    }

    fn offsets0(&mut self, delta: f32) {
        if self.offsets.0 != 0.0 {
            self.offsets.0 += delta * 240.0;
            if self.offsets.0 > 0.0 {
                self.offsets.0 = 0.0;
            }
        }
    }

    fn offsets1(&mut self, delta: f32) {
        if self.offsets.1 != 0.0 {
            self.offsets.1 -= delta * 240.0;
            if self.offsets.1 < 0.0 {
                self.offsets.1 = 0.0;
            }
        }
    }
}

impl<ID> BattleIntroduction<ID, D, P, M, I> for BasicBattleIntroduction {
    fn spawn(
        &mut self,
        _: &PokedexClientData,
        local: &GuiLocalPlayer<ID, P, M, I>,
        remotes: &GuiRemotePlayers<ID, P>,
        text: &mut Option<MessageState<[f32; 4]>>,
    ) {
        if let Some(remote) = remotes.players.get_index(remotes.current).map(|(.., o)| o) {}
        self.common_setup(local, text);
    }

    fn update(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        pokemonr: &mut PokemonRenderer<D>,
        local: &mut GuiLocalPlayer<ID, P, M, I>,
        remotes: &mut GuiRemotePlayers<ID, P>,
        text: &mut Option<BattleMessageState>,
    ) {
        let delta = app.timer.delta_f32();

        if text.as_ref().map(|t| t.page() + 1) == text.as_ref().map(|t| t.pages())
            && self.counter < Self::PLAYER_DESPAWN
        {
            self.counter += delta * 180.0;
        }

        // if let Some(active) = remotes
        //     .players
        //     .get_index(remotes.current)
        //     .and_then(|(.., r)| r.renderer.get(0))
        // {
        //     if active.status.alive() {
        //         self.offsets0(delta);
        //     } else if text.as_ref().map(|s| s.waiting).unwrap_or_default()
        //         && text.as_ref().map(|p| p.page()) >= text.as_ref().map(|p| p.pages() - 2)
        //     {
        //         for active in remotes
        //             .players
        //             .get_index_mut(remotes.current)
        //             .into_iter()
        //             .flat_map(|(.., r)| r.renderer.iter_mut())
        //         {
        //             active.status.spawn();
        //         }
        //     }
        // } else {
        //     self.offsets0(delta);
        // }

        // if let Some(active) = local.renderer.get(0) {
        // if pokemonr.spawning() {
        // for active in local.renderer.iter_mut() {
        //     active.pokemon.spawner.update(app, plugins, delta);
        // }
        // } else if active.status.alive() {
        //     self.offsets1(delta);
        // } else if self.counter >= Self::PLAYER_T2 {
        //     for active in local.renderer.iter_mut() {
        //         active.pokemon.spawn();
        //         active.status.spawn();
        //     }
        // }
        // } else {
        //     self.offsets1(delta);
        // }
    }

    fn draw(
        &self,
        draw: &mut Draw,
        pokemonr: &mut PokemonRenderer<D>,
        local: Option<&GuiLocalPlayer<ID, P, M, I>>,
        remotes: &GuiRemotePlayers<ID, P>,
    ) {
        pokemonr.draw_remotes(draw, remotes, Default::default(), Color::WHITE);
        self.draw_local(draw, pokemonr, local);
    }

    fn reset(&mut self) {
        self.counter = 0.0;
        self.offsets = Self::OFFSETS;
    }

    fn finished(&self) -> bool {
        self.counter >= Self::PLAYER_DESPAWN //&& self.offsets.1 == 0.0
    }
}
