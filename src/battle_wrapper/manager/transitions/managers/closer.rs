use battlecli::battle::data::BattleType;
use firecore_battle_engine::ui::text::BattleText;

use crate::pokengine::PokedexClientData;

use crate::{
    battle_wrapper::TransitionState,
    engine::{Context, EngineContext},
};

use worldcli::battle::*;

use crate::battle_wrapper::manager::transitions::{
    closers::{Closers, TrainerBattleCloser, WildBattleCloser},
    BattleCloser,
};

#[derive(Default)]
pub struct BattleCloserManager {
    pub state: TransitionState,
    current: Closers,

    wild: WildBattleCloser,
    trainer: TrainerBattleCloser,
}

impl BattleCloserManager {
    pub fn begin<'d>(
        &mut self,
        ctx: &PokedexClientData,
        battle_type: BattleType,
        player: &BattleId,
        player_name: &str,
        winner: Option<&BattleId>,
        trainer_entry: Option<&BattleTrainerEntry>,
        text: &mut BattleText,
    ) {
        self.state = TransitionState::Run;
        match battle_type {
            BattleType::Wild => self.current = Closers::Wild,
            _ => self.current = Closers::Trainer,
        }
        let current = self.get_mut();
        current.reset();
        current.spawn(ctx, player, player_name, winner, trainer_entry, text);
    }

    pub fn end(&mut self) {
        self.state = TransitionState::Begin;
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        eng: &mut EngineContext,
        delta: f32,
        text: &mut BattleText,
    ) {
        let current = self.get_mut();
        current.update(ctx, eng, delta, text);
        if current.finished() {
            self.state = TransitionState::End;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.get().draw(ctx);
    }

    pub fn draw_battle(&self, ctx: &mut Context) {
        self.get().draw_battle(ctx);
    }

    pub fn world_active(&self) -> bool {
        matches!(self.state, TransitionState::Run) && self.get().world_active()
    }

    fn get(&self) -> &dyn BattleCloser {
        match self.current {
            Closers::Wild => &self.wild,
            Closers::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleCloser {
        match self.current {
            Closers::Wild => &mut self.wild,
            Closers::Trainer => &mut self.trainer,
        }
    }
}
