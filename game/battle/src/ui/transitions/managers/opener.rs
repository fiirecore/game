use crate::{
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

#[derive(Default)]
pub struct BattleOpenerManager {
    pub state: TransitionState,
    current: Openers,

    wild: WildBattleOpener,
    trainer: TrainerBattleOpener,
}

impl BattleOpenerManager {

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
        if current.is_finished() {
            self.state = TransitionState::End;
        }
    }

    pub fn render_below_panel(&self, battle: &Battle) {
        self.get().render_below_panel(battle);
    }

    pub fn render(&self) {
        self.get().render();
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