use crate::battle::battle_info::BattleType;
use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;
use crate::util::{Update, Render};
use crate::battle::battle::Battle;
use crate::battle::transitions::battle_transition_traits::BattleOpener;
use crate::battle::transitions::battle_transition_traits::BattleTransitionManager;
use crate::battle::transitions::openers::trainer_battle_opener::TrainerBattleOpener;
use crate::battle::transitions::openers::wild_battle_opener::WildBattleOpener;
use crate::util::{Reset, Completable};
use crate::util::Load;

use super::battle_introduction_manager::BattleIntroductionManager;

pub struct BattleOpenerManager {

    alive: bool,

    pub battle_introduction_manager: BattleIntroductionManager,

    pub openers: Vec<Box<dyn BattleOpener>>,
    pub current_opener_id: usize,
}

impl BattleOpenerManager {
    pub fn new() -> Self {
        Self {

            alive: false,

            battle_introduction_manager: BattleIntroductionManager::new(),

            openers: Vec::new(),
            current_opener_id: 0,

        }
    }

    pub fn load_openers(&mut self) {
        self.openers.push(Box::new(WildBattleOpener::new()));
        self.openers.push(Box::new(TrainerBattleOpener::new()));
        for opener in &mut self.openers {
            opener.load();
        }
        // self.battle_introduction_manager.load_introductions();
    }

    pub fn render_below_panel(&self, tr: &TextRenderer, battle: &Battle) {
        self.battle_introduction_manager.render_with_offset(battle, self.offset());
        self.openers[self.current_opener_id].render_below_panel(tr);
    }

    pub fn offset(&self) -> f32 {
        return self.openers[self.current_opener_id].offset();
    }

    pub fn spawn_type(&mut self, battle_type: BattleType) {
        match battle_type {
            BattleType::Wild => {
                self.current_opener_id = 0;
            }
            BattleType::Trainer => {
                self.current_opener_id = 1;
            }
            BattleType::GymLeader => {
                self.current_opener_id = 1;
            }
        }
        self.battle_introduction_manager.spawn_type(self.current_opener_id);
        self.spawn();
    }

}

impl Update for BattleOpenerManager {

    fn update(&mut self, delta: f32) {
        if self.is_alive() {
            if self.openers[self.current_opener_id].is_alive() {
                if self.openers[self.current_opener_id].is_finished() {
                    self.openers[self.current_opener_id].despawn();
                    self.battle_introduction_manager.spawn();
                    self.battle_introduction_manager.on_start();
                } else {
                    self.openers[self.current_opener_id].update(delta);
                }
            } else if self.battle_introduction_manager.is_alive() {
                self.battle_introduction_manager.update(delta);
            }
        }
    }
}

impl Render for BattleOpenerManager {

    fn render(&self, tr: &TextRenderer) {
        if self.is_alive() {
            if self.battle_introduction_manager.is_alive() {
                self.battle_introduction_manager.render(tr);
            } else {
                self.openers[self.current_opener_id].render(tr);
            }
            
        }
        
    }

}

impl BattleTransitionManager for BattleOpenerManager {}

impl Reset for BattleOpenerManager {

    fn reset(&mut self) {
        self.openers[self.current_opener_id].reset();
        self.battle_introduction_manager.reset();
    }

}

impl Completable for BattleOpenerManager {

    fn is_finished(&self) -> bool {
        return self.battle_introduction_manager.is_finished();
    }
    
}

impl Entity for BattleOpenerManager {
    fn spawn(&mut self) {
        self.alive = true;
        self.openers[self.current_opener_id].spawn();
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.openers[self.current_opener_id].despawn();
        self.battle_introduction_manager.despawn();
        self.reset();
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }
}

impl Load for BattleOpenerManager {

    fn load(&mut self) {
        self.openers[self.current_opener_id].load();
    }

    fn on_start(&mut self) {
        self.openers[self.current_opener_id].on_start();
    }
}
