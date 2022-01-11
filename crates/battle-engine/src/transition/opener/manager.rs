use core::ops::Deref;
use bevy_ecs::prelude::*;
use pokedex::{engine::utils::HashMap, item::Item, moves::Move, pokemon::Pokemon};

use pokedex::{engine::Context, PokedexClientData};

use battle::data::BattleType;

use crate::{
    context::BattleGuiData,
    ui::view::{ActivePokemonRenderer, GuiLocalPlayer, GuiRemotePlayer},
};

use super::{BattleOpener, Openers, TrainerBattleOpener, WildBattleOpener};

pub struct BattleOpenerManager {
    current: Openers,

    wild: WildBattleOpener,
    trainer: TrainerBattleOpener,
}

impl BattleOpenerManager {
    pub fn new(ctx: &mut Context, gui: &BattleGuiData) -> Self {
        Self {
            current: Openers::default(),

            wild: WildBattleOpener::new(ctx, gui),
            trainer: TrainerBattleOpener::new(gui),
        }
    }

    // pub fn update_with_state<
    //     ID: Eq + Hash,
    //     P: Deref<Target = Pokemon>,
    //     M: Deref<Target = Move>,
    //     I: Deref<Target = Item>,
    // >(
    //     &mut self,
    //     dex: &PokedexClientData,
    //     local: &GuiLocalPlayer<ID, P, M, I>,
    //     remotes: &HashMap<ID, GuiRemotePlayer<ID, P>>,
    //     delta: f32,
    //     state: &mut TransitionState,
    // ) -> TransitionResult {
    //     match state {
    //         TransitionState::Begin => {
    //             self.begin(dex, state, local, remotes);

    //             TransitionResult::Rerun
    //         }
    //         TransitionState::Run => {
    //             self.update::<ID, P, M, I>(state, delta);
    //             TransitionResult::None
    //         }
    //         TransitionState::End => {
    //             self.end::<ID, P, M, I>();
    //             TransitionResult::Next
    //         },
    //     }
    // }

    pub fn begin<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        ctx: &PokedexClientData,
        local: &GuiLocalPlayer<ID, P, M, I>,
        opponents: &HashMap<ID, GuiRemotePlayer<ID, P>>,
    ) {
        self.current = match local.data.type_ {
            BattleType::Wild => Openers::Wild,
            BattleType::Trainer => Openers::Trainer,
            BattleType::GymLeader => Openers::Trainer,
        };
        let current = self.get_mut::<ID, P, M, I>();
        current.reset();
        current.spawn(ctx, local, opponents);
    }

    // pub fn end(&mut self, state: &mut TransitionState) {
    //     *state = TransitionState::Begin;
    // }

    pub fn update<
        ID: Component + Eq + std::hash::Hash + Clone + std::fmt::Debug,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        state: &mut State<crate::state::BattlePlayerState<ID>>,
        delta: f32,
    ) {
        let current = self.get_mut::<ID, P, M, I>();
        current.update(delta);
        if current.finished() {
            state.pop();
        }
    }

    // pub fn end<ID, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>(&mut self) {
    //     let current = self.get_mut::<ID, P, M, I>();
    //     current.end();
    // }

    pub fn draw_below_panel<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &self,
        ctx: &mut Context,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    ) {
        self.get::<ID, P, M, I>()
            .draw_below_panel(ctx, player, opponent);
    }

    pub fn draw<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &self,
        ctx: &mut Context,
    ) {
        self.get::<ID, P, M, I>().draw(ctx);
    }

    pub fn offset<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &self,
    ) -> f32 {
        self.get::<ID, P, M, I>().offset()
    }

    fn get<ID, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>(
        &self,
    ) -> &dyn BattleOpener<ID, P, M, I> {
        match self.current {
            Openers::Wild => &self.wild,
            Openers::Trainer => &self.trainer,
        }
    }

    fn get_mut<ID, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>(
        &mut self,
    ) -> &mut dyn BattleOpener<ID, P, M, I> {
        match self.current {
            Openers::Wild => &mut self.wild,
            Openers::Trainer => &mut self.trainer,
        }
    }
}
