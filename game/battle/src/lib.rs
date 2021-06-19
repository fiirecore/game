// #![feature(map_into_keys_values)] // for move queue fn

extern crate firecore_dependencies as deps;
extern crate firecore_pokedex as pokedex;

use {
	deps::{
		random::{Random, RandomState, GLOBAL_STATE},
		rhai::Engine,
        log::{info, warn, debug}, 
	},
	pokedex::{
		types::Effective,
		pokemon::{
			Health,
			Experience,
			stat::{StatType, BaseStat},
			instance::PokemonInstance,
		},
		moves::{
			Priority,
			instance::MoveInstance,
			usage::MoveResult,
			target::MoveTargetInstance,
		},
		item::ItemUseType,
	}, 
};

use crate::{
	state::BattleState,
	data::*,
	pokemon::*,
	message::{ServerMessage, ClientMessage},
};

pub mod state;
pub mod data;

pub mod pokemon;

pub mod client;
pub mod message;

pub static BATTLE_RANDOM: Random = Random::new(RandomState::Static(&GLOBAL_STATE));

pub struct Battle {
	battle: Option<(BattleState, BattleHost)>,
	engine: Engine,
}

impl Battle {
	pub fn new(engine: Engine) -> Self {
		Self {
			engine,
			battle: None,
		}
	}
	pub fn battle(&mut self, host: BattleHost) {
		self.battle = Some((BattleState::default(), host));
	}
	pub fn is_some(&self) -> bool {
		self.battle.is_some()
	}
	#[deprecated]
	pub fn begin(&mut self) {
		if let Some((state, battle)) = &mut self.battle {
			battle.begin(state);
		}
	}
	pub fn update(&mut self) {
		if let Some((state, battle)) = &mut self.battle {
			battle.update(state, &self.engine);
		}
	}
	pub fn take(&mut self) -> Option<BattleHost> {
		self.battle.take().map(|(_, b)| b)
	}
	pub fn end(&mut self) {
		if let Some((s, _)) = self.battle.as_mut() {
			*s = BattleState::End(false, "none".parse().unwrap());
		}
	}
}


// rename this
pub struct BattleHost { ////////////// if using hashmap, only remaining player should be winner

	data: BattleData,

	// #[deprecated(note = "use hashmap")]
	pub player1: BattlePlayer,
	pub player2: BattlePlayer,

	// players: deps::hash::HashMap<TrainerId, UnsafeCell<BattlePlayer>>,
	
}

impl BattleHost {

    pub fn new(data: BattleData, player1: BattlePlayer, player2: BattlePlayer) -> Self {
        Self {
            data,
            player1,
            player2,
        }
    }

    pub fn data(&self) -> &BattleData {
        &self.data
    }

	#[deprecated]
	fn begin(&mut self, state: &mut BattleState) {

		self.player1.client.send(ServerMessage::User(self.data.clone(), self.player1.as_known()));
		self.player2.client.send(ServerMessage::User(self.data.clone(), self.player2.as_known()));

		self.player1.reveal_active();
		self.player2.reveal_active();

		self.player1.client.send(ServerMessage::Opponents(self.player2.as_unknown()));
		self.player2.client.send(ServerMessage::Opponents(self.player1.as_unknown()));
		
		*state = BattleState::SELECTING_START;
	}

	#[deprecated(note = "fix")]
	fn _receive(&mut self, state: &mut BattleState, first: bool) {
		let (user, other) = match first {
			true => (&mut self.player1, &mut self.player2),
			false => (&mut self.player2, &mut self.player1),
		};
		while let Some(message) = user.client.receive() {
			match message {
				ClientMessage::Move(active, bmove) => {
					if let Some(pokemon) = user.active.get_mut(active) {
						match pokemon {
							ActivePokemon::Some(_, queued_move) => {
								*queued_move = Some(bmove);
								// party.client.confirm_move(active);
							},
							_ => warn!("Party {} could not add move #{:?} to pokemon #{}", user.id, bmove, active),
						}
					}
				},
				ClientMessage::FaintReplace(active, index) => {
					debug!("faint replace: {} replacing {} with {}", user.name(), active, index);
					if !user.active_contains(index) {
						if if let Some(pokemon) = user.pokemon.get(index) {
							if !pokemon.pokemon.value().fainted() {
								user.active.set(active, ActivePokemon::Some(index, None));
								if let Some(pokemon) = user.know(index) {
									other.client.send(ServerMessage::AddUnknown(index, pokemon));
								}
								other.client.send(ServerMessage::FaintReplace(ActivePokemonIndex { team: user.id, index: active }, Some(index)));
								false
							} else {
								true
							}
							// party.client.confirm_replace(active, index);
						} else {
							true
						} {
							warn!("Player {} tried to replace a fainted pokemon with an missing/fainted pokemon", user.name());
						}
					} else {
						warn!("Player {} tried to replace a pokemon with a pokemon that is already active.", user.name());
					}
				},
				ClientMessage::RequestPokemon(request) => {
					if let Some(pokemon) = other.pokemon.get(request) {
						if matches!(self.data.type_, BattleType::Wild) || pokemon.requestable || (pokemon.pokemon.value().fainted() && pokemon.known) {
							user.client.send(ServerMessage::PokemonRequest(request, pokemon.pokemon.cloned()));
						}
					}
				},
				ClientMessage::Forfeit => *state = BattleState::End(false, other.id),
				ClientMessage::FinishedTurnQueue => (),
			}
		}
	}

