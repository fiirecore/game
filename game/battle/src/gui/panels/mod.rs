use game::{
    util::Entity,
    pokedex::pokemon::instance::PokemonInstance,
    macroquad::prelude::{Vec2, Texture2D},
    gui::party::PokemonPartyGui,
    graphics::{byte_texture, draw},
};

use crate::{
    Battle,
    gui::{
        text::BattleText,
        panels::{
            battle::BattleOptions,
            fight::FightPanel,
        }
    }
};

pub mod moves;
pub mod battle;
pub mod fight;
pub mod level_up;
pub mod move_info;

pub struct BattlePanel {

    alive: bool,
    
	pos: Vec2,

    background: Texture2D,

    battle: BattleOptions,
    fight: FightPanel,

}

impl BattlePanel {

	pub fn new(pos: Vec2) -> Self {
		Self {

            alive: false,

            pos,

            background: byte_texture(include_bytes!("../../../assets/gui/panel.png")),

            battle: BattleOptions::new(),
            fight: FightPanel::new(pos),
            
        }
	}

    pub fn start(&mut self) {
        self.spawn();
        self.battle.spawn();
    }

    pub fn update_text(&mut self, instance: &PokemonInstance) {
        self.battle.update_text(instance);
        self.fight.update_gui(instance);
    }
    
    pub fn input(&mut self, battle: &mut Battle, text: &mut BattleText, party_gui: &mut PokemonPartyGui) {
        if self.battle.is_alive() {
            self.battle.input(battle, party_gui);
        } else if self.fight.is_alive() {
            self.fight.input(battle, text);
        }
    }

    pub fn update(&mut self) {
        if self.alive {
            if self.battle.is_alive() && self.battle.spawn_fight_panel {
                self.battle.spawn_fight_panel = false;
                self.battle.despawn();
                self.fight.spawn();
            } else if self.fight.is_alive() && self.fight.spawn_battle_panel {
                self.fight.spawn_battle_panel = false;
                self.fight.despawn();
                self.battle.spawn();
            }
        }              
	}

	pub fn render(&self) {
        draw(self.background, self.pos.x, self.pos.y);
		if self.alive {
            self.battle.render();
            self.fight.render();
		}
	}

}

impl Entity for BattlePanel {

	fn spawn(&mut self) {
        self.alive = true;
        // self.battle.spawn();
	}

	fn despawn(&mut self) {
		self.alive = false;
        self.battle.despawn();
        self.fight.despawn();
	}

	fn is_alive(& self) -> bool {
		self.alive
	}

}