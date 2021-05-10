use game::{
    util::Entity,
    pokedex::pokemon::instance::PokemonInstance,
    input::{pressed, Control},
    gui::{
        party::PartyGui,
        bag::BagGui,
    }
};

use crate::{
    Battle,
    gui::panels::{
        battle::BattleOptions,
        fight::FightPanel,
    }
};

pub mod moves;
pub mod move_info;
pub mod target;

pub mod battle;
pub mod fight;

pub mod level_up;

pub struct BattlePanel {

    alive: bool,

    active: BattlePanels,

    battle: BattleOptions,
    pub fight: FightPanel,

}

enum BattlePanels {
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

    pub fn update_text(&mut self, instance: &PokemonInstance, targets: &Box<[crate::pokemon::ActivePokemon]>) {
        self.battle.update_text(instance);
        self.fight.update_gui(instance, targets);
    }

    pub fn input(&mut self, battle: &Battle, closer: &mut crate::manager::BattleCloserManager, party_gui: &mut PartyGui, bag_gui: &mut BagGui) -> bool {
        if self.alive {
            match self.active {
                BattlePanels::Main => {
                    self.battle.input();
                    if pressed(Control::A) {
                        match self.battle.cursor {
                            0 => {
                                self.active = BattlePanels::Fight;
                            },
                            1 => {
                                bag_gui.spawn(false);
                            },
                            2 => {
                                super::battle_party_gui(party_gui, &battle.player, true);
                            },
                            3 => {
                                if battle.battle_type == game::util::battle::BattleType::Wild {
                                    closer.spawn_closer(battle);
                                }
                            },
                            _ => {}
                        }
                    }
                    false
                }
                BattlePanels::Fight => {    
                    if pressed(Control::B) {
                        self.active = BattlePanels::Main;
                    }
                    true
                }
            }
        } else {
            false
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