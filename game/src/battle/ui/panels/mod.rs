use crate::{
    util::{Entity, Reset},
    pokedex::{
        pokemon::instance::PokemonInstance,
        moves::target::MoveTarget,
    },
    input::{pressed, Control},
    tetra::Context,
};

use crate::battle::{
    pokemon::BattlePartyUnknown,
    ui::panels::{
        battle::BattleOptions,
        fight::FightPanel,
        target::TargetPanel,
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
    pub targets: TargetPanel,

}

pub enum BattlePanels {
    Main,
    Fight,
    Target(MoveTarget),
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
            targets: TargetPanel::new(ctx),
        }
    }

    pub fn user(&mut self, /*last_move: &mut Option<(usize, usize)>,*/ instance: &PokemonInstance) {
        self.battle.setup(instance);
        self.fight.user(instance);
        // let last_move = last_move.take().unwrap_or_default();
        self.fight.moves.cursor = 0; // self.fight.moves.cursor = last_move.0;
        self.targets.cursor = 0; // self.fight.targets.cursor = last_move.1;
        self.spawn();
    }

    pub fn target(&mut self, targets: &BattlePartyUnknown) {
        self.targets.update_names(targets);
    }

    pub fn input(&mut self, ctx: &Context, pokemon: &PokemonInstance) -> Option<BattlePanels> {
        if self.alive {
            match self.active {
                BattlePanels::Main => {
                    self.battle.input(ctx);
                    pressed(ctx, Control::A).then(|| BattlePanels::Main)
                }
                BattlePanels::Fight => {    
                    if pressed(ctx, Control::B) {
                        self.active = BattlePanels::Main;
                    }
                    self.fight.input(ctx, pokemon);
                    pressed(ctx, Control::A).then(|| BattlePanels::Fight)
                }
                BattlePanels::Target(..) => {
                    if pressed(ctx, Control::B) {
                        self.active = BattlePanels::Fight;
                    }
                    self.targets.input(ctx);
                    pressed(ctx, Control::A).then(|| std::mem::take(&mut self.active))
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
                BattlePanels::Target(..) => self.targets.draw(ctx),
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