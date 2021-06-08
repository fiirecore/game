use std::rc::Rc;

use crate::{
	game::GameState,
	deps::rhai::Engine,
	storage::{data_mut, player::{PlayerSave, PlayerId}},
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	pokedex::{
		pokemon::instance::BorrowedPokemon,
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
	pokemon::BattleParty,
	ui::{
		transitions::managers::{
			transition::BattleScreenTransitionManager,
			opener::BattleOpenerManager,
			introduction::BattleIntroductionManager,
			closer::BattleCloserManager,
		}
	},
	gui::BattlePlayerGuiRef,
};

pub struct BattleManager {

	state: BattleManagerState,
	
	battle: Option<GameBattle>,
	
	transition: BattleScreenTransitionManager,
	opener: BattleOpenerManager,
	introduction: BattleIntroductionManager,
	closer: BattleCloserManager,

	engine: Engine,

	player: BattlePlayerGuiRef,

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

			player: BattlePlayerGuiRef::new(ctx, party, bag),

			finished: false,

		}
		
	}

	pub fn battle(&mut self, entry: BattleEntry) -> bool { // add battle type parameter
		self.finished = false;
		self.state = BattleManagerState::default();
		let data = data_mut();
		let player = &mut data.party;
		self.battle = (!(
			player.is_empty() || 
			entry.party.is_empty() ||
			// Checks if player has any pokemon in party that aren't fainted (temporary)
			!player.iter().any(|pokemon| !pokemon.fainted())
		)).then(|| {
				let data = data_mut();
				GameBattle::new(
				BattleParty::new(
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
		self.battle.is_some()
	}

	pub fn update(&mut self, ctx: &mut Context, delta: f32, input_lock: bool) {
		if is_debug() {
			if debug_pressed(ctx, DebugBind::F1) { // exit shortcut
				self.end();
				return;
			}
		}
		if let Some(battle) = self.battle.as_mut() {
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
						self.transition.begin(ctx, battle.battle.battle_type(), &battle.trainer);
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
						self.opener.begin(battle.battle.battle_type(), battle.trainer.as_ref());
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
							self.introduction.begin(battle.battle.battle_type(), battle.trainer.as_ref(), &mut player);
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
				BattleManagerState::Battle => match battle.battle.state {
					BattleState::End => self.state = BattleManagerState::Closer,
					_ => {

						let mut player = self.player.get();

						player.update(ctx, delta);
						player.gui.bounce.update(delta);

						battle.battle.update(&mut self.engine);
					}
				},
				BattleManagerState::Closer => match self.closer.state {
					TransitionState::Begin => {
						self.closer.begin(battle.battle.battle_type(), battle.battle.winner.as_ref(), battle.trainer.as_ref(), &mut self.player.get().gui.text);
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
		self.battle.take().map(|battle| battle.update_data(player_save)).flatten()
	}

	pub fn world_active(&self) -> bool {
		self.state == BattleManagerState::Transition || self.closer.world_active()		
	}

	pub fn end(&mut self) {
		self.finished = true;
		match self.state {
			BattleManagerState::Begin => (),
			BattleManagerState::Transition => self.transition.state = TransitionState::Begin,
			BattleManagerState::Opener => self.opener.state = TransitionState::Begin,
			BattleManagerState::Introduction => self.introduction.state = TransitionState::Begin,
			BattleManagerState::Battle => {
				if let Some(battle) = self.battle.as_mut() {
					battle.battle.state = BattleState::End;
					battle.battle.update(&mut self.engine);
				}
			},
			BattleManagerState::Closer => self.closer.state = TransitionState::Begin,
		}
		if let Some(battle) = self.battle.as_mut() {
			battle.battle.winner = Some(data_mut().id);
		}
	}
	
}

impl GameState for BattleManager {
    fn process(&mut self, command: crate::game::CommandResult) {
        match command.command.as_str() {
			"end" => self.end(),
			_ => (),
		}
    }

    fn draw(&self, ctx: &mut deps::tetra::Context) {
        if self.battle.is_some() {
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
			    BattleManagerState::Closer => {
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