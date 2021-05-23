use crate::{
	// pokedex::pokemon::p,
	deps::rhai::Engine,
	storage::{data_mut, player::PlayerSave},
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	pokedex::{
		pokemon::instance::BorrowedPokemon,
		moves::target::Team,
	},
	macroquad::prelude::{Vec2, WHITE, is_key_pressed, KeyCode},
	battle::BattleEntry,
	is_debug,
};

use crate::battle::{
	Battle,
	state::{
		BattleManagerState,
		BattleState,
		TransitionState,
	},
	ui::{
		BattleGui,
		transitions::managers::{
			transition::BattleScreenTransitionManager,
			opener::BattleOpenerManager,
			introduction::BattleIntroductionManager,
			closer::BattleCloserManager,
		}
	},
};

pub struct BattleManager {

	state: BattleManagerState,
	
	battle: Option<Battle>,
	
	transition: BattleScreenTransitionManager,
	opener: BattleOpenerManager,
	introduction: BattleIntroductionManager,
	closer: BattleCloserManager,

	engine: Engine,
	gui: BattleGui,

	pub finished: bool,
	
}

impl BattleManager {
	
	pub fn new() -> BattleManager {
		
		BattleManager {

			state: BattleManagerState::default(),

			battle: None,

			transition: BattleScreenTransitionManager::default(),
			opener: BattleOpenerManager::default(),
			introduction: BattleIntroductionManager::default(),
			closer: BattleCloserManager::default(),

			engine: crate::pokedex::moves::script::engine(),
			gui: BattleGui::new(),

			finished: false,

		}
		
	}

	pub fn battle(&mut self, entry: BattleEntry) -> bool { // add battle type parameter
		self.finished = false;
		self.state = BattleManagerState::default();
		self.battle = Battle::new(
			data_mut().party.iter_mut().map(|instance| 
				Some(BorrowedPokemon::Borrowed(instance))
			).collect(), 
			entry
		);
		self.battle.is_some()
	}

	pub fn update(&mut self, delta: f32, party_gui: &mut PartyGui, bag: &mut BagGui) {

		if is_debug() {
			if is_key_pressed(KeyCode::F1) { // exit shortcut
				self.finished = true;
				match self.state {
				    BattleManagerState::Begin => (),
				    BattleManagerState::Transition => self.transition.state = TransitionState::Begin,
				    BattleManagerState::Opener => self.opener.state = TransitionState::Begin,
				    BattleManagerState::Introduction => self.introduction.state = TransitionState::Begin,
				    BattleManagerState::Battle => {
						if let Some(battle) = self.battle.as_mut() {
							battle.state = BattleState::End;
							battle.update(delta, &mut self.engine, &mut self.gui, party_gui, bag);
						}
					},
				    BattleManagerState::Closer => self.closer.state = TransitionState::Begin,
				}
				if let Some(battle) = self.battle.as_mut() {
					battle.data.winner = Some(Team::Player);
				}
				return;
			}
		}
		if let Some(battle) = self.battle.as_mut() {
			match self.state {
				BattleManagerState::Begin => {
					self.gui.reset();
					self.state = BattleManagerState::Transition;
					self.transition.state = TransitionState::Begin;
					self.update(delta, party_gui, bag);
				},
				BattleManagerState::Transition => match self.transition.state {
					TransitionState::Begin => {
						self.transition.begin(battle.data.battle_type, &battle.data.trainer);
						self.update(delta, party_gui, bag);
					},
					TransitionState::Run => self.transition.update(delta),
					TransitionState::End => {
						self.transition.end();
						self.state = BattleManagerState::Opener;
						self.update(delta, party_gui, bag);
					}
				}
				BattleManagerState::Opener => match self.opener.state {
					TransitionState::Begin => {
						self.opener.begin(battle);
						self.update(delta, party_gui, bag);
					}
					TransitionState::Run => self.opener.update(delta),
					TransitionState::End => {
						self.opener.end();
						self.state = BattleManagerState::Introduction;
						self.update(delta, party_gui, bag);
					}
				}
				BattleManagerState::Introduction => match self.introduction.state {
					TransitionState::Begin => {
						self.introduction.begin(battle, &mut self.gui.text);
						self.update(delta, party_gui, bag);
					}
					TransitionState::Run => self.introduction.update(delta, battle, &mut self.gui.text),
					TransitionState::End => {
						self.introduction.end();
						self.state = BattleManagerState::Battle;
						self.update(delta, party_gui, bag);
					}
				}
				BattleManagerState::Battle => match battle.state {
					BattleState::End => self.state = BattleManagerState::Closer,
					_ => battle.update(delta, &mut self.engine, &mut self.gui, party_gui, bag),
				},
				BattleManagerState::Closer => match self.closer.state {
					TransitionState::Begin => {
						self.closer.begin(battle, &mut self.gui.text);
						self.update(delta, party_gui, bag);
					}
					TransitionState::Run => self.closer.update(delta, &mut self.gui.text),
					TransitionState::End => {
						self.closer.end();
						self.state = BattleManagerState::default();
						self.finished = true;
					}
				}
			}
		}
	}	

    pub fn render(&self) {
		if let Some(battle) = self.battle.as_ref() {
			match self.state {
				BattleManagerState::Begin => (),
			    BattleManagerState::Transition => self.transition.render(),
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
					if self.closer.state != TransitionState::End {
						if !self.world_active() {
							self.gui.background.render(0.0);
							self.gui.render_panel();
							self.gui.panel.render();
							for active in battle.player.active.iter() {
								active.renderer.render(Vec2::ZERO, WHITE);
							}
							self.closer.render_battle();
							self.gui.text.render();
						}
						self.closer.render();
					}
				}
			}
		}
	}

	pub fn update_data(&mut self, player_save: &mut PlayerSave) -> Option<(Team, bool)> {
		self.battle.take().map(|battle| battle.update_data(player_save)).flatten()
	}

	pub fn world_active(&self) -> bool {
		self.state == BattleManagerState::Transition || self.closer.world_active()		
	}
	
}