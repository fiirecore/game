use std::{ops::Deref, rc::Rc};

use crate::{engine::{
		input::keyboard::{Key, pressed as is_key_pressed},
		graphics::Color,
		Context,
	}, game::{battle_glue::{BattleEntry, BattleId}}};

use crate::pokedex::{context::PokedexClientData, gui::{
		party::PartyGui,
		bag::BagGui,
	}};
use firecore_battle::pokedex::Dex;
use rand::{SeedableRng, prelude::SmallRng};
use worldlib::character::player::PlayerCharacter;
use crate::saves::OwnedPlayer;

use crate::pokedex::{pokemon::Pokemon, moves::Move, item::Item};

use super::{GameBattleWrapper};

use firecore_battle_gui::{BattlePlayerGui, context::BattleGuiContext, transition::TransitionState};

pub mod transitions;

use transitions::managers::{
	transition::BattleScreenTransitionManager,
	closer::BattleCloserManager,
};

pub struct BattleManager<P: Deref<Target = Pokemon> + Clone, M: Deref<Target = Move> + Clone, I: Deref<Target = Item> + Clone> {

	state: BattleManagerState,
	
	battle: GameBattleWrapper<P, M, I>,

	dex: Rc<PokedexClientData>,
	btl: BattleGuiContext,
	
	transition: BattleScreenTransitionManager,
	closer: BattleCloserManager,

	player: BattlePlayerGui<BattleId, P, M, I>,

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

impl<P: Deref<Target = Pokemon> + Clone, M: Deref<Target = Move> + Clone, I: Deref<Target = Item> + Clone> BattleManager<P, M, I> {
	
	pub fn new(ctx: &mut Context, btl: BattleGuiContext, dex: Rc<PokedexClientData>, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
		
		Self {

			state: BattleManagerState::default(),

			battle: GameBattleWrapper::new(),

			transition: BattleScreenTransitionManager::new(ctx),
			closer: BattleCloserManager::default(),

			player: BattlePlayerGui::new(ctx, &btl, party, bag),

			random: SmallRng::seed_from_u64(0),

			finished: false,
    dex,
    btl,

		}
		
	}

	pub fn battle<'d>(&mut self, 
		pokedex: &'d dyn Dex<'d, Pokemon, P>,
	movedex: &'d dyn Dex<'d, Move, M>,
	itemdex: &'d dyn Dex<'d, Item, I>, 
	save: &mut OwnedPlayer<P, M, I>, entry: BattleEntry, 
        ) -> bool { // add battle type parameter
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
					pokedex, movedex, itemdex,
					save,
					self.player.endpoint(),
					entry
				)
			}
		);
		self.battle.battle.is_some()
	}

	pub fn update<'d>(&mut self, ctx: &mut Context,
		pokedex: &'d dyn Dex<'d, Pokemon, P>,
	movedex: &'d dyn Dex<'d, Move, M>,
	itemdex: &'d dyn Dex<'d, Item, I>, delta: f32, save: &mut OwnedPlayer<P, M, I>) {
		if ctx.debug() && is_key_pressed(ctx, Key::F1) { // exit shortcut
  				self.end();
  				return;
  			}
		if let Some(battle) = &mut self.battle.battle {
			self.player.process(&mut self.random, &self.dex, &self.btl,
				pokedex,
				movedex,
				itemdex, &mut save.party, );
			match self.state {
				BattleManagerState::Begin => {
					self.player.gui.reset();
					self.state = BattleManagerState::Transition;
					self.transition.state = TransitionState::Begin;

					battle.begin();

					// self.player.on_begin(dex);

					self.update(ctx, pokedex, movedex, itemdex, delta, save);
				},
				BattleManagerState::Transition => match self.transition.state {
					TransitionState::Begin => {
						self.transition.begin(ctx, self.player.data.type_, &self.battle.trainer);
						self.update(ctx, pokedex, movedex, itemdex, delta, save);
					},
					TransitionState::Run => self.transition.update(ctx, delta),
					TransitionState::End => {
						self.transition.end();
						self.state = BattleManagerState::Battle;
						self.player.start(true);
						self.update(ctx, pokedex, movedex, itemdex, delta, save);
					}
				}
				BattleManagerState::Battle => {

					if !battle.finished() {
						battle.update(&mut self.random, &mut self.battle.engine, delta, movedex, itemdex);
					}

					self.player.update(ctx, &self.dex, pokedex, movedex, itemdex, delta, &mut save.bag);

					if let Some(winner) = self.player.winner().flatten() {
						self.state = BattleManagerState::Closer(*winner);
					}

				},
				BattleManagerState::Closer(winner) => match self.closer.state {
					TransitionState::Begin => {
						self.closer.begin(&self.dex, self.player.data.type_, self.player.local.player.id(), self.player.local.player.name(), Some(&winner), self.battle.trainer.as_ref(), &mut self.player.gui.text);
						self.update(ctx, pokedex, movedex, itemdex, delta, save);
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

	pub fn update_data(&mut self, winner: bool, player: &mut PlayerCharacter) -> bool {
		self.battle.update_data(winner, player)
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

	pub fn draw(&self, ctx: &mut Context, save: &OwnedPlayer<P, M, I>) {
        if self.battle.battle.is_some() {
			match self.state {
				BattleManagerState::Begin => (),
			    BattleManagerState::Transition => self.transition.draw(ctx),
			    BattleManagerState::Battle => self.player.draw(ctx, &self.dex, &save.party, &save.bag),
			    BattleManagerState::Closer(..) => {
					if !matches!(self.closer.state, TransitionState::End) {
						if !self.world_active() {
							self.player.gui.background.draw(ctx, 0.0);
							self.player.gui.draw_panel(ctx);
							self.player.draw(ctx, &self.dex, &save.party, &save.bag);
							for active in self.player.local.renderer.iter() {
								active.pokemon.draw(ctx, firecore_battle_gui::pokedex::engine::math::Vec2::ZERO, Color::WHITE);
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