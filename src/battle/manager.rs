use std::rc::Rc;

use crate::{engine::{
		input::{debug_pressed, DebugBind},
		graphics::ZERO,
		tetra::{graphics::Color},
		EngineContext,
	}, game::{battle_glue::{BattleEntry, BattleId}, is_debug}};

use pokedex::{context::PokedexClientContext, gui::{
		party::PartyGui,
		bag::BagGui,
	}};
use rand::{SeedableRng, prelude::SmallRng};
use saves::PlayerData;

use super::{GameBattleWrapper};

use firecore_battle_gui::{BattlePlayerGui, transition::TransitionState};

pub mod transitions;

use transitions::managers::{
	transition::BattleScreenTransitionManager,
	closer::BattleCloserManager,
};

pub struct BattleManager<'d> {

	state: BattleManagerState,
	
	battle: GameBattleWrapper<'d>,
	
	transition: BattleScreenTransitionManager,
	closer: BattleCloserManager,

	player: BattlePlayerGui<'d, BattleId>,

	pub random: SmallRng,

	pub finished: bool,
	
}

#[derive(Debug)]
pub enum BattleManagerState {
	Begin,
	Transition,
	Battle,
	Closer(BattleId),
}

impl Default for BattleManagerState {
    fn default() -> Self {
        Self::Begin
    }
}

impl<'d> BattleManager<'d> {
	
	pub fn new(ctx: &mut EngineContext, dex: &PokedexClientContext<'d>, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
		
		Self {

			state: BattleManagerState::default(),

			battle: GameBattleWrapper::new(),

			transition: BattleScreenTransitionManager::new(ctx),
			closer: BattleCloserManager::default(),

			player: BattlePlayerGui::new(ctx, dex, party, bag),

			random: SmallRng::seed_from_u64(0),

			finished: false,

		}
		
	}

	pub fn battle(&mut self, save: &mut PlayerData, dex: &PokedexClientContext<'d>, entry: BattleEntry) -> bool { // add battle type parameter
		self.finished = false;
		self.state = BattleManagerState::default();
		let player = &mut save.party;
		(!(
			player.is_empty() || 
			entry.party.is_empty() ||
			// Checks if player has any pokemon in party that aren't fainted (temporary)
			!player.iter().any(|pokemon| !pokemon.fainted())
		)).then(|| {
				self.battle.battle(
					&mut self.random,
					dex.pokedex,
					dex.movedex,
					dex.itemdex,
					save,
					self.player.endpoint(),
					entry
				)
			}
		);
		self.battle.battle.is_some()
	}

	pub fn update(&mut self, ctx: &mut EngineContext, dex: &PokedexClientContext<'d>, delta: f32, save: &mut PlayerData<'d>) {
		if is_debug() {
			if debug_pressed(ctx, DebugBind::F1) { // exit shortcut
				self.end();
				return;
			}
		}
		if let Some(battle) = &mut self.battle.battle {
			self.player.process(&mut self.random, dex, &mut save.party);
			match self.state {
				BattleManagerState::Begin => {
					self.player.gui.reset();
					self.state = BattleManagerState::Transition;
					self.transition.state = TransitionState::Begin;

					battle.begin();

					// self.player.on_begin(dex);

					self.update(ctx, dex, delta, save);
				},
				BattleManagerState::Transition => match self.transition.state {
					TransitionState::Begin => {
						self.transition.begin(ctx, self.player.data.type_, &self.battle.trainer);
						self.update(ctx, dex, delta, save);
					},
					TransitionState::Run => self.transition.update(ctx, delta),
					TransitionState::End => {
						self.transition.end();
						self.state = BattleManagerState::Battle;
						self.player.start(true);
						self.update(ctx, dex, delta, save);
					}
				}
				BattleManagerState::Battle => {

					if self.player.battling() {

						battle.update(&mut self.random, &mut self.battle.engine, dex.movedex, dex.itemdex);
	
						if let Some(winner) = battle.winner() {
							self.state = BattleManagerState::Closer(*winner);
						}
					}

					self.player.update(ctx, dex, delta, &mut save.bag);

				},
				BattleManagerState::Closer(winner) => match self.closer.state {
					TransitionState::Begin => {
						self.closer.begin(dex, self.player.data.type_, self.player.local.player.id(), self.player.local.player.name(), Some(&winner), self.battle.trainer.as_ref(), &mut self.player.gui.text);
						self.update(ctx, dex, delta, save);
					}
					TransitionState::Run => self.closer.update(ctx, delta, &mut self.player.gui.text),
					TransitionState::End => {
						self.closer.end();
						self.state = BattleManagerState::default();
						self.finished = true;
					}
				}
			}
		}

		if let Some(ai) = self.battle.ai.as_mut() {
			ai.update();
		}

	}

	pub fn winner(&self) -> Option<&BattleId> {
		self.battle.battle.as_ref().map(|b| b.winner()).flatten()
	}

	pub fn update_data(&mut self, winner: &BattleId, player_save: &mut PlayerData) -> bool {
		self.battle.update_data(winner, player_save)
	}

	pub fn world_active(&self) -> bool {
		matches!(self.state, BattleManagerState::Transition) || self.closer.world_active()		
	}

	pub fn seed(&mut self, seed: u64) {
		self.battle.seed = seed;
		self.random = SmallRng::seed_from_u64(seed);
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

	pub fn draw(&self, ctx: &mut EngineContext, dex: &PokedexClientContext<'d>, save: &PlayerData<'d>) {
        if self.battle.battle.is_some() {
			match self.state {
				BattleManagerState::Begin => (),
			    BattleManagerState::Transition => self.transition.draw(ctx),
			    BattleManagerState::Battle => self.player.draw(ctx, dex, &save.party, &save.bag),
			    BattleManagerState::Closer(..) => {
					if !matches!(self.closer.state, TransitionState::End) {
						if !self.world_active() {
							self.player.gui.background.draw(ctx, 0.0);
							self.player.gui.draw_panel(ctx);
							self.player.draw(ctx, dex, &save.party, &save.bag);
							for active in self.player.local.renderer.iter() {
								active.pokemon.draw(ctx, ZERO, Color::WHITE);
							}
							self.closer.draw_battle(ctx);
							self.player.gui.text.draw(ctx);
						}
						self.closer.draw(ctx);
					}
				}
			}
		}
    }
	
}