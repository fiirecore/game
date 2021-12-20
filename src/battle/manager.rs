use std::{ops::Deref, rc::Rc};

use crate::{engine::{
		input::keyboard::{Key, pressed as is_key_pressed},
		graphics::Color,
		Context,
	}, game::{battle_glue::{BattleEntry, BattleId}}};

use crate::pokedex::{PokedexClientData, gui::{
		party::PartyGui,
		bag::BagGui,
	}};
use battlelib::{default_engine::{scripting::MoveScripts, EngineMoves}, prelude::{DefaultEngine, Battle, BattleData, BattleType, PlayerSettings, PlayerData, BattleAi}, pokedex::{Uninitializable, pokemon::owned::OwnedPokemon}};
use firecore_battle::pokedex::Dex;
use rand::{SeedableRng, prelude::SmallRng, RngCore};
use worldlib::character::player::PlayerCharacter;
use crate::saves::OwnedPlayer;

use crate::pokedex::{pokemon::Pokemon, moves::Move, item::Item};

use super::{GameBattleWrapper};

use firecore_battle_gui::{BattlePlayerGui, context::BattleGuiData, transition::TransitionState, pokedex::engine::utils::Reset};

pub mod transitions;

use transitions::managers::{
	transition::BattleScreenTransitionManager,
	closer::BattleCloserManager,
};

pub struct BattleManager<P: Deref<Target = Pokemon> + Clone, M: Deref<Target = Move> + Clone, I: Deref<Target = Item> + Clone> {

	state: BattleManagerState,
	
    engine: DefaultEngine,
	
	battle: Option<GameBattleWrapper<P, M, I>>,

	dex: Rc<PokedexClientData>,
	btl: BattleGuiData,
	
	transition: BattleScreenTransitionManager,
	closer: BattleCloserManager,

	player: BattlePlayerGui<BattleId, P, M, I>,

	seed: u64,

	random: SmallRng,

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
	
