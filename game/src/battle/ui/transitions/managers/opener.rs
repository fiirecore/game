use crate::{battle::pokemon::gui::ActiveRenderer, tetra::Context};

use crate::battle::{
    Battle,
    BattleType,
    state::TransitionState,
    ui::transitions::{
        BattleOpener, 
        openers::{
            Openers,
            WildBattleOpener,
            TrainerBattleOpener,
        },
    }
};

pub struct BattleOpenerManager {
    pub state: TransitionState,
    current: Openers,

    wild: WildBattleOpener,
    trainer: TrainerBattleOpener,
}

impl BattleOpenerManager {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            state: TransitionState::default(),
            current: Openers::default(),

            wild: WildBattleOpener::new(ctx),
            trainer: TrainerBattleOpener::new(ctx),
        }
    }

    pub fn begin(&mut self, battle: &Battle) {
        self.state = TransitionState::Run;
        self.current = match battle.data.battle_type {
            BattleType::Wild => Openers::Wild,
            BattleType::Trainer => Openers::Trainer,
            BattleType::GymLeader => Openers::Trainer,
        };
        let current = self.get_mut();
        current.reset();
        current.spawn(battle);
    }
    
    pub fn end(&mut self) {
        self.state = TransitionState::Begin;
    }

    pub fn update(&mut self, delta: f32) {
        let current = self.get_mut();
        current.update(delta);
        if current.finished() {
            self.state = TransitionState::End;
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