use std::rc::Rc;

use crate::{
	storage::{data_mut, player::PlayerSave},
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	pokedex::{
		pokemon::instance::BorrowedPokemon,
		trainer::TrainerId,
	},
	input::{debug_pressed, DebugBind},
	graphics::ZERO,
	tetra::{
		Context,
		graphics::Color,
	},
	battle_cli::BattleEntry,
	is_debug,
};

use battle::player::{BattlePlayer, PlayerSettings};

use crate::battle_cli::{
	GameBattleWrapper,
	clients::gui::{
		guiref::BattlePlayerGuiRef,
		transition::TransitionState,
	},
};

pub mod transitions;

use transitions::managers::{
	transition::BattleScreenTransitionManager,
	closer::BattleCloserManager,
};

pub struct BattleManager {

	state: BattleManagerState,
	
	battle: GameBattleWrapper,
	
	transition: BattleScreenTransitionManager,
	closer: BattleCloserManager,

	player: BattlePlayerGuiRef<TrainerId>,

	pub finished: bool,
	
}

#[derive(Debug)]
pub enum BattleManagerState {
	Begin,
	Transition,
	Battle,
	Closer(TrainerId),
}

impl Default for BattleManagerState {
    fn default() -> Self {
        Self::Begin
    }
}

impl BattleManager {
	
	pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> BattleManager {
		
		BattleManager {

			state: BattleManagerState::default(),

			battle: GameBattleWrapper::new(),

			transition: BattleScreenTransitionManager::new(ctx),
			closer: BattleCloserManager::default(),

			player: BattlePlayerGuiRef::new(ctx, party, bag, <pokedex::moves::Move as deps::borrow::Identifiable>::UNKNOWN),

			finished: false,

		}
		
	}

	pub fn battle(&mut self, entry: BattleEntry) -> bool { // add battle type parameter
		self.finished = false;
		self.state = BattleManagerState::default();
		let data = data_mut();
		let player = &mut data.party;
		(!(
			player.is_empty() || 
			entry.party.is_empty() ||
			// Checks if player has any pokemon in party that aren't fainted (temporary)
			!player.iter().any(|pokemon| !pokemon.fainted())
		)).then(|| {
				let data = data_mut();
				self.battle.battle(
					BattlePlayer::new(
						data.id,
						data.party.iter_mut().map(|instance| BorrowedPokemon::Borrowed(instance)).collect(), 
						None,
						PlayerSettings {
							gains_exp: true,
						},
						Box::new(self.player.clone()),
						entry.size
					),
					entry
				)
			}
		);
		self.battle.battle.is_some()
	}

	pub fn update(&mut self, ctx: &mut Context, delta: f32, input_lock: bool) {
		if is_debug() {
			if debug_pressed(ctx, DebugBind::F1) { // exit shortcut
				self.end();
				return;
			}
		}
		if let Some(battle) = &mut self.battle.battle {
			match self.state {
				BattleManagerState::Begin => {
					self.player.get().gui.reset();
					self.state = BattleManagerState::Transition;
					self.transition.state = TransitionState::Begin;

					battle.battle.begin();

					self.player.get().on_begin(ctx);

					self.update(ctx, delta, input_lock);
				},
				BattleManagerState::Transition => match self.transition.state {
					TransitionState::Begin => {
						self.transition.begin(ctx, self.player.get().battle_data.type_, &battle.trainer);
						self.update(ctx, delta, input_lock);
					},
					TransitionState::Run => self.transition.update(ctx, delta),
					TransitionState::End => {
						self.transition.end();
						self.state = BattleManagerState::Battle;
						self.player.get().start(true);
						self.update(ctx, delta, input_lock);
					}
				}
				BattleManagerState::Battle => {

					let player = self.player.get();

					if player.battling() {

						battle.battle.update(&self.battle.engine);
	
						if let Some(winner) = player.winner() {
							self.state = BattleManagerState::Closer(winner);
						}
					}

					player.update(ctx, delta, input_lock);

				},
				BattleManagerState::Closer(winner) => match self.closer.state {
					TransitionState::Begin => {
						self.closer.begin(self.player.get().battle_data.type_, Some(&winner), self.player.get().opponent.party.trainer.as_ref(), battle.trainer.as_ref(), &mut self.player.get().gui.text);
						self.update(ctx, delta, input_lock);
					}
					TransitionState::Run => self.closer.update(ctx, delta, &mut self.player.get().gui.text),
					TransitionState::End => {
						self.closer.end();
						self.state = BattleManagerState::default();
						self.finished = true;
					}
				}
			}
		}
	}

	pub fn winner(&self) -> Option<TrainerId> {
		self.player.get().winner()
	}

	pub fn update_data(&mut self, winner: &TrainerId, player_save: &mut PlayerSave) -> bool {
		self.battle.battle.as_mut().map(|b| b.update_data(winner, player_save)).unwrap_or_default()
	}

	pub fn world_active(&self) -> bool {
		matches!(self.state, BattleManagerState::Transition) || self.closer.world_active()		
	}

	pub fn end(&mut self) {
		self.finished = true;
		match self.state {
			BattleManagerState::Begin => (),
			BattleManagerState::Transition => self.transition.state = TransitionState::Begin,
			BattleManagerState::Battle => self.battle.battle = None,
			BattleManagerState::Closer(..) => self.closer.state = TransitionState::Begin,
		}
	}

	pub fn draw(&self, ctx: &mut Context) {
        if self.battle.battle.is_some() {
			match self.state {
				BattleManagerState::Begin => (),
			    BattleManagerState::Transition => self.transition.draw(ctx),
			    BattleManagerState::Battle => self.player.get().draw(ctx),
			    BattleManagerState::Closer(..) => {
					if !matches!(self.closer.state, TransitionState::End) {
						if !self.world_active() {
							self.player.get().gui.background.draw(ctx, 0.0);
							self.player.get().gui.draw_panel(ctx);
							self.player.get().draw(ctx);
							for active in self.player.get().player.renderer.iter() {
								active.renderer.draw(ctx, ZERO, Color::WHITE);
							}
							self.closer.draw_battle(ctx);
							self.player.get().gui.text.draw(ctx);
						}
						self.closer.draw(ctx);
					}
				}
			}
		}
    }
	
}