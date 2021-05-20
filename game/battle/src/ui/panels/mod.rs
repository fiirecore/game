use game::{
    util::Entity,
    pokedex::pokemon::instance::PokemonInstance,
    input::{pressed, Control},
};

use crate::ui::panels::{
    battle::BattleOptions,
    fight::FightPanel,
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

    pub fn new() -> Self {
        Self {
            alive: false,
            active: BattlePanels::default(),
            battle: BattleOptions::new(),
            fight: FightPanel::new(),
        }
    }

    pub fn run(&mut self, last_move: &mut Option<(usize, usize)>, instance: &PokemonInstance, targets: &crate::pokemon::ActivePokemonArray) {
        self.battle.setup(instance);
        self.fight.setup(instance, targets);
        let last_move = last_move.take().unwrap_or_default();
        self.fight.moves.cursor = last_move.0;
        self.fight.targets.cursor = last_move.1;
        self.spawn();
    }

    pub fn input(&mut self, pokemon: &PokemonInstance) -> Option<BattlePanels> {
        if self.alive {
            match self.active {
                BattlePanels::Main => {
                    self.battle.input();
                    if pressed(Control::A) { Some(BattlePanels::Main) } else { None }
                }
                BattlePanels::Fight => {    
                    if pressed(Control::B) {
                        self.active = BattlePanels::Main;
                    }
                    if self.fight.input(pokemon) { Some(BattlePanels::Fight) } else { None }
                }
            }
        } else {
            None
        }
    }

	pub fn render(&self) {
		if self.alive {
            match self.active {
                BattlePanels::Main => self.battle.render(),
                BattlePanels::Fight => self.fight.render(),
            }
		}
	}

}

impl Entity for BattlePanel {
    fn spawn(&mut self) {
        self.alive = true;
        self.active = BattlePanels::default();
        game::util::Reset::reset(&mut self.fight);
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}