	pub fn update(&mut self, state: &mut BattleState, engine: &Engine) {

		self._receive(state, true);
		self._receive(state, false);

		match state {
			
			BattleState::Setup => self.begin(state),

			BattleState::StartWait => (),
			// Select pokemon moves / items / party switches

		    BattleState::Selecting(started) => match *started {
				false => {
					self.player1.client.send(ServerMessage::StartSelecting);
					self.player2.client.send(ServerMessage::StartSelecting);
					*started = true;
				}
				true => if self.player1.ready_to_move() && self.player2.ready_to_move() {
					*state = BattleState::MOVE_START;
				}
			},
		    BattleState::Moving(waiting) => match waiting {
				false => {

					let queue = Self::create_move_queue(&mut self.player1, &mut self.player2);

					let player_queue = self.create_client_queue(
						engine, 
						queue,
					);

					// end queue calculations
	
					self.player2.client.send(ServerMessage::TurnQueue(std::borrow::Cow::Borrowed(&player_queue)));
					self.player1.client.send(ServerMessage::TurnQueue(std::borrow::Cow::Owned(player_queue)));
					
					self.player1.client.send(ServerMessage::AskFinishedTurnQueue);
					self.player2.client.send(ServerMessage::AskFinishedTurnQueue);

					*waiting = true;

				}
				true => {
					if self.player1.client.finished_turn() && self.player2.client.finished_turn() {
						if self.player2.all_fainted() {
							debug!("{} won!", self.player1.name());
							*state = BattleState::End(false, self.player1.id);
						} else if self.player1.all_fainted() {
							debug!("{} won!", self.player2.name());
							*state = BattleState::End(false, self.player2.id);
						} else if !self.player1.needs_replace() && !self.player2.needs_replace() {
							*state = BattleState::SELECTING_START;
						}
					}
				}
			},
			BattleState::End(started, winner) => if !*started {
				self.player1.client.send(ServerMessage::Winner(*winner));
				self.player2.client.send(ServerMessage::Winner(*winner));
				*started = true;
			}
		}
	}

