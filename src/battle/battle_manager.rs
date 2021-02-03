use macroquad::prelude::info;
use crate::pokemon::pokedex::Pokedex;
use crate::util::Reset;
use crate::util::battle_data::BattleData;
use crate::util::{Update, Render};
use crate::gui::battle::battle_gui::BattleGui;
use crate::util::Completable;
use crate::util::Load;
use crate::io::data::player::PlayerData;

use super::battle::Battle;

use crate::entity::Entity;

use super::transitions::managers::battle_closer_manager::BattleCloserManager;
use super::transitions::managers::battle_screen_transition_manager::BattleScreenTransitionManager;
use super::transitions::managers::battle_opener_manager::BattleOpenerManager;

pub struct BattleManager {	
	
	pub current_battle: Battle,
	pub battle_data: BattleData,
	
	battle_screen_transition_manager: BattleScreenTransitionManager,
	battle_opener_manager: BattleOpenerManager,
	battle_closer_manager: BattleCloserManager,

	pub battle_gui: BattleGui,

	pub finished: bool,
	
}

impl BattleManager {
	
	pub fn new() -> BattleManager {
		
		BattleManager {

			battle_screen_transition_manager: BattleScreenTransitionManager::new(),
			battle_opener_manager: BattleOpenerManager::new(),
			battle_closer_manager: BattleCloserManager::default(),
		
			current_battle: Battle::default(),
			battle_data: BattleData::default(),

			battle_gui: BattleGui::new(),

			finished: false,

		}
		
	}

	pub fn world_active(&self) -> bool {
		return self.battle_screen_transition_manager.is_alive() || self.battle_closer_manager.world_active();
	}

}

impl Load for BattleManager {

	fn load(&mut self) {

		self.battle_screen_transition_manager.load();
		self.battle_screen_transition_manager.load_transitions();
		self.battle_opener_manager.load_openers();
		self.battle_closer_manager.load_closers();

		self.battle_gui.load();

	}

}

impl Completable for BattleManager {

	fn is_finished(&self) -> bool {
		self.finished
	}
	
}

impl Reset for BattleManager {

	fn reset(&mut self) {
		self.battle_gui.despawn();
		self.battle_gui.spawn();		
		self.battle_screen_transition_manager.spawn();
	}
	
}

impl BattleManager {

	pub fn on_start(&mut self, pokedex: &Pokedex, player_data: &PlayerData) { // add battle type parameter
		self.finished = false;
		self.battle_data = crate::util::battle_data::BATTLE_DATA.lock().take().unwrap();
		self.create_battle(player_data, pokedex);
		self.reset();
		self.battle_screen_transition_manager.on_start(&self.battle_data);
	}

	pub fn create_battle(&mut self, player_data: &PlayerData, pokedex: &Pokedex) {
		let battle = Battle::new(self.battle_data.battle_type, pokedex, &player_data.party, &self.battle_data.party);
		info!("Loading Battle: {}", battle);
		self.current_battle = battle;
		self.current_battle.load();
		self.battle_gui.on_battle_start(&self.current_battle);
	}

	pub fn update(&mut self, delta: f32, player_data: &mut PlayerData) {
		
		if self.battle_screen_transition_manager.is_alive() {
			if self.battle_screen_transition_manager.is_finished() {
				self.battle_screen_transition_manager.despawn();
				self.battle_opener_manager.spawn_type(self.battle_data.battle_type);
				self.battle_opener_manager.on_start();
				self.battle_opener_manager.battle_introduction_manager.setup_text(&self.current_battle, self.battle_data.trainer_data.as_ref());
			} else {
				self.battle_screen_transition_manager.update(delta);
			}
		} else if self.battle_opener_manager.is_alive() {
			if self.battle_opener_manager.is_finished() {
				self.battle_opener_manager.despawn();
				self.battle_gui.player_panel.start();
			} else {
				self.battle_opener_manager.update(delta);
				self.battle_opener_manager.battle_introduction_manager.update_gui(&mut self.battle_gui, delta);
				//self.battle_gui.opener_update(context);
			}
		} else if self.battle_closer_manager.is_alive() {
			if self.battle_closer_manager.is_finished() {
				// self.battle_closer_manager.update_player(player_data);
				self.battle_closer_manager.despawn();
				self.current_battle.update_data(player_data);
				self.finished = true;
			} else {
				self.battle_closer_manager.update(delta);
			}
		} else /*if !self.current_battle.is_finished()*/ {
			self.current_battle.update(delta, &mut self.battle_gui, &mut self.battle_closer_manager);
			self.battle_gui.update(delta);
		}

	}	

    pub fn render(&self) {

		if self.battle_screen_transition_manager.is_alive() {
			self.battle_screen_transition_manager.render();
		} else if self.battle_opener_manager.is_alive() {
			self.battle_gui.render_background(self.battle_opener_manager.offset());
			self.battle_opener_manager.render_below_panel(&self.current_battle);
			self.battle_gui.render();
			self.battle_gui.render_panel();
			self.battle_opener_manager.render();
		} else if self.battle_closer_manager.is_alive() {
			if !self.world_active() {
				self.battle_gui.render_background(0.0);
				self.current_battle.render(0.0, self.battle_gui.player_bounce.pokemon_offset());
				self.battle_gui.render();
				self.battle_gui.render_panel();
			}
			self.battle_closer_manager.render();
		} else {
			self.battle_gui.render_background(0.0);
			self.current_battle.render(0.0, self.battle_gui.player_bounce.pokemon_offset());
			self.battle_gui.render();
			self.battle_gui.render_panel();
		}
	}
	
	pub fn input(&mut self, delta: f32) {

		if cfg!(debug_assertions) {

			if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F1) {
				//self.battle_closer_manager.spawn() // exit shortcut
				self.finished = true;
				self.current_battle.update_data(&mut std::ops::DerefMut::deref_mut(&mut macroquad::prelude::collections::storage::get_mut::<PlayerData>().unwrap()))
			}

		}

		if !self.battle_screen_transition_manager.is_alive() {	
			if self.battle_opener_manager.is_alive() {
				self.battle_opener_manager.battle_introduction_manager.input(delta);
			} else if self.battle_closer_manager.is_alive() {
				//self.battle_closer_manager.input(context);
			} else {
				self.battle_gui.input(delta, &mut self.current_battle);
			}
		}
	}
	
}

