use std::rc::Rc;

use crate::{
	game::GameState,
	deps::rhai::Engine,
	storage::{data_mut, player::PlayerSave},
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	pokedex::{
		pokemon::instance::BorrowedPokemon,
		moves::target::PlayerId,
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
	GameBattle,
	state::BattleState,
	client_state::{
		BattleManagerState,
		TransitionState,
	},
	pokemon::BattlePlayer,
	ui::{
		transitions::managers::{
			transition::BattleScreenTransitionManager,
			opener::BattleOpenerManager,
			introduction::BattleIntroductionManager,
			closer::BattleCloserManager,
		}
	},
	gui::guiref::BattlePlayerGuiRef,
};

pub struct BattleManager {

	state: BattleManagerState,
	
	battle: GameBattle,
	
	transition: BattleScreenTransitionManager,
	opener: BattleOpenerManager,
	introduction: BattleIntroductionManager,
	closer: BattleCloserManager,

	player: BattlePlayerGuiRef,

	pub finished: bool,
	
}

impl BattleManager {
	
	pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> BattleManager {
		
		BattleManager {

			state: BattleManagerState::default(),

			battle: GameBattle::new(crate::pokedex::moves::usage::script::engine()),

			transition: BattleScreenTransitionManager::new(ctx),
			opener: BattleOpenerManager::new(ctx),
			introduction: BattleIntroductionManager::new(ctx),
			closer: BattleCloserManager::default(),

			player: BattlePlayerGuiRef::new(ctx, party, bag),

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
						&data.name, 
						data.party.iter_mut().map(|instance| BorrowedPokemon::Borrowed(instance)).collect(), 
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
		if let Some(data) = self.battle.battle.data() {
			match self.state {
				BattleManagerState::Begin => {
					self.player.get().gui.reset();
					self.state = BattleManagerState::Transition;
					self.transition.state = TransitionState::Begin;

					self.battle.battle.begin();

					self.player.get().on_begin(ctx);

					self.update(ctx, delta, input_lock);
				},
				BattleManagerState::Transition => match self.transition.state {
					TransitionState::Begin => {
						self.transition.begin(ctx, data.type_, &self.battle.trainer);
						self.update(ctx, delta, input_lock);
					},
					TransitionState::Run => self.transition.update(ctx, delta),
					TransitionState::End => {
						self.transition.end();
						self.state = BattleManagerState::Opener;
						self.update(ctx, delta, input_lock);
					}
				}
				BattleManagerState::Opener => match self.opener.state {
					TransitionState::Begin => {
						self.opener.begin(data.type_, self.battle.trainer.as_ref());
						self.update(ctx, delta, input_lock);
					}
					TransitionState::Run => self.opener.update(delta),
					TransitionState::End => {
						self.opener.end();
						self.state = BattleManagerState::Introduction;
						self.update(ctx, delta, input_lock);
					}
				}
				BattleManagerState::Introduction => match self.introduction.state {
					TransitionState::Begin => {
						{
							let mut player = self.player.get();
							self.introduction.begin(data.type_, self.battle.trainer.as_ref(), &mut player);
						}
						self.update(ctx, delta, input_lock);
					}
					TransitionState::Run => {
						let mut player = self.player.get();
						self.introduction.update(ctx, delta, &mut player);
					}
					TransitionState::End => {
						self.introduction.end(&mut self.player.get().gui.text);
						self.state = BattleManagerState::Battle;
						self.update(ctx, delta, input_lock);
					}
				}
				BattleManagerState::Battle => match self.battle.battle.state().unwrap() {
					BattleState::End(id) => self.state = BattleManagerState::Closer(*id),
					_ => {

						let player = self.player.get();

						player.update(ctx, delta);
						player.gui.bounce.update(delta);

						self.battle.battle.update();
					}
				},
				BattleManagerState::Closer(winner) => match self.closer.state {
					TransitionState::Begin => {
						self.closer.begin(data.type_, Some(&winner), self.battle.trainer.as_ref(), &mut self.player.get().gui.text);
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

	pub fn update_data(&mut self, player_save: &mut PlayerSave) -> Option<(PlayerId, bool)> {
		self.battle.update_data(player_save)
	}

	pub fn world_active(&self) -> bool {
		matches!(self.state, BattleManagerState::Transition) || self.closer.world_active()		
	}

	pub fn end(&mut self) {
		self.finished = true;
		match self.state {
			BattleManagerState::Begin => (),
			BattleManagerState::Transition => self.transition.state = TransitionState::Begin,
			BattleManagerState::Opener => self.opener.state = TransitionState::Begin,
			BattleManagerState::Introduction => self.introduction.state = TransitionState::Begin,
			BattleManagerState::Battle => self.battle.battle.end(),
			BattleManagerState::Closer(..) => self.closer.state = TransitionState::Begin,
		}
	}
	
}

impl GameState for BattleManager {
    fn process(&mut self, mut result: crate::game::CommandResult) {
		use deps::log::warn;
		if let Some(battle) = self.battle.battle.as_mut() {
			match result.command {
				"end" => self.end(),
				"faint" => if let Some(team) = result.args.next() {
					if let Some(index) = result.args.next().map(|index| index.parse::<usize>().ok()).flatten() {
						match team {
							"player" => if let Some(active) = battle.player1.active_mut(index) {
								active.current_hp = 0;
								// battle.battle.player1.client.add_unknown(index, unknown)
							}
							"opponent" => if let Some(active) = battle.player2.active_mut(index) {
								active.current_hp = 0;
							}
							_ => warn!("Unknown team!"),
						}
					} else {
						warn!("Unknown index!")
					}
				} else {
					warn!("Provide arguments team and index")
				}
				"heal" => if let Some(team) = result.args.next() {
					if let Some(index) = result.args.next().map(|index| index.parse::<usize>().ok()).flatten() {
						match team {
							"player" => if let Some(active) = battle.player1.active_mut(index) {
								active.current_hp = active.max_hp();
							}
							"opponent" => if let Some(active) = battle.player2.active_mut(index) {
								active.current_hp = active.max_hp();
							}
							_ => warn!("Unknown team!"),
						}
					} else {
						warn!("Unknown index!")
					}
				} else {
					warn!("Provide arguments team and index")
				}
				_ => warn!("Unknown command"),
			}
		}
    }

    fn draw(&self, ctx: &mut deps::tetra::Context) {
        if self.battle.battle.is_some() {
			match self.state {
				BattleManagerState::Begin => (),
			    BattleManagerState::Transition => self.transition.draw(ctx),
			    BattleManagerState::Opener => {
					self.player.get().gui.background.draw(ctx, self.opener.offset());
					self.opener.draw_below_panel(ctx, &self.player.get().player.renderer, &self.player.get().opponent.renderer);
					self.player.get().gui.draw_panel(ctx);
					self.opener.draw(ctx);
				}
			    BattleManagerState::Introduction => {
					self.player.get().gui.background.draw(ctx, 0.0);
					self.introduction.draw(ctx, &self.player.get().player.renderer, &self.player.get().opponent.renderer);
					self.player.get().gui.draw_panel(ctx);
					self.player.get().gui.text.draw(ctx);
				}
			    BattleManagerState::Battle => self.player.get().draw(ctx),
			    BattleManagerState::Closer(..) => {
					if self.closer.state != TransitionState::End {
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