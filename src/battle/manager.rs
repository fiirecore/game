use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_util::{Entity, Completable};
use macroquad::prelude::collections::storage::get_mut;
use crate::data::player::list::PlayerSaves;
use crate::gui::game::party::PokemonPartyGui;
use crate::util::battle_data::BattleData;
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
	
	battle: Option<Battle>,
	
	screen_transition_manager: BattleScreenTransitionManager,
	opener_manager: BattleOpenerManager,
	closer_manager: BattleCloserManager,

	gui: BattleGui,

	finished: bool,
	
}

impl BattleManager {
	
	pub fn new() -> BattleManager {
		
		BattleManager {

			battle: None,

			screen_transition_manager: BattleScreenTransitionManager::new(),
			opener_manager: BattleOpenerManager::new(),
			closer_manager: BattleCloserManager::default(),

			gui: BattleGui::new(),

			finished: false,

		}
		
	}

	pub fn battle(&mut self, player: &PokemonParty, data: BattleData) -> bool { // add battle type parameter

		self.finished = false;

		// Create the battle

		self.battle = Battle::new(&player, data);

		// Despawn anything from previous battle

		self.gui.despawn();

		// Setup transition and GUI

		if let Some(battle) = self.battle.as_ref() {
			self.screen_transition_manager.spawn_with_type(battle.battle_type);
			self.gui.on_battle_start(battle);
		}

		self.battle.is_some()

	}

	pub fn input(&mut self, delta: f32) {

		if let Some(battle) = self.battle.as_mut() {
			if !self.screen_transition_manager.is_alive() {	
				if self.opener_manager.is_alive() {
					self.opener_manager.battle_introduction_manager.input();
				} else if self.closer_manager.is_alive() {
					//self.closer_manager.input(context);
				} else {
					self.gui.input(delta, battle);
				}
			}
		}

		if crate::debug() {
			if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F1) {
				// exit shortcut
				self.finished = true;
				if let Some(mut saves) = get_mut::<PlayerSaves>() {
					if let Some(battle) = self.battle.take() {
						battle.update_data(saves.get_mut());
					}
				}
			}
		}

	}

	pub fn update(&mut self, delta: f32, party_gui: &mut PokemonPartyGui) {
		
		if let Some(battle) = self.battle.as_mut() {

			if self.screen_transition_manager.is_alive() {
				if self.screen_transition_manager.is_finished() {
					self.screen_transition_manager.despawn();
					self.opener_manager.spawn_type(battle.battle_type);
					self.opener_manager.on_start();
					self.opener_manager.battle_introduction_manager.setup_text(battle);
				} else {
					self.screen_transition_manager.update(delta);
				}
			} else if self.opener_manager.is_alive() {
				if self.opener_manager.is_finished() {
					self.opener_manager.despawn();
					self.gui.player_panel.start();
				} else {
					self.opener_manager.update(delta);
					self.opener_manager.battle_introduction_manager.update_gui(battle, &mut self.gui, delta);
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
				battle.update(delta, &mut self.gui, &mut self.closer_manager, party_gui);
				self.gui.update(delta);
			}

		}

	}	

    pub fn render(&self) {

		if let Some(battle) = self.battle.as_ref() {

			if self.screen_transition_manager.is_alive() {
				self.screen_transition_manager.render();
			} else if self.opener_manager.is_alive() {
				self.gui.render_background(self.opener_manager.offset());
				self.opener_manager.render_below_panel(battle);
				self.gui.render();
				self.gui.render_panel();
				self.opener_manager.render();
			} else if self.closer_manager.is_alive() {
				if !self.world_active() {
					self.gui.render_background(0.0);
					battle.render_pokemon(self.gui.player_bounce.offset);
					self.gui.render();
					self.gui.render_panel();
				}
				self.closer_manager.render();
			} else {
				self.gui.render_background(0.0);
				battle.render_pokemon(self.gui.player_bounce.offset);
				self.gui.render();
				self.gui.render_panel();
			}

		}

	}

	pub fn update_data(&mut self) {
		if let Some(mut player_saves) = get_mut::<PlayerSaves>() {
			if let Some(battle) = self.battle.take() {
				battle.update_data(player_saves.get_mut());
			}
		}
	}

	pub fn player_party(&self) -> Option<&BattleParty> {
		self.battle.as_ref().map(|battle| &battle.player)
	}

	pub fn world_active(&self) -> bool {
		return self.screen_transition_manager.is_alive() || self.closer_manager.world_active();
	}

	pub fn is_finished(&self) -> bool {
		self.finished
	}
	
}