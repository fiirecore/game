use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::battle::battle_info::BattleType;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::battle::battle::Battle;
use crate::battle::transitions::battle_transition_traits::BattleOpener;
use crate::battle::transitions::battle_transition_traits::BattleTransitionManager;
use crate::battle::transitions::openers::trainer_battle_opener::TrainerBattleOpener;
use crate::battle::transitions::openers::wild_battle_opener::WildBattleOpener;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

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
        self.battle_introduction_manager.load_introductions();
    }

    pub fn render_below_panel(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer, battle: &Battle) {
        self.battle_introduction_manager.render_with_offset(ctx, g, battle, self.offset());
        self.openers[self.current_opener_id].render_below_panel(ctx, g, tr);
    }

    pub fn offset(&self) -> u16 {
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

    fn reset(&mut self) {
        self.openers[self.current_opener_id].reset();
        self.battle_introduction_manager.reset();
    }

}

impl Ticking for BattleOpenerManager {
    fn update(&mut self, context: &mut GameContext) {
        if self.is_alive() {
            if self.openers[self.current_opener_id].is_alive() {
                if self.openers[self.current_opener_id].is_finished() {
                    self.openers[self.current_opener_id].despawn();
                    self.battle_introduction_manager.spawn();
                    self.battle_introduction_manager.on_start(context);
                } else {
                    self.openers[self.current_opener_id].update(context);
                }
            } else if self.battle_introduction_manager.is_alive() {
                self.battle_introduction_manager.update(context);
            }
        }
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        if self.is_alive() {
            if self.battle_introduction_manager.is_alive() {
                self.battle_introduction_manager.render(ctx, g, tr);
            } else {
                self.openers[self.current_opener_id].render(ctx, g, tr);
            }
            
        }
        
    }
}

impl BattleTransitionManager for BattleOpenerManager {

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

impl Loadable for BattleOpenerManager {

    fn load(&mut self) {
        self.openers[self.current_opener_id].load();
    }

    fn on_start(&mut self, context: &mut GameContext) {
        self.openers[self.current_opener_id].on_start(context);
    }
}
