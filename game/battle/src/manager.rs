use game::{
	util::{Entity, Completable},
	pokedex::pokemon::saved::SavedPokemonParty,
	storage::player::PlayerSave,
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	macroquad::prelude::{is_key_pressed, KeyCode},
	battle::{BattleEntry, BattleTeam},
	graphics::draw,
};

// use crate::script::action::ActionTextures;

use crate::{
	Battle,
	gui::BattleGui,
	transitions::{
		BattleTransition,
		BattleTransitionGui,
		BattleOpener,
		BattleCloser,
		managers::{
			screen_transition::BattleScreenTransitionManager,
			opener::BattleOpenerManager,
		}
	}
};

pub use crate::transitions::managers::closer::BattleCloserManager;

// pub type TrainerTextures = HashMap<String, Texture2D>;

pub struct BattleManager {

	// state: BattleManagerState,
	
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

	pub fn battle(&mut self, player: &SavedPokemonParty, entry: BattleEntry) -> bool { // add battle type parameter

		self.finished = false;

		// Create the battle

		self.battle = Battle::new(&player, entry);

		// Despawn anything from previous battle

		// self.gui.despawn();

		// Setup battle GUI

		if let Some(battle) = self.battle.as_ref() {
			self.screen_transition.spawn_with_type(battle.data.battle_type);
			self.gui.bounce.reset();
		}

		self.battle.is_some()

	}

	pub fn input(&mut self) {

		if let Some(battle) = self.battle.as_mut() {
			if !self.screen_transition.is_alive() {	
				if self.opener.is_alive() {
					self.opener.introduction.input();
				} else if self.closer.is_alive() {
					self.closer.input();
				} else {
					battle.input(&mut self.gui);
				}
			}
		}

		if game::is_debug() {
			if is_key_pressed(KeyCode::F1) {
				// exit shortcut
				self.finished = true;
				if let Some(battle) = self.battle.as_mut() {
					battle.data.winner = Some(BattleTeam::Player);
				}
			}
		}

	}

	pub fn update(&mut self, delta: f32, party_gui: &mut PartyGui, bag_gui: &mut BagGui) {
		
		if let Some(battle) = self.battle.as_mut() {

			// Update the level up move thing

			// if self.gui.level_up.is_alive() {

			// 	#[deprecated(note = "move this")]
			// 	self.gui.level_up.update(delta, battle.player.pokemon_mut(self.gui.level_up.index).unwrap());

			// 	return;

			// }

			if self.screen_transition.is_alive() {
				if self.screen_transition.is_finished() {
					self.screen_transition.despawn();
					self.opener.spawn_type(battle);
				} else {
					self.screen_transition.update(delta);
				}
			} else if self.opener.is_alive() {
				if self.opener.is_finished() {
					self.opener.despawn();
				} else {
					self.opener.update(delta);
					self.opener.introduction.update_gui(delta, battle);
				}
			} else if self.closer.is_alive() {
				if self.closer.is_finished() {
					self.closer.despawn();
					self.finished = true;
				} else {
					self.closer.update(delta);
				}
			} else /*if !self.current_battle.is_finished()*/ {
				self.gui.bounce.update(delta);
				battle.update(delta, &mut self.gui, &mut self.closer, party_gui, bag_gui);
			}

		}

	}	

    pub fn render(&self) {

		if let Some(battle) = self.battle.as_ref() {

			if self.screen_transition.is_alive() {
				self.screen_transition.render();
			} else if self.opener.is_alive() {
				self.gui.background.render(self.opener.offset());
				self.opener.render_below_panel(battle);
        		draw(self.gui.background.panel, 0.0, 113.0);
				self.opener.render();
			} else if self.closer.is_alive() {
				if !self.world_active() {
					self.gui.background.render(0.0);
					draw(self.gui.background.panel, 0.0, 113.0);
					self.gui.panel.render();
					self.closer.render_battle();
				}
				self.closer.render();
			} else {
				battle.render(&self.gui);
			}

		}

	}

	pub fn update_data(&mut self, player_save: &mut PlayerSave) -> Option<(BattleTeam, bool)> {
		self.battle.take().map(|battle| battle.update_data(player_save)).flatten()
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