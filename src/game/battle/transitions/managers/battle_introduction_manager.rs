use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::game::battle::battle::Battle;
use crate::game::battle::transitions::battle_transition_traits::BattleIntroduction;
use crate::game::battle::transitions::battle_transition_traits::BattleTransitionManager;
use crate::game::battle::transitions::introductions::basic_battle_introduction::BasicBattleIntroduction;
use crate::gui::battle::battle_gui::BattleGui;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

pub struct BattleIntroductionManager {

    alive: bool,
    
    introductions: Vec<Box<dyn BattleIntroduction>>,
    current_introduction_index: usize,

}

impl BattleIntroductionManager {

    pub fn new() -> Self {

        Self {

            alive: false,
            
            introductions: Vec::new(),
            current_introduction_index: 0,

        }

    }

    pub fn load_introductions(&mut self) {
        self.introductions.push(Box::new(BasicBattleIntroduction::new(0, 113)));
        for introduction in &mut self.introductions {
            introduction.load();
        }
    }

    pub fn input(&mut self, context: &mut GameContext) {
		if self.is_alive() {
            self.introductions[self.current_introduction_index].input(context);
        }
    }
    
    pub fn setup_text(&mut self, battle: &Battle) {
        self.introductions[self.current_introduction_index].setup_text(battle);
    }

    pub fn render_with_offset(&self, ctx: &mut Context, g: &mut GlGraphics, battle: &Battle, offset: u16) {
        self.introductions[self.current_introduction_index].render_offset(ctx, g, battle, offset);
    }

    pub fn update_gui(&mut self, battle_gui: &mut BattleGui) {
        self.introductions[self.current_introduction_index].update_gui(battle_gui);
    }

}


impl Ticking for BattleIntroductionManager {

    fn update(&mut self, context: &mut GameContext) {
        self.introductions[self.current_introduction_index].update(context);     
	}

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut crate::engine::text::TextRenderer) {
        self.introductions[self.current_introduction_index].render(ctx, g, tr);
	}
}

impl BattleTransitionManager for BattleIntroductionManager {

}

impl Completable for BattleIntroductionManager {

    fn is_finished(&self) -> bool {
        return self.introductions[self.current_introduction_index].is_finished();
    }
}

impl Entity for BattleIntroductionManager {

    fn spawn(&mut self) {
        self.alive = true;
        self.introductions[self.current_introduction_index].spawn();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.introductions[self.current_introduction_index as usize].despawn();
    }

    fn is_alive(&self) -> bool {
        return self.introductions[self.current_introduction_index as usize].is_alive();
    }
}

impl Loadable for BattleIntroductionManager {

    fn load(&mut self) {
        self.introductions[self.current_introduction_index].load();
    }

    fn on_start(&mut self, context: &mut GameContext) {
        self.introductions[self.current_introduction_index].on_start(context);
    }
}