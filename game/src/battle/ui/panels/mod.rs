use crate::{battle::pokemon::BattlePartyUnknown, input::{pressed, Control}, pokedex::pokemon::instance::PokemonInstance, tetra::Context, util::{Entity, Reset}};

use crate::battle::{
    ui::panels::{
        battle::BattleOptions,
        fight::FightPanel,
    },
};

pub mod moves;
pub mod move_info;
pub mod target;

pub mod battle;
pub mod fight;

// pub mod level_up;

pub struct BattlePanel {

    pub alive: bool,

    pub active: BattlePanels,

    pub battle: BattleOptions,
    pub fight: FightPanel,

}

pub enum BattlePanels {
    Main,
    Fight,
}

impl Default for BattlePanels {
    fn default() -> Self {
        Self::Main
    }
}

impl BattlePanel {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            alive: false,
            active: BattlePanels::default(),
            battle: BattleOptions::new(ctx),
            fight: FightPanel::new(ctx),
        }
    }

    pub fn user(&mut self, /*last_move: &mut Option<(usize, usize)>,*/ instance: &PokemonInstance) {
        self.battle.setup(instance);
        self.fight.user(instance);
        // let last_move = last_move.take().unwrap_or_default();
        self.fight.moves.cursor = 0; // self.fight.moves.cursor = last_move.0;
        self.fight.targets.cursor = 0; // self.fight.targets.cursor = last_move.1;
        self.spawn();
    }

    pub fn target(&mut self, targets: &BattlePartyUnknown) {
        self.fight.target(targets);
    }

    pub fn input(&mut self, ctx: &Context, pokemon: &PokemonInstance) -> Option<BattlePanels> {
        if self.alive {
            match self.active {
                BattlePanels::Main => {
                    self.battle.input(ctx);
                    if pressed(ctx, Control::A) { Some(BattlePanels::Main) } else { None }
                }
                BattlePanels::Fight => {    
                    if pressed(ctx, Control::B) {
                        self.active = BattlePanels::Main;
                    }
                    if self.fight.input(ctx, pokemon) { Some(BattlePanels::Fight) } else { None }
                }
            }
        } else {
            None
        }
    }

	pub fn draw(&self, ctx: &mut Context) {
		if self.alive {
            match self.active {
                BattlePanels::Main => self.battle.draw(ctx),
                BattlePanels::Fight => self.fight.draw(ctx),
            }
		}
	}

}

impl Entity for BattlePanel {
    fn spawn(&mut self) {
        self.alive = true;
        self.active = BattlePanels::default();
        self.fight.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}