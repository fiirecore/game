use firecore_pokedex::pokemon::instance::PokemonInstance;
use firecore_util::Entity;
use macroquad::prelude::Vec2;

use crate::battle::gui::battle_text::BattleText;
use crate::battle::Battle;

use self::battle::BattlePanel;
use self::fight::FightPanel;

pub mod battle;
pub mod fight;
pub mod move_info;

pub struct PlayerPanel {

    alive: bool,
    
	pub pos: Vec2,

    pub battle_panel: BattlePanel,
    pub fight_panel: FightPanel,

}

impl PlayerPanel {

	pub fn new(pos: Vec2) -> Self {
		Self {

            alive: false,

            pos,

            battle_panel: BattlePanel::new(pos),
            fight_panel: FightPanel::new(pos),
            
        }
	}

    pub fn start(&mut self) {
        self.spawn();
        self.battle_panel.spawn();
    }

    pub fn update_text(&mut self, instance: &PokemonInstance) {
        self.battle_panel.update_text(instance);
        self.fight_panel.move_panel.update_names(instance);
        self.fight_panel.update_move(instance);
    }
    
    pub fn input(&mut self, delta: f32, battle: &mut Battle, text: &mut BattleText) {
        if self.battle_panel.is_alive() {
            self.battle_panel.input(delta, battle);
        } else if self.fight_panel.is_alive() {
            self.fight_panel.input(battle, text);
        }
    }

    pub fn update(&mut self) {
        if self.alive {
            if self.battle_panel.is_alive() && self.battle_panel.spawn_fight_panel {
                self.battle_panel.spawn_fight_panel = false;
                self.battle_panel.despawn();
                self.fight_panel.spawn();
            } else if self.fight_panel.is_alive() && self.fight_panel.spawn_battle_panel {
                self.fight_panel.spawn_battle_panel = false;
                self.fight_panel.despawn();
                self.battle_panel.spawn();
            }
        }              
	}

	pub fn render(&self) {
		if self.alive {
            self.battle_panel.render();
            self.fight_panel.render();
		}
	}

}

impl Entity for PlayerPanel {

	fn spawn(&mut self) {
        self.alive = true;
        self.battle_panel.spawn();
	}

	fn despawn(&mut self) {
		self.alive = false;
        self.battle_panel.despawn();
        self.fight_panel.despawn();
	}

	fn is_alive(& self) -> bool {
		self.alive
	}

}