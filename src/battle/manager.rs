use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_util::{Entity, Completable, battle::BattleType};
use macroquad::prelude::collections::storage::get_mut;
use crate::data::player::list::PlayerSaves;
use crate::gui::game::party::PokemonPartyGui;
use crate::util::battle_data::BattleData;
use crate::util::battle_data::TrainerData;
use super::battle_party::BattleParty;
use super::gui::BattleGui;
use super::{
	Battle,
	transitions::{
		BattleTransition,
		BattleOpener,
		BattleCloser,
		managers::{
			screen_transition::BattleScreenTransitionManager,
			opener::BattleOpenerManager,
			closer::BattleCloserManager,
		}
	}
};

pub struct BattleManager {
	
	current_battle: Battle,
	
	screen_transition_manager: BattleScreenTransitionManager,
	opener_manager: BattleOpenerManager,
	closer_manager: BattleCloserManager,

	gui: BattleGui,

	trainer: Option<TrainerData>,

	finished: bool,
	
}

impl BattleManager {
	
	pub fn new() -> BattleManager {
		
		BattleManager {

			current_battle: Battle::default(),

			screen_transition_manager: BattleScreenTransitionManager::new(),
			opener_manager: BattleOpenerManager::new(),
			closer_manager: BattleCloserManager::default(),

			gui: BattleGui::new(),

			trainer: None,

			finished: false,

		}
		
	}

	pub fn create_battle(&mut self, player_party: &PokemonParty, opponent_party: &PokemonParty, battle_type: BattleType) {
		let battle = Battle::new(&player_party, opponent_party, battle_type);
		if battle.verify() {
			self.current_battle = battle;
		} else {
			self.finished = true;
		}	
	}

	pub fn on_start(&mut self, player_party: &PokemonParty, data: BattleData) { // add battle type parameter

		self.finished = false;

		// Create the battle

		self.create_battle(player_party, &data.party, data.trainer_data.as_ref().map(|data| data.npc_data.trainer.as_ref().unwrap().battle_type).unwrap_or_default());
		self.trainer = data.trainer_data;

		// Despawn anything from previous battle

		self.gui.despawn();

		// Setup transition and GUI
		
		self.screen_transition_manager.spawn_with_type(self.current_battle.battle_type);
		self.gui.on_battle_start(&self.current_battle);

	}

	pub fn input(&mut self, delta: f32) {

		if crate::debug() {

			if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F1) {
				// exit shortcut
				self.finished = true;
				if let Some(mut saves) = get_mut::<PlayerSaves>() {
					self.current_battle.update_data(saves.get_mut());
				}
			}

		}

		if !self.screen_transition_manager.is_alive() {	
			if self.opener_manager.is_alive() {
				self.opener_manager.battle_introduction_manager.input();
			} else if self.closer_manager.is_alive() {
				//self.closer_manager.input(context);
			} else {
				self.gui.input(delta, &mut self.current_battle);
			}
		}
	}

	pub fn update(&mut self, delta: f32, party_gui: &mut PokemonPartyGui) {
		
		if self.screen_transition_manager.is_alive() {
			if self.screen_transition_manager.is_finished() {
				self.screen_transition_manager.despawn();
				self.opener_manager.spawn_type(self.current_battle.battle_type);
				self.opener_manager.on_start();
				self.opener_manager.battle_introduction_manager.setup_text(&self.current_battle, self.trainer.as_ref());
			} else {
				self.screen_transition_manager.update(delta);
			}
		} else if self.opener_manager.is_alive() {
			if self.opener_manager.is_finished() {
				self.opener_manager.despawn();
				self.gui.player_panel.start();
			} else {
				self.opener_manager.update(delta);
				self.opener_manager.battle_introduction_manager.update_gui(&mut self.gui, delta);
				//self.gui.opener_update(context);
			}
		} else if self.closer_manager.is_alive() {
			if self.closer_manager.is_finished() {
				// self.closer_manager.update_player(player_data);
				self.closer_manager.despawn();
				self.finished = true;
			} else {
				self.closer_manager.update(delta);
			}
		} else /*if !self.current_battle.is_finished()*/ {
			self.current_battle.update(delta, &mut self.gui, &mut self.closer_manager, party_gui);
			self.gui.update(delta);
		}

	}	

    pub fn render(&self) {

		if self.screen_transition_manager.is_alive() {
			self.screen_transition_manager.render();
		} else if self.opener_manager.is_alive() {
			self.gui.render_background(self.opener_manager.offset());
			self.opener_manager.render_below_panel(&self.current_battle);
			self.gui.render();
			self.gui.render_panel();
			self.opener_manager.render();
		} else if self.closer_manager.is_alive() {
			if !self.world_active() {
				self.gui.render_background(0.0);
				self.current_battle.render_pokemon(self.gui.player_bounce.offset);
				self.gui.render();
				self.gui.render_panel();
			}
			self.closer_manager.render();
		} else {
			self.gui.render_background(0.0);
			self.current_battle.render_pokemon(self.gui.player_bounce.offset);
			self.gui.render();
			self.gui.render_panel();
		}
	}

	pub fn update_data(&self) {
		if let Some(mut player_saves) = get_mut::<PlayerSaves>() {
			self.current_battle.update_data(player_saves.get_mut());
		}
	}

	pub fn player_party(&self) -> &BattleParty {
		&self.current_battle.player
	}

	pub fn world_active(&self) -> bool {
		return self.screen_transition_manager.is_alive() || self.closer_manager.world_active();
	}

	pub fn is_finished(&self) -> bool {
		self.finished
	}
	
}