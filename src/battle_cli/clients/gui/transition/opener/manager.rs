use crate::{
    tetra::Context,
    pokedex::trainer::TrainerData,
};

use battle::data::BattleType;

use crate::battle_cli::clients::gui::{transition::TransitionState, ui::view::ActiveRenderer};

use super::{
    Openers,
    BattleOpener,
    WildBattleOpener,
    TrainerBattleOpener,
};

pub struct BattleOpenerManager {
    current: Openers,

    wild: WildBattleOpener,
    trainer: TrainerBattleOpener,
}

impl BattleOpenerManager {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            current: Openers::default(),

            wild: WildBattleOpener::new(ctx),
            trainer: TrainerBattleOpener::new(ctx),
        }
    }

    pub fn begin(&mut self, state: &mut TransitionState, battle_type: BattleType, opponent: Option<&TrainerData>) {
        *state = TransitionState::Run;
        self.current = match battle_type {
            BattleType::Wild => Openers::Wild,
            BattleType::Trainer => Openers::Trainer,
            BattleType::GymLeader => Openers::Trainer,
        };
        let current = self.get_mut();
        current.reset();
        current.spawn(opponent);
    }
    
    // pub fn end(&mut self, state: &mut TransitionState) {
    //     *state = TransitionState::Begin;
    // }

    pub fn update(&mut self, state: &mut TransitionState, delta: f32) {
        let current = self.get_mut();
        current.update(delta);
        if current.finished() {
            *state = TransitionState::End;
        }
    }

    pub fn draw_below_panel(&self, ctx: &mut Context, player: &ActiveRenderer, opponent: &ActiveRenderer) {
        self.get().draw_below_panel(ctx, player, opponent);
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.get().draw(ctx);
    }

    pub fn offset(&self) -> f32 {
        self.get().offset()
    }

    fn get(&self) -> &dyn BattleOpener {
        match self.current {
            Openers::Wild => &self.wild,
            Openers::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleOpener {
        match self.current {
            Openers::Wild => &mut self.wild,
            Openers::Trainer => &mut self.trainer,
        }
    }

}