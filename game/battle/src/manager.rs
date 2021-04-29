use game::{
	util::{Entity, Completable},
	pokedex::pokemon::saved::SavedPokemonParty,
	storage::player::PlayerSave,
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	macroquad::prelude::{is_key_pressed, KeyCode},
	battle::{BattleData, BattleWinner},
};

// use crate::script::action::ActionTextures;

use crate::{
	Battle,
	pokemon::BattleParty,
	gui::BattleGui,
	transitions::{
		BattleTransition,
		BattleTransitionGui,
		BattleOpener,
		BattleCloser,
		managers::{
			screen_transition::BattleScreenTransitionManager,
			opener::BattleOpenerManager,
			closer::BattleCloserManager,
		}
	}
};

// pub type TrainerTextures = HashMap<String, Texture2D>;

pub struct BattleManager {
	
	battle: Option<Battle>,
	
	screen_transition: BattleScreenTransitionManager,
	opener: BattleOpenerManager,
	closer: BattleCloserManager,

	gui: BattleGui,

	// action_textures: ActionTextures,

	finished: bool,
	
}

impl BattleManager {
	
	pub fn new() -> BattleManager {
		
		BattleManager {

			battle: None,

			screen_transition: BattleScreenTransitionManager::new(),
			opener: BattleOpenerManager::new(),
			closer: BattleCloserManager::new(),

			gui: BattleGui::new(),

			// action_textures: ActionTextures::new(),

			finished: false,

		}
		
	}

	pub fn battle(&mut self, player: &SavedPokemonParty, data: BattleData) -> bool { // add battle type parameter

		self.finished = false;

		// Create the battle

		self.battle = Battle::new(&player, data);

		// Despawn anything from previous battle

		self.gui.despawn();

		// Setup transition and GUI

		if let Some(battle) = self.battle.as_ref() {
			self.screen_transition.spawn_with_type(battle.battle_type);
			self.gui.on_battle_start(battle);
		}

		self.battle.is_some()

	}

	pub fn input(&mut self, party_gui: &mut PartyGui, bag_gui: &mut BagGui) {

		if let Some(battle) = self.battle.as_mut() {
			if !self.screen_transition.is_alive() {	
				if self.opener.is_alive() {
					self.opener.introduction.input();
				} else if self.closer.is_alive() {
					self.closer.input();
				} else {
					self.gui.input(battle, party_gui, bag_gui);
				}
			}
		}

		if game::is_debug() {
			if is_key_pressed(KeyCode::F1) {
				// exit shortcut
				self.finished = true;
				if let Some(battle) = self.battle.as_mut() {
					battle.winner = Some(BattleWinner::Player);
				}
			}
		}

	}

	pub fn update(&mut self, delta: f32, party_gui: &mut PartyGui, bag_gui: &mut BagGui) {
		
		if let Some(battle) = self.battle.as_mut() {

			if self.screen_transition.is_alive() {
				if self.screen_transition.is_finished() {
					self.screen_transition.despawn();
					self.opener.spawn_type(battle.battle_type);
					self.opener.on_start();
					self.opener.introduction.setup_text(battle);
				} else {
					self.screen_transition.update(delta);
				}
			} else if self.opener.is_alive() {
				if self.opener.is_finished() {
					self.opener.despawn();
					self.gui.panel.start();
				} else {
					self.opener.update(delta);
					self.opener.introduction.update_gui(battle, &mut self.gui, delta);
					//self.gui.opener_update(context);
				}
			} else if self.closer.is_alive() {
				if self.closer.is_finished() {
					// self.closer.update_player(player_data);
					self.closer.despawn();
					self.finished = true;
				} else {
					self.closer.update(delta);
				}
			} else /*if !self.current_battle.is_finished()*/ {
				battle.update(delta, &mut self.gui, &mut self.closer, party_gui, bag_gui);
				self.gui.update(delta);
			}

		}

	}	

    pub fn render(&self) {

		if let Some(battle) = self.battle.as_ref() {

			if self.screen_transition.is_alive() {
				self.screen_transition.render();
			} else if self.opener.is_alive() {
				self.gui.render_background(self.opener.offset());
				self.opener.render_below_panel(battle);
				self.gui.render();
				self.gui.render_panel();
				self.opener.render();
			} else if self.closer.is_alive() {
				if !self.world_active() {
					self.gui.render_background(0.0);
					battle.render_pokemon(self.gui.bounce.offset);
					self.gui.render_panel();
					self.gui.render();
					self.closer.render_battle();
				}
				self.closer.render();
			} else {
				self.gui.render_background(0.0);
				battle.render_pokemon(self.gui.bounce.offset);
				self.gui.render();
				self.gui.render_panel();
			}

		}

	}

	pub fn update_data(&mut self, player_save: &mut PlayerSave) -> Option<(BattleWinner, bool)> {
		self.battle.take().map(|battle| battle.update_data(player_save)).flatten()
	}

	pub fn player_party(&self) -> Option<&BattleParty> {
		self.battle.as_ref().map(|battle| &battle.player)
	}

	pub fn world_active(&self) -> bool {
		self.screen_transition.is_alive() || 
		if self.closer.is_alive() {
			self.closer.world_active()
		} else {
			false
		}		
	}

	pub fn is_finished(&self) -> bool {
		self.finished
	}
	
}