use core::ops::Deref;
use pokengine::{
    engine::{graphics::Draw, App, Plugins},
    pokedex::{item::Item, moves::Move, pokemon::Pokemon},
    PokedexClientData,
};

use battle::data::BattleType;

use crate::{
    context::BattleGuiData,
    players::{GuiLocalPlayer, GuiRemotePlayers},
    ui::{pokemon::PokemonRenderer, text::BattleMessageState},
};

use super::TransitionState;

mod basic;
mod trainer;

pub use basic::*;
pub use trainer::*;

pub enum Introductions {
    Basic,
    Trainer,
}

impl Default for Introductions {
    fn default() -> Self {
        Self::Basic
    }
}

pub(crate) trait BattleIntroduction<ID> {
    fn spawn(
        &mut self,
        ctx: &PokedexClientData,
        local: &GuiLocalPlayer<ID, P, M, I>,
        opponents: &GuiRemotePlayers<ID, P>,
        text: &mut Option<BattleMessageState>,
    );

    fn update(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        pokemon: &mut PokemonRenderer<D>,
        local: &mut GuiLocalPlayer<ID, P, M, I>,
        remotes: &mut GuiRemotePlayers<ID, P>,
        text: &mut Option<BattleMessageState>,
    );

    fn draw(
        &self,
        draw: &mut Draw,
        pokemon: &mut PokemonRenderer<D>,
        local: Option<&GuiLocalPlayer<ID, P, M, I>>,
        remote: &GuiRemotePlayers<ID, P>,
    );

    fn reset(&mut self);

    fn finished(&self) -> bool;
}

pub struct BattleIntroductionManager {
    current: Introductions,

    basic: BasicBattleIntroduction,
    trainer: TrainerBattleIntroduction,
    accumulator: f32,
}

impl BattleIntroductionManager {
    pub fn new(ctx: &BattleGuiData) -> Self {
        Self {
            current: Introductions::default(),

            basic: BasicBattleIntroduction::new(ctx),
            trainer: TrainerBattleIntroduction::new(ctx),
            accumulator: 0.0,
        }
    }

    // pub fn update_with_state<ID, P, M, I>(&mut self, dex: &PokedexClientData, text: &mut MessageBox, delta: f32, local: &GuiLocalPlayer<ID, P, M, I>, remotes: &HashMap<ID, GuiRemotePlayer<ID, P>>, state: &TransitionState) {
    //     match state {
    //         TransitionState::Begin => {
    //             self.begin(
    //                 dex,
    //                 state,
    //                 local,
    //                 self.remotes.values().next().unwrap(),
    //                 &mut self.gui.text,
    //             );
    //             TransitionResult::Rerun
    //         }
    //         TransitionState::Run => {
    //             self.update(
    //                 state,
    //                 ctx,
    //                 delta,
    //                 local,
    //                 self.remotes.values_mut().next().unwrap(),
    //                 &mut self.gui.text,
    //             );
    //             if self.gui.text.page() > 0
    //                 && !self.gui.trainer.ending()
    //                 && !matches!(local.data.type_, BattleType::Wild)
    //             {
    //                 self.gui.trainer.end();
    //             }
    //         }
    //         TransitionState::End => {
    //             self.end(&mut self.gui.text);
    //             TransitionResult::Next
    //         }
    //     }
    // }

    pub fn begin<ID>(
        &mut self,
        ctx: &PokedexClientData,
        state: &mut TransitionState,
        local: &GuiLocalPlayer<ID, P, M, I>,
        opponents: &GuiRemotePlayers<ID, P>,
        text: &mut Option<BattleMessageState>,
    ) {
        *state = TransitionState::Run;
        match local.data.type_ {
            BattleType::Wild => self.current = Introductions::Basic,
            _ => self.current = Introductions::Trainer,
        }
        self.accumulator = 0.0;
        let current = self.get_mut::<ID, D, P, M, I>();
        current.reset();
        current.spawn(ctx, local, opponents, text);
    }

    pub fn update<ID>(
        &mut self,
        state: &mut TransitionState,
        app: &mut App,
        plugins: &mut Plugins,
        local: &mut GuiLocalPlayer<ID, P, M, I>,
        remotes: &mut GuiRemotePlayers<ID, P>,
        text: &mut Option<BattleMessageState>,
    ) {
        self.accumulator += app.timer.delta_f32();
        let finished = self.accumulator > 5.0;
        let current = self.get_mut::<ID, D, P, M, I>();
        current.update(app, plugins, pokemon, local, remotes, text);
        if current.finished() || finished {
            *state = TransitionState::End;
        }
    }

    pub fn draw<ID>(
        &self,
        draw: &mut Draw,
        pokemon: &mut PokemonRenderer<D>,
        player: Option<&GuiLocalPlayer<ID, P, M, I>>,
        opponent: &GuiRemotePlayers<ID, P>,
    ) {
        self.get::<ID, D, P, M, I>()
            .draw(draw, pokemon, player, opponent);
    }

    fn get<ID>(&self) -> &dyn BattleIntroduction<ID, D, P, M, I> {
        match self.current {
            Introductions::Basic => &self.basic,
            Introductions::Trainer => &self.trainer,
        }
    }

    fn get_mut<ID>(&mut self) -> &mut dyn BattleIntroduction<ID, D, P, M, I> {
        match self.current {
            Introductions::Basic => &mut self.basic,
            Introductions::Trainer => &mut self.trainer,
        }
    }
}
