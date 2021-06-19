use crate::{
    pokedex::trainer::{TrainerId, TrainerData},
    gui::TextDisplay,
    tetra::Context,
    battle_glue::BattleTrainerEntry,
};

use battle::data::BattleType;

use crate::battle_cli::{
    clients::gui::transition::TransitionState,
    ui::transitions::{
        BattleCloser,
        closers::{
            Closers,
            WildBattleCloser,
            TrainerBattleCloser,
        }
    }
};

#[derive(Default)]
pub struct BattleCloserManager {
    pub state: TransitionState,
    current: Closers,

    wild: WildBattleCloser,
    trainer: TrainerBattleCloser,
}

impl BattleCloserManager {

    pub fn begin(&mut self, battle_type: BattleType, winner: Option<&TrainerId>, trainer: Option<&TrainerData>, trainer_entry: Option<&BattleTrainerEntry>, text: &mut TextDisplay) {
        self.state = TransitionState::Run;
        match battle_type {
            BattleType::Wild => self.current = Closers::Wild,
            _ => self.current = Closers::Trainer,
        }
        let current = self.get_mut();
        current.reset();
        current.spawn(winner, trainer, trainer_entry, text);
    }

    pub fn end(&mut self) {
        self.state = TransitionState::Begin;
    }

    pub fn update(&mut self, ctx: &mut Context, delta: f32, text: &mut TextDisplay) {
        let current = self.get_mut();
        current.update(ctx, delta, text);
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