	pub fn create_client_queue(&mut self, engine: &Engine, queue: Vec<ActionInstance<BattleMove>>) -> Vec<ActionInstance<BattleClientAction>> {
		let mut player_queue = Vec::with_capacity(queue.len());

		for instance in queue {
			let (user, other) = match instance.pokemon.team == self.player1.id {
				true => (&mut self.player1, &mut self.player2),
				false => (&mut self.player2, &mut self.player1),
			};
			if user.active(instance.pokemon.index).is_some() {
				match instance.action {
					BattleMove::Move(move_index, targets) => {
						
						let userp = user.active(instance.pokemon.index).unwrap();

						let targets = targets.into_iter().flat_map(|target| match target {
							MoveTargetInstance::Opponent(index) => other.active(index),
							MoveTargetInstance::Team(index) => user.active(index),
							MoveTargetInstance::User => Some(userp),
						}.map(|i| (target, i))).map(|(target, pokemon)| pokedex::moves::usage::pokemon::PokemonTarget {
							pokemon,
							active: target,
						}).collect();

						let turn = userp.use_own_move(engine, move_index, targets);

						let mut target_results = Vec::with_capacity(turn.results.len());

						for (target_instance, result) in turn.results {

							let mut client_results = Vec::new();

							let (user, other) = match instance.pokemon.team == self.player1.id {
								true => (&mut self.player1, &mut self.player2),
								false => (&mut self.player2, &mut self.player1),
							};
							
							{

								let user = user.active_mut(instance.pokemon.index).unwrap();

								if let Some(result) = &result {
									match result {
										MoveResult::Drain(_, heal, _) => {
											user.current_hp = (user.current_hp + *heal).min(user.base.hp());
											client_results.push(BattleClientMove::UserHP(user.current_hp as f32 / user.max_hp() as f32));
										}
										_ => (),
									}
								}

							}

							match result {
								Some(result) => {

									let (target_party, index) = match target_instance {
										MoveTargetInstance::Opponent(index) => (other, index),
										MoveTargetInstance::User => (user, instance.pokemon.index),
										MoveTargetInstance::Team(index) => (user, index),
									};

									let target = target_party.active_mut(index).unwrap();

									fn on_damage(pokemon: &mut PokemonInstance, results: &mut Vec<BattleClientMove>, damage: Health, effective: Effective) {
										pokemon.current_hp = pokemon.current_hp.saturating_sub(damage);
										results.push(BattleClientMove::TargetHP(pokemon.hp() as f32 / pokemon.max_hp() as f32));
										if effective != Effective::Effective {
											results.push(BattleClientMove::Effective(effective));
										}
									}

									match result {
										MoveResult::Damage(damage, effective) => {
											on_damage(target, &mut client_results, damage, effective);
										},
										MoveResult::Status(effect) => {
											target.status = Some(effect);
										},
										MoveResult::Drain(damage, _, effective) => {
											on_damage(target, &mut client_results, damage, effective);
										},
										MoveResult::StatStage(stat, stage) => {
											target.base.change_stage(stat, stage);
											client_results.push(BattleClientMove::StatStage(stat, stage));
										}
										MoveResult::Todo => {
											client_results.push(BattleClientMove::Fail);
										},
									}

									if target.fainted() {
										
										let experience = (target.exp_from() as f32 * match matches!(self.data.type_, BattleType::Wild) {
											true => 1.5,
											false => 1.0,
										} * 7.0) as Experience;
										
										client_results.push(BattleClientMove::Faint(ActivePokemonIndex { team: target_party.id, index }));

										let inactive = target_party.any_inactive();
										if let Some(active) = target_party.active.get_mut(index) {
											if inactive {
												*active = ActivePokemon::ToReplace;
											} else {
												*active = ActivePokemon::None;
											}
										}

										let user = match instance.pokemon.team == self.player1.id {
											true => (&mut self.player1),
											false => (&mut self.player2),
										};

										client_results.push(BattleClientMove::GainExp(experience));// .send(ServerMessage::GainExp(instance.pokemon.index, experience));

										let pokemon = user.active_mut(instance.pokemon.index).unwrap();

										let level = pokemon.level;

										pokemon.add_exp(experience);

										if !pokemon.moves.is_full() {
											let mut moves = pokemon.moves_from(level..pokemon.level);
											while let Some(moves) = moves.pop() {
												if let Err(_) = pokemon.moves.try_push(MoveInstance::new(moves)) {
													break;
												}
											}
										}

									}
									
								}
								None => client_results.push(BattleClientMove::Miss),
							}

							target_results.push((target_instance, client_results));

						}

						player_queue.push(BattleClientActionInstance {
							pokemon: instance.pokemon,
							action: BattleClientAction::Move(turn.pokemon_move, target_results),
						});

					}
					BattleMove::UseItem(item, target) => {
						if match &item.value().usage {
							ItemUseType::Script(script) => {
								user.active_mut(instance.pokemon.index).unwrap().execute_item_script(script);
								true
							},
							ItemUseType::Pokeball => {
								match self.data.type_ {
									BattleType::Wild => {
										let (target, active) = match target {
											MoveTargetInstance::Opponent(index) => (other, index),
											_ => unreachable!(),
										};
										if let Some(active) = target.active.get_mut(active) {
											let active = std::mem::replace(active, ActivePokemon::None);
											if let Some(index) = active.index() {
												if target.pokemon.len() > index {
													let pokemon = target.pokemon.remove(index);
													user.client.send(ServerMessage::PokemonRequest(index, pokemon.pokemon.owned()));
												}
											}
										}
										true
									}
									_ => {
										info!("Cannot use pokeballs in trainer battles!");
										false
									},
								}
							},
							ItemUseType::None => true,
						} {
							player_queue.push(BattleClientActionInstance {
								pokemon: instance.pokemon,
								action: BattleClientAction::UseItem(item, target),
							});
						}
					}
					BattleMove::Switch(new) => {
						user.replace(instance.pokemon.index, Some(new));
						let unknown = user.active_index(instance.pokemon.index).map(|index| user.know(index)).flatten();
						player_queue.push(BattleClientActionInstance {
							pokemon: instance.pokemon,
							action: BattleClientAction::Switch(new, unknown),
						});
					}
				}
			}
		}
		player_queue
	}

	pub fn create_move_queue(player1: &mut BattlePlayer, player2: &mut BattlePlayer) -> Vec<BattleMoveInstance> {

		use std::cmp::Reverse;

		#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
		enum MovePriority {
			First(ActivePokemonIndex),
			Second(Reverse<Priority>, Reverse<BaseStat>, ActivePokemonIndex), // priority, speed, pokemon <- fix last, player always goes first
		}

		fn insert(map: &mut std::collections::BTreeMap<MovePriority, BattleMoveInstance>, party: &mut BattlePlayer) {
			for index in 0..party.active.len() {
				if let Some(pokemon) = party.active.get_mut(index) {
					if let Some(action) = pokemon.use_move() {
						if let Some(instance) = party.active(index) {
							let pokemon = ActivePokemonIndex { team: party.id, index };
							map.insert(
								match action {
									BattleMove::Move(index, ..) => MovePriority::Second(Reverse(instance.moves[index].move_ref.value().priority), Reverse(instance.base.get(StatType::Speed)), pokemon),
									_ => MovePriority::First(pokemon),
								}, 
								BattleMoveInstance { pokemon, action }
							);
						}
					}
				}
			}
		}

		let mut map = std::collections::BTreeMap::new();

		insert(&mut map, player1);
		insert(&mut map, player2);

		map.into_iter().map(|(_, i)| i).collect() // into_values

	}
	
}