	pub fn new(ctx: &mut Context, btl: BattleGuiData, dex: Rc<PokedexClientData>, party: Rc<PartyGui>, bag: Rc<BagGui>, data: (EngineMoves, MoveScripts)) -> Self {

        let mut engine = DefaultEngine::new::<BattleId, SmallRng>();

        engine.moves = data.0;

        engine.scripting.moves = data.1;
		
		Self {

			state: BattleManagerState::default(),

			battle: None,

			transition: BattleScreenTransitionManager::new(ctx),
			closer: BattleCloserManager::default(),

			player: BattlePlayerGui::new(ctx, &btl, party, bag),

			engine,

			seed: 0,

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
		self.player.reset();
		(!(
			save.party.is_empty() || 
			entry.party.is_empty() ||
			// Checks if player has any pokemon in party that aren't fainted (temporary)
			!save.party.iter().any(|pokemon| !pokemon.fainted())
		)).then(|| {

				if let Some(trainer) = entry.trainer.as_ref() {
					let mut sprites: firecore_battle_gui::pokedex::engine::utils::HashMap<_, _> = Default::default();
					sprites.insert(entry.id, trainer.sprite);
					self.player.set_next_groups(sprites);
				}

				let player = PlayerData {
					id: BattleId::Player,
					name: Some(save.character.name.clone()),
					party: save
						.party
						.iter()
						.cloned()
						.map(OwnedPokemon::uninit)
						.collect(),
					settings: PlayerSettings { gains_exp: true },
					endpoint: Box::new(self.player.endpoint().clone()),
				};
		
				let ai = BattleAi::new(SmallRng::seed_from_u64(self.random.next_u64()));
		
				let ai_player = PlayerData {
					id: entry.id,
					name: entry.trainer.as_ref().map(|trainer| trainer.name.clone()),
					party: entry.party,
					settings: PlayerSettings { gains_exp: false },
					endpoint: Box::new(ai.endpoint().clone()),
				};
		
				self.battle = Some(GameBattleWrapper {
					battle: Battle::new(
						BattleData {
							type_: entry
								.trainer
								.as_ref()
								.map(|trainer| {
									if trainer.badge.is_some() {
										BattleType::GymLeader
									} else {
										BattleType::Trainer
									}
								})
								.unwrap_or(BattleType::Wild),
						},
						&mut self.random,
						entry.active,
						pokedex,
						movedex,
						itemdex,
						std::iter::once(player).chain(std::iter::once(ai_player)),
					),
					trainer: entry.trainer,
					ai,
				});
			}
		);
		self.battle.is_some()
	}

	pub fn update<'d>(&mut self, ctx: &mut Context,
		pokedex: &'d dyn Dex<'d, Pokemon, P>,
	movedex: &'d dyn Dex<'d, Move, M>,
	itemdex: &'d dyn Dex<'d, Item, I>, delta: f32, save: &mut OwnedPlayer<P, M, I>) {
		if ctx.debug() && is_key_pressed(ctx, Key::F1) { // exit shortcut
  				self.end();
  				return;
  			}
		if let Some(battle) = &mut self.battle {
			self.player.process(&mut self.random, &self.dex, &self.btl,
				pokedex,
				movedex,
				itemdex);
			match self.state {
				BattleManagerState::Begin => {
					self.player.gui.reset();
					self.state = BattleManagerState::Transition;
					self.transition.state = TransitionState::Begin;

					battle.battle.begin();

					// self.player.on_begin(dex);

					self.update(ctx, pokedex, movedex, itemdex, delta, save);
				},
				BattleManagerState::Transition => match self.transition.state {
					TransitionState::Begin => {
						for p in self.player.remotes.values() {
							println!("{}", p.player.pokemon.len());
						}
						self.transition.begin(ctx, self.player.local.as_ref().unwrap().data.type_, &battle.trainer);
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

					if !battle.battle.finished() {
						battle.update(&mut self.random, &mut self.engine, delta, pokedex, movedex, itemdex);
					}

					self.player.update(ctx, &self.dex, pokedex, movedex, itemdex, delta, &mut save.bag);

					if let Some(winner) = self.player.winner().flatten() {
						self.state = BattleManagerState::Closer(*winner);
					}

				},
				BattleManagerState::Closer(winner) => match self.closer.state {
					TransitionState::Begin => {
						let local = self.player.local.as_ref().unwrap();
						self.closer.begin(&self.dex, local.data.type_, local.player.id(), local.player.name(), Some(&winner), battle.trainer.as_ref(), &mut self.player.gui.text);
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
	}

	pub fn winner(&self) -> Option<&BattleId> {
		self.battle.as_ref().map(|b| b.battle.winner()).flatten()
	}

	pub fn update_data(&mut self, winner: bool, player: &mut PlayerCharacter) -> bool {
		self.battle.as_mut().map(|battle| {
			let trainer = battle.trainer.is_some();

        if winner {
            if let Some(trainer) = battle.trainer.take() {
                player.worth += trainer.worth as u32;
                if let Some(badge) = trainer.badge {
                    player.world.badges.insert(badge);
                }
            }
        }

        trainer
		}).unwrap_or_default()
	}

	pub fn world_active(&self) -> bool {
		matches!(self.state, BattleManagerState::Transition) || self.closer.world_active()		
	}

	pub fn seed(&mut self, seed: u64) {
		self.seed = seed;
		self.random = SmallRng::seed_from_u64(seed);
	}

	pub fn end(&mut self) {
		self.finished = true;
		match self.state {
			BattleManagerState::Begin => (),
			BattleManagerState::Transition => self.transition.state = TransitionState::Begin,
			BattleManagerState::Battle => self.battle = None,
			BattleManagerState::Closer(..) => self.closer.state = TransitionState::Begin,
		}
	}

	pub fn draw(&self, ctx: &mut Context, save: &OwnedPlayer<P, M, I>) {
        if self.battle.is_some() {
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
							for active in self.player.local.as_ref().unwrap().renderer.iter() {
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