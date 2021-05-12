use game::{
	util::Completable,
	pokedex::pokemon::saved::SavedPokemonParty,
	storage::player::PlayerSave,
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	macroquad::prelude::{is_key_pressed, KeyCode},
	battle::{BattleEntry, BattleTeam},
};

use crate::{
	Battle,
	state::BattleManagerState,
	ui::{
		BattleGui,
		transitions::{
			BattleTransition,
			BattleOpener,
			BattleCloser,
			managers::{
				transition::BattleScreenTransitionManager,
				opener::BattleOpenerManager,
				introduction::BattleIntroductionManager,
				closer::BattleCloserManager,
			}
		}
	},
};

pub struct BattleManager {

	state: BattleManagerState,
	
	battle: Option<Battle>,
	
	screen_transition: BattleScreenTransitionManager,
	opener: BattleOpenerManager,
	introduction: BattleIntroductionManager,
	closer: BattleCloserManager,

	gui: BattleGui,

	pub finished: bool,
	
}

impl BattleManager {
	
	pub fn new() -> BattleManager {
		
		BattleManager {

			state: BattleManagerState::default(),

			battle: None,

			screen_transition: BattleScreenTransitionManager::default(),
			opener: BattleOpenerManager::default(),
			introduction: BattleIntroductionManager::default(),
			closer: BattleCloserManager::default(),

			gui: BattleGui::new(),

			finished: false,

		}
		
	}

	pub fn battle(&mut self, player: &SavedPokemonParty, entry: BattleEntry) -> bool { // add battle type parameter

		self.finished = false;
		self.state = BattleManagerState::Transition;

		self.battle = Battle::new(&player, entry);

		// Setup battle GUI

		if let Some(battle) = self.battle.as_ref() {
			self.screen_transition.spawn(battle.data.battle_type);
			self.gui.bounce.reset();
		}

		self.battle.is_some()

	}

	pub fn update(&mut self, delta: f32, party_gui: &mut PartyGui, bag_gui: &mut BagGui) {
		if game::is_debug() {
			if is_key_pressed(KeyCode::F1) { // exit shortcut
				self.finished = true;
				if let Some(battle) = self.battle.as_mut() {
					battle.data.winner = Some(BattleTeam::Player);
				}
			}
		}
		if let Some(battle) = self.battle.as_mut() {
			match self.state {
				BattleManagerState::Transition => {
					if self.screen_transition.is_alive() {
						if self.screen_transition.is_finished() {
							self.screen_transition.despawn();
							self.opener.spawn(battle);
							self.state = BattleManagerState::Opener;
						} else {
							self.screen_transition.update(delta);
						}
					} else {
						self.screen_transition.spawn(battle.data.battle_type);
						self.update(delta, party_gui, bag_gui);
					}
				}
				BattleManagerState::Opener => {
					if self.opener.is_alive() {
						if self.opener.is_finished() {
							self.introduction.spawn(self.opener.despawn(), battle, &mut self.gui.text);
							self.state = BattleManagerState::Introduction;
						} else {
							self.opener.update(delta);
						}
					} else {
						self.opener.spawn(battle);
						self.update(delta, party_gui, bag_gui);
					}
				}
				BattleManagerState::Introduction => {
					if self.introduction.is_alive() {
						if self.introduction.is_finished() && self.gui.text.is_finished() {
							self.introduction.despawn();
							self.state = BattleManagerState::Battle;
						} else {
							self.introduction.update(delta, battle, &mut self.gui.text);
						}
					} else {
						self.introduction.spawn(&self.opener.current, battle, &mut self.gui.text);
						self.update(delta, party_gui, bag_gui);
					}
				}
				BattleManagerState::Battle => {
					battle.update(delta, &mut self.gui, &mut self.closer, party_gui, bag_gui);
					if self.closer.is_alive() {
						self.state = BattleManagerState::Closer;
					}
				},
				BattleManagerState::Closer => {
					if self.closer.is_alive() {
						if self.closer.is_finished() {
							self.closer.despawn();
							self.finished = true;
						} else {
							self.closer.update(delta, &mut self.gui.text);
						}
					} else {
						self.closer.spawn(battle, &mut self.gui.text);
						self.update(delta, party_gui, bag_gui);
					}
				}
			}
		}
	}	

    pub fn render(&self) {
		if let Some(battle) = self.battle.as_ref() {
			match self.state {
			    BattleManagerState::Transition => self.screen_transition.render(),
			    BattleManagerState::Opener => {
					self.gui.background.render(self.opener.offset());
					self.opener.render_below_panel(battle);
					self.gui.render_panel();
					self.opener.render();
				}
			    BattleManagerState::Introduction => {
					self.gui.background.render(0.0);
					self.introduction.render(battle);
					self.gui.render_panel();
					self.gui.text.render();
				}
			    BattleManagerState::Battle => battle.render(&self.gui),
			    BattleManagerState::Closer => {
					if !self.world_active() {
						self.gui.background.render(0.0);
						self.gui.render_panel();
						self.gui.panel.render();
						self.closer.render_battle();
						self.gui.text.render();
					}
					self.closer.render();
				}
			}
		}
	}

	pub fn update_data(&mut self, player_save: &mut PlayerSave) -> Option<(BattleTeam, bool)> {
		self.battle.take().map(|battle| battle.update_data(player_save)).flatten()
	}

	pub fn world_active(&self) -> bool {
		self.screen_transition.is_alive() || self.closer.world_active()		
	}
	
}