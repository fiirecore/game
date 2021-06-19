use crate::{
    util::{Entity, Completable},
    gui::TextDisplay, 
    tetra::Context,
};

use crate::battle::{
    data::BattleType,
    pokemon::view::{BattlePartyKnown, BattlePartyUnknown},
};

use crate::battle_cli::ui::view::{ActiveRenderer, ActivePokemonParty};

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

pub(crate) trait BattleIntroduction: Completable {

    fn spawn(&mut self, battle_type: BattleType, player: &BattlePartyKnown, opponent: &BattlePartyUnknown, text: &mut TextDisplay);

    fn update(&mut self, ctx: &Context, delta: f32, player: &mut ActivePokemonParty<BattlePartyKnown>, opponent: &mut ActivePokemonParty<BattlePartyUnknown>, text: &mut TextDisplay);

    fn draw(&self, ctx: &mut Context, player: &ActiveRenderer, opponent: &ActiveRenderer);

}

pub struct BattleIntroductionManager {
    current: Introductions,

    basic: BasicBattleIntroduction,
    trainer: TrainerBattleIntroduction,
}

impl BattleIntroductionManager {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            current: Introductions::default(),

            basic: BasicBattleIntroduction::new(ctx),
            trainer: TrainerBattleIntroduction::new(ctx),
        }
    }

    pub fn begin(&mut self, state: &mut TransitionState, battle_type: BattleType, player: &BattlePartyKnown, opponent: &BattlePartyUnknown, text: &mut TextDisplay) {
        *state = TransitionState::Run;
        match battle_type {
            BattleType::Wild => self.current = Introductions::Basic,
            _ => self.current = Introductions::Trainer,
        }
        let current = self.get_mut();
        current.reset();
        current.spawn(battle_type, player, opponent, text);
        text.spawn();
    }

    pub fn end(&mut self, text: &mut TextDisplay) {
        text.clear();
    }

    pub fn update(&mut self, state: &mut TransitionState, ctx: &Context, delta: f32, player: &mut ActivePokemonParty<BattlePartyKnown>, opponent: &mut ActivePokemonParty<BattlePartyUnknown>, text: &mut TextDisplay) {
        let current = self.get_mut();
        current.update(ctx, delta, player, opponent, text);
        if current.finished() {
            *state = TransitionState::End;
        }
    }

    pub fn draw(&self, ctx: &mut Context, player: &ActiveRenderer, opponent: &ActiveRenderer) {
        self.get().draw(ctx, player, opponent);
    }

    fn get(&self) -> &dyn BattleIntroduction {
        match self.current {
            Introductions::Basic => &self.basic,
            Introductions::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleIntroduction {
        match self.current {
            Introductions::Basic => &mut self.basic,
            Introductions::Trainer => &mut self.trainer,
        }
    }

}