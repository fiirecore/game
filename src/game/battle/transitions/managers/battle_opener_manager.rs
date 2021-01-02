use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::game::battle::transitions::openers::trainer_battle_opener::TrainerBattleOpener;
use crate::game::battle::transitions::openers::wild_battle_opener::WildBattleOpener;
use crate::game::battle::transitions::traits::battle_opener::BattleOpener;
use crate::game::battle::transitions::traits::battle_transition_manager::BattleTransitionManager;
use crate::util::traits::Loadable;

pub struct BattleOpenerManager {
    alive: bool,

    pub openers: Vec<Box<dyn BattleOpener>>,
    pub current_opener_id: usize,
}

impl BattleOpenerManager {
    pub fn new() -> Self {
        Self {
            alive: false,

            openers: Vec::new(),
            current_opener_id: 0,
        }
    }

    pub fn load_openers(&mut self) {
        self.openers.push(Box::new(WildBattleOpener::new()));
        self.openers.push(Box::new(TrainerBattleOpener::new()));
    }

    pub fn render_below_panel(
        &mut self,
        ctx: &mut Context,
        g: &mut GlGraphics,
        tr: &mut TextRenderer,
    ) {
        self.openers[self.current_opener_id].render_below_panel(ctx, g, tr);
    }

    pub fn offset(&self) -> u16 {
        return self.openers[self.current_opener_id].offset();
    }
}

impl Ticking for BattleOpenerManager {
    fn update(&mut self, context: &mut GameContext) {
        self.openers[self.current_opener_id].update(context);
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.openers[self.current_opener_id].render(ctx, g, tr);
    }
}

impl BattleTransitionManager for BattleOpenerManager {
    fn is_finished(&self) -> bool {
        return self.openers[self.current_opener_id].is_finished();
    }
}

impl Entity for BattleOpenerManager {
    fn spawn(&mut self) {
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.openers[self.current_opener_id].despawn();
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }
}

impl Loadable for BattleOpenerManager {

    fn load(&mut self) {

    }

    fn on_start(&mut self, context: &mut GameContext) {
        self.current_opener_id = context.random.rand_range(0..self.openers.len() as u32) as usize;

        self.current_opener_id = 0;

        self.openers[self.current_opener_id].spawn();
        self.openers[self.current_opener_id].on_start(context);
    }
}
