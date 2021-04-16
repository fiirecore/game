use game::{
	util::Entity,
	macroquad::prelude::Vec2,
	gui::party::PokemonPartyGui
};

use background::BattleBackground;
use text::BattleText;
use panels::BattlePanel;
use bounce::PlayerBounce;
use pokemon::{
	PokemonGui,
	player::PlayerPokemonGui,
	opponent::OpponentPokemonGui,
};

use crate::Battle;

use self::panels::level_up::LevelUpMovePanel;

pub mod background;
pub mod text;
pub mod pokemon;
pub mod bounce;
pub mod exp_bar;

pub mod panels;

pub struct BattleGui {

	pub background: BattleBackground,
    
	pub panel: BattlePanel,

	pub battle_text: BattleText,

	pub player: PlayerPokemonGui,
	pub opponent: OpponentPokemonGui,

	pub bounce: PlayerBounce,

	pub level_up: LevelUpMovePanel,

}

impl BattleGui {

	pub fn new() -> Self {

		let panel = Vec2::new(0.0, 113.0);

		Self {

			background: BattleBackground::new(),

			panel: BattlePanel::new(panel),

			player: PlayerPokemonGui::new(127.0, 76.0),
			opponent: OpponentPokemonGui::new(14.0, 18.0),

			battle_text: BattleText::new(),

			bounce: PlayerBounce::new(),

			level_up: LevelUpMovePanel::new(panel),

		}

	}

	pub fn despawn(&mut self) {
		self.panel.despawn();
		self.player.despawn();
		self.opponent.despawn();
		self.battle_text.text.despawn();
    }

	pub fn on_battle_start(&mut self, battle: &Battle) {
		// self.player_pokemon_gui.reset();
		// self.opponent_pokemon_gui.reset();
		self.bounce.reset();
		self.update_gui(battle, true, true);
	}

	pub fn update_gui(&mut self, battle: &Battle, pnew: bool, onew: bool) {

		self.panel.update_text(battle.player.active());
		
		self.opponent.update_gui(battle.opponent.active(), onew);
		self.player.update_gui(battle.player.active(), pnew);
	}

	pub fn update(&mut self, delta: f32) {
		self.bounce.update(delta, &mut self.player);
		self.panel.update();
		self.player.update(delta);
		self.opponent.update(delta);
	}

	pub fn input(&mut self, battle: &mut Battle, party_gui: &mut PokemonPartyGui) {
		self.battle_text.text.input();
		self.panel.input(battle, &mut self.battle_text, party_gui);
	}

	pub fn render_background(&self, offset: f32) {
		self.background.render(offset);
	}

	pub fn render_panel(&self) {
		self.panel.render();
		if self.level_up.is_alive() {
			self.level_up.render();
		} else {
			self.battle_text.text.render();
		}		
	}

	pub fn render(&self) {
		self.opponent.render();
		self.player.render();
	}

}