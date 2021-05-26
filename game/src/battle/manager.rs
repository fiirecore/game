use std::{cell::RefCell, rc::Rc};

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
	input::{debug_pressed, DebugBind},
	graphics::ZERO,
	tetra::{
		Context,
		graphics::Color,
	},
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

use super::{player::gui::BattlePlayerGui, ui::panels::BattlePanel};

pub struct BattleManager {

	state: BattleManagerState,
	
	battle: Option<Battle>,
	
	transition: BattleScreenTransitionManager,
	opener: BattleOpenerManager,
	introduction: BattleIntroductionManager,
	closer: BattleCloserManager,

	engine: Engine,
	gui: BattleGui,

	player: BattlePlayerGui,

	pub finished: bool,
	
}

impl BattleManager {
	
	pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> BattleManager {
		
		BattleManager {

			state: BattleManagerState::default(),

			battle: None,

			transition: BattleScreenTransitionManager::new(ctx),
			opener: BattleOpenerManager::new(ctx),
			introduction: BattleIntroductionManager::new(ctx),
			closer: BattleCloserManager::default(),

			engine: crate::pokedex::moves::usage::script::engine(),
			gui: BattleGui::new(ctx),

			player: BattlePlayerGui {
				party,
				bag,
				panel: Rc::new(RefCell::new(BattlePanel::new(ctx))),
			},

			finished: false,

		}
		
	}

	pub fn battle(&mut self, ctx: &mut Context, entry: BattleEntry) -> bool { // add battle type parameter
		self.finished = false;
		self.state = BattleManagerState::default();
		self.battle = Battle::new(
			ctx,
			Box::new(self.player.clone()),
			data_mut().party.iter_mut().map(|instance| 
				Some(BorrowedPokemon::Borrowed(instance))
			).collect(), 
			entry
		);
		self.battle.is_some()
	}

	pub fn update(&mut self, ctx: &mut Context, delta: f32) {
		if is_debug() {
			if debug_pressed(ctx, DebugBind::F1) { // exit shortcut
				self.finished = true;
				match self.state {
				    BattleManagerState::Begin => (),
				    BattleManagerState::Transition => self.transition.state = TransitionState::Begin,
				    BattleManagerState::Opener => self.opener.state = TransitionState::Begin,
				    BattleManagerState::Introduction => self.introduction.state = TransitionState::Begin,
				    BattleManagerState::Battle => {
						if let Some(battle) = self.battle.as_mut() {
							battle.state = BattleState::End;
							battle.update(ctx, delta, &mut self.engine, &mut self.gui);
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
					self.update(ctx, delta);
				},
				BattleManagerState::Transition => match self.transition.state {
					TransitionState::Begin => {
						self.transition.begin(ctx, battle.data.battle_type, &battle.data.trainer);
						self.update(ctx, delta);
					},
					TransitionState::Run => self.transition.update(ctx, delta),
					TransitionState::End => {
						self.transition.end();
						self.state = BattleManagerState::Opener;
						self.update(ctx, delta);
					}
				}
				BattleManagerState::Opener => match self.opener.state {
					TransitionState::Begin => {
						self.opener.begin(battle);
						self.update(ctx, delta);
					}
					TransitionState::Run => self.opener.update(delta),
					TransitionState::End => {
						self.opener.end();
						self.state = BattleManagerState::Introduction;
						self.update(ctx, delta);
					}
				}
				BattleManagerState::Introduction => match self.introduction.state {
					TransitionState::Begin => {
						self.introduction.begin(battle, &mut self.gui.text);
						self.update(ctx, delta);
					}
					TransitionState::Run => self.introduction.update(ctx, delta, battle, &mut self.gui.text),
					TransitionState::End => {
						self.introduction.end();
						self.state = BattleManagerState::Battle;
						self.update(ctx, delta);
					}
				}
				BattleManagerState::Battle => match battle.state {
					BattleState::End => self.state = BattleManagerState::Closer,
					_ => battle.update(ctx, delta, &mut self.engine, &mut self.gui),
				},
				BattleManagerState::Closer => match self.closer.state {
					TransitionState::Begin => {
						self.closer.begin(battle, &mut self.gui.text);
						self.update(ctx, delta);
					}
					TransitionState::Run => self.closer.update(ctx, delta, &mut self.gui.text),
					TransitionState::End => {
						self.closer.end();
						self.state = BattleManagerState::default();
						self.finished = true;
					}
				}
			}
		}
	}	

    pub fn draw(&self, ctx: &mut Context) {
		if let Some(battle) = self.battle.as_ref() {
			match self.state {
				BattleManagerState::Begin => (),
			    BattleManagerState::Transition => self.transition.draw(ctx),
			    BattleManagerState::Opener => {
					self.gui.background.draw(ctx, self.opener.offset());
					self.opener.draw_below_panel(ctx, battle);
					self.gui.draw_panel(ctx);
					self.opener.draw(ctx);
				}
			    BattleManagerState::Introduction => {
					self.gui.background.draw(ctx, 0.0);
					self.introduction.draw(ctx, battle);
					self.gui.draw_panel(ctx);
					self.gui.text.draw(ctx);
				}
			    BattleManagerState::Battle => battle.render(ctx, &self.gui),
			    BattleManagerState::Closer => {
					if self.closer.state != TransitionState::End {
						if !self.world_active() {
							self.gui.background.draw(ctx, 0.0);
							self.gui.draw_panel(ctx);
							battle.player.player.draw(ctx);
							for active in battle.player.party.active.iter() {
								active.renderer.draw(ctx, ZERO, Color::WHITE);
							}
							self.closer.draw_battle(ctx);
							self.gui.text.draw(ctx);
						}
						self.closer.draw(ctx);
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