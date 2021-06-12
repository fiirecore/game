#[allow(deprecated, deprecated_in_future, useless_deprecated)]
// #![feature(map_into_keys_values)] // for move queue fn

extern crate firecore_dependencies as deps;
extern crate firecore_pokedex as pokedex;

use std::rc::Rc;

use serde::{Deserialize, Serialize};

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
			stat::StatType,
			instance::PokemonInstance,
		},
		moves::{
			usage::MoveResult,
			target::{MoveTargetInstance, PlayerId},
		},
		item::ItemUseType,
	}, 
};

use crate::{
	state::{
		BattleState,
		MoveState,
	},
	pokemon::{
		ActivePokemon,
		BattleParty,
		ActivePokemonIndex,
		BattleMoveInstance,
		BattleMove,
		BattleClientMove,
		BattleClientAction,
		BattleClientActionInstance,
		view::UnknownPokemon,
	},
	message::{ServerMessage, ClientMessage},
};

pub mod state;

pub mod pokemon;

pub mod client;
pub mod message;

// #[deprecated(note = "look into replacing this")]
pub static BATTLE_RANDOM: Random = Random::new(RandomState::Static(&GLOBAL_STATE));

#[deprecated(note = "unused, no functions")]
pub struct BattleWrapper {
	pub battle: Option<Battle>,
	engine: Engine,
}

pub struct Battle {
	
	// #[deprecated(note = "look into replacing this")]
	pub state: BattleState,


	engine: Rc<Engine>,


	battle_type: BattleType,

	// add battle data (weather, etc) here.

	#[deprecated(note = "look into replacing this")]
	pub winner: Option<PlayerId>, // if using hashmap, only remaining player should be winner
	
	#[deprecated(note = "use hashmap")]
	pub player1: BattleParty,
	pub player2: BattleParty,

	// players: deps::hash::HashMap<PlayerId, UnsafeCell<BattleParty>>,
	
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BattleType { // move somewhere else

    Wild,
    Trainer,
    GymLeader,

}

impl Default for BattleType {
    fn default() -> Self {
        Self::Wild
    }
}

impl Battle {

    pub fn new(engine: Rc<Engine>, battle_type: BattleType, player1: BattleParty, player2: BattleParty) -> Self {
        Self {
            state: Default::default(),
			engine,
            battle_type,
            winner: None,
            player1,
            player2,
        }
    }

    pub fn battle_type(&self) -> BattleType {
        self.battle_type
    }

	#[deprecated]
	pub fn begin(&mut self) {

		self.player1.client.send(ServerMessage::User(self.battle_type, self.player1.as_known()));
		self.player2.client.send(ServerMessage::User(self.battle_type, self.player2.as_known()));

		self.player1.client.send(ServerMessage::Opponents(self.player2.as_unknown()));
		self.player2.client.send(ServerMessage::Opponents(self.player1.as_unknown()));
		
		self.state = BattleState::SELECTING_START;
	}

	#[deprecated(note = "fix")]
	fn _receive(&mut self, first: bool) {
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
					debug!("received faint replace: {}: {} with {}", user.id, active, index);
					if if let Some(pokemon) = user.pokemon.get(index) {
						if !pokemon.value().fainted() {
							user.active.set(active, ActivePokemon::Some(index, None));
							other.client.send(ServerMessage::FaintReplace(user.id, active, Some(index)));
							debug!("replaced fainted pokemon.");
							false					
						} else {
							true
						}
						// party.client.confirm_replace(active, index);
					} else {
						true
					} {
						warn!("Player {} tried to replace a fainted pokemon with an missing/fainted pokemon", user.name);
					}
				}
				ClientMessage::FinishedTurnQueue => (),
				ClientMessage::Forfeit => {
					self.winner = Some(other.id);
				},
			}
		}
	}

	pub fn update(&mut self) {

		self._receive(true);
		self._receive(false);

		match &mut self.state {

			BattleState::StartWait => (),

			// Select pokemon moves / items / party switches

		    BattleState::Selecting(started) => match *started {
				false => {
					self.player1.client.send(ServerMessage::StartSelecting);
					self.player2.client.send(ServerMessage::StartSelecting);
					*started = true;
				}
				true => {

					if self.player1.ready_to_move() && self.player2.ready_to_move() {

						#[deprecated(note = "temporary")] {
							for (i, p) in self.player1.pokemon.iter().enumerate() {
								self.player2.client.send(ServerMessage::AddUnknown(self.player1.id, i, UnknownPokemon::new(p.value())));
							}
							for (i, p) in self.player2.pokemon.iter().enumerate() {
								self.player1.client.send(ServerMessage::AddUnknown(self.player2.id, i, UnknownPokemon::new(p.value())));
							}
						}

						self.state = BattleState::MOVE_START;
					} else if self.winner.is_some() {
						self.state = BattleState::End;
					}
					
				}
			},
		    BattleState::Moving(move_state) => {
				match move_state {
					MoveState::Start => *move_state = MoveState::SetupPokemon,
					MoveState::SetupPokemon => {
						// Queue pokemon moves					
						*move_state = MoveState::Pokemon(Self::move_queue(&mut self.player1, &mut self.player2));
					},
					MoveState::Pokemon(queue) => {

						let mut player_queue = Vec::with_capacity(queue.len());

						for instance in queue {
							let (user, other) = match instance.pokemon.team == self.player1.id {
								true => (&mut self.player1, &mut self.player2),
								false => (&mut self.player2, &mut self.player1),
							};
							if user.active(instance.pokemon.index).is_some() {
								match &mut instance.action {
									BattleMove::Move(move_index, targets) => {
										
										let userp = user.active(instance.pokemon.index).unwrap();

										let targets = targets.iter().flat_map(|target| match target {
											MoveTargetInstance::Opponent(team, index) => other.active(*index),
											MoveTargetInstance::Team(index) => user.active(*index),
											MoveTargetInstance::User => Some(userp),
										}.map(|i| (target, i))).map(|(target, pokemon)| pokedex::moves::usage::pokemon::PokemonTarget {
											pokemon,
											active: *target,
										}).collect();

										let turn = userp.use_own_move(self.engine.as_ref(), *move_index, targets);

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
														MoveTargetInstance::Opponent(team, index) => (other, index),
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

														let t =  match target_instance {
															MoveTargetInstance::Opponent(team, index) => ActivePokemonIndex { team, index },
															MoveTargetInstance::Team(index) => ActivePokemonIndex { team: instance.pokemon.team, index },
															MoveTargetInstance::User => ActivePokemonIndex { team: instance.pokemon.team, index: instance.pokemon.index },
														};
														
														client_results.push(BattleClientMove::Faint(t)); // queue.actions.push_front(BattleMoveInstance { pokemon, action: BattleAction::Faint(Some(target_instance)) });
														if let Some(active) = target_party.active.get_mut(index) {
															active.replace();
														}

													}
													
												}
												None => client_results.push(BattleClientMove::Miss), // ui::text::on_miss(text, user.active[instance.pokemon.active].pokemon.as_ref().unwrap()),
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
												match self.battle_type {
													BattleType::Wild => {

														let index = match target {
															MoveTargetInstance::Opponent(t, i) => *i,
															_ => unreachable!(),
														};

														// player_queue.push(BattleClientActionInstance {
														// 	pokemon: instance.pokemon,
														// 	action: BattleClientAction::Catch(index),
														// });

														todo!()

													},
													_ => info!("Cannot use pokeballs in trainer battles!"),
												}
												false
											},
											ItemUseType::None => true,
										} {
											debug!("to - do: test using items");
											player_queue.push(BattleClientActionInstance {
												pokemon: instance.pokemon,
												action: BattleClientAction::UseItem(*item, *target),
											});
										}
									}
									BattleMove::Switch(new) => {
										debug!("to - do: dont send unknown every time");
										user.replace(instance.pokemon.index, Some(*new));
										player_queue.push(BattleClientActionInstance {
											pokemon: instance.pokemon,
											action: BattleClientAction::Switch(*new, Some(UnknownPokemon::new(user.active(instance.pokemon.index).unwrap()))),
										});
									}
								}
							}
						}

						// end queue calculations

						self.player2.client.send(ServerMessage::TurnQueue(std::borrow::Cow::Borrowed(&player_queue)));
						self.player1.client.send(ServerMessage::TurnQueue(std::borrow::Cow::Owned(player_queue)));

						*move_state = MoveState::SetupPost;

					},
					MoveState::SetupPost => *move_state = MoveState::Post,
					MoveState::Post => *move_state = MoveState::SetupEnd,
					MoveState::SetupEnd => {
						self.player1.client.send(ServerMessage::AskFinishedTurnQueue);
						self.player2.client.send(ServerMessage::AskFinishedTurnQueue);
						*move_state = MoveState::End;
					},
					MoveState::End => {

						if self.player1.client.finished_turn() && self.player2.client.finished_turn() {
							if self.player2.all_fainted() {
								debug!("{} won!", self.player1.name);
								self.winner = Some(self.player1.id);
								self.state = BattleState::End;
							} else if self.player1.all_fainted() {
								debug!("{} won!", self.player2.name);
								self.winner = Some(self.player2.id);
								self.state = BattleState::End;
							} else if !self.player1.needs_replace() && !self.player2.needs_replace() {
								self.state = BattleState::SELECTING_START;
							}
						}
					},
				}
			},
    		BattleState::End => {
				// bag.despawn();
				// party_gui.despawn();
				// gui.panel.despawn();
			},
		}
	}

	pub fn move_queue(player1: &mut BattleParty, player2: &mut BattleParty) -> Vec<BattleMoveInstance> {

		use std::cmp::Reverse;

		#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
		enum Priority {
			First(ActivePokemonIndex),
			Second(Reverse<u8>, Reverse<u16>, ActivePokemonIndex), // priority, speed, pokemon <- fix last, player always goes first
		}

		fn insert(map: &mut std::collections::BTreeMap<Priority, BattleMoveInstance>, party: &mut BattleParty) {
			for index in 0..party.active.len() {
				if let Some(pokemon) = party.active.get_mut(index) {
					if let Some(action) = pokemon.use_move() {
						if let Some(pokemon) = party.active(index) {
							let index = ActivePokemonIndex { team: party.id, index };
							map.insert(
								match action {
									BattleMove::Move(..) => Priority::Second(Reverse(0), Reverse(pokemon.base.get(StatType::Speed)), index),
									_ => Priority::First(index),
								}, 
								BattleMoveInstance { pokemon: index, action }
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