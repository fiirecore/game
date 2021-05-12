use game::util::{
    Reset,
    Completable,
    battle::BattleType,
};

use crate::{
    Battle,
    ui::transitions::{
        BattleOpener, 
        openers::{
            Openers,
            trainer::TrainerBattleOpener,
            wild::WildBattleOpener,
        },
    }
};

#[derive(Default)]
pub struct BattleOpenerManager {

    alive: bool,

    pub(crate) current: Openers,
    wild: WildBattleOpener,
    trainer: TrainerBattleOpener,
}

impl BattleOpenerManager {
    
    pub fn despawn(&mut self) -> &Openers {
        self.alive = false;
        self.reset();
        &self.current
    }
    
    pub fn is_alive(&self) -> bool {
        self.alive
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

impl BattleOpener for BattleOpenerManager {

    fn spawn(&mut self, battle: &Battle) {
        self.current = match battle.data.battle_type {
            BattleType::Wild => Openers::Wild,
            BattleType::Trainer => Openers::Trainer,
            BattleType::GymLeader => Openers::Trainer,
        };
        self.alive = true;
        self.reset();
        self.get_mut().spawn(battle);
    }

    fn update(&mut self, delta: f32) {
        self.get_mut().update(delta);
    }

    fn offset(&self) -> f32 {
        self.get().offset()
    }

    fn render_below_panel(&self, battle: &Battle) {
        self.get().render_below_panel(battle);
    }

    fn render(&self) {
        self.get().render();
    }
}

impl Reset for BattleOpenerManager {
    fn reset(&mut self) {
        self.get_mut().reset();
    }
}
impl Completable for BattleOpenerManager {
    fn is_finished(&self) -> bool {
        self.get().is_finished()
    }
}