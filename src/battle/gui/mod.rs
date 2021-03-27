use firecore_util::Entity;
use macroquad::prelude::Vec2;

use crate::battle::Battle;
use battle_background::BattleBackground;
use battle_text::BattleText;
use panels::PlayerPanel;
use player_bounce::PlayerBounce;
use pokemon::{
	PokemonGui,
	player::PlayerPokemonGui,
	opponent::OpponentPokemonGui,
};

use crate::util::graphics::{Texture, texture::byte_texture};

use self::level_up_move_panel::LevelUpMovePanel;

pub mod battle_background;
pub mod battle_text;
pub mod pokemon;
pub mod player_bounce;
pub mod pokemon_texture;
pub mod exp_bar;

pub mod move_panel;
pub mod panels;
pub mod level_up_move_panel;

pub struct BattleGui {

	pub battle_background: BattleBackground,
    
	panel: Texture,
	pub player_panel: PlayerPanel,

	pub battle_text: BattleText,

	pub player_pokemon_gui: PlayerPokemonGui,
	pub opponent_pokemon_gui: OpponentPokemonGui,

	pub player_bounce: PlayerBounce,

	pub level_up: LevelUpMovePanel,

}

impl BattleGui {

	pub fn new() -> Self {

		let panel = Vec2::new(0.0, 113.0);

		Self {

			battle_background: BattleBackground::new(),

			panel: byte_texture(include_bytes!("../../../build/assets/gui/battle/panel.png")),
			player_panel: PlayerPanel::new(panel),

			player_pokemon_gui: PlayerPokemonGui::new(127.0, 76.0),
			opponent_pokemon_gui: OpponentPokemonGui::new(14.0, 18.0),

			battle_text: BattleText::new(),

			player_bounce: PlayerBounce::new(),

			level_up: LevelUpMovePanel::new(panel),

		}

	}

	pub fn despawn(&mut self) {
		self.player_panel.despawn();
		self.player_pokemon_gui.despawn();
		self.opponent_pokemon_gui.despawn();
		self.battle_text.text.despawn();
    }

	pub fn on_battle_start(&mut self, battle: &Battle) {
		// self.player_pokemon_gui.reset();
		// self.opponent_pokemon_gui.reset();
		self.player_bounce.reset();
		self.update_gui(battle, true, true);
	}

	pub fn update_gui(&mut self, battle: &Battle, pnew: bool, onew: bool) {

		self.player_panel.update_text(battle.player.active());
		
		self.opponent_pokemon_gui.update_gui(battle.opponent.active(), onew);
		self.player_pokemon_gui.update_gui(battle.player.active(), pnew);
	}

	pub fn update(&mut self, delta: f32) {
		self.player_bounce.update(delta, &mut self.player_pokemon_gui);
		self.player_panel.update();
		self.player_pokemon_gui.update(delta);
		self.opponent_pokemon_gui.update(delta);
	}

	pub fn input(&mut self, delta: f32, battle: &mut Battle) {
		self.player_panel.input(delta, battle, &mut self.battle_text);
	}

	pub fn render_background(&self, offset: f32) {
		self.battle_background.render(offset);
	}

	pub fn render_panel(&self) {
		crate::util::graphics::draw(self.panel, self.player_panel.pos.x, self.player_panel.pos.y);
		self.player_panel.render();
		if self.level_up.is_alive() {
			self.level_up.render();
		} else {
			self.battle_text.text.render();
		}		
	}

	pub fn render(&self) {
		self.opponent_pokemon_gui.render();
		self.player_pokemon_gui.render();
	}

}