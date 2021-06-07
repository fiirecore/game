// #![feature(map_into_keys_values)] // for move queue fn

use serde::{Deserialize, Serialize};

use crate::{
	deps::{
		Random,
		hash::HashMap,
		rhai::Engine,
	}, 
	log::{info, debug}, 
	pokedex::{
		types::Effective,
		pokemon::{
			instance::BorrowedPokemon,
			stat::StatType,
		},
		moves::{
			usage::{
				MoveResult,
			},
			target::MoveTargetInstance,
		},
		item::ItemUseType,
	}, 
	storage::player::{PlayerSave, PlayerId},
	battle_glue::{
		BattleEntry,
		BattleTrainerEntry,
	}, 
};

use crate::battle::{
	state::{
		BattleState,
		MoveState,
	},
	pokemon::{
		ActivePokemon,
		BattleParty,
		ActivePokemonIndex,
		BattleActionInstance,
		BattleMove,
		BattleClientMove,
		BattleClientAction,
		BattleClientActionInstance,
		view::PokemonUnknown,
	},
	client::{
		BattleClient,
		ai::BattlePlayerAi,
	},
};

pub mod state;
pub mod manager;

pub mod pokemon;
pub mod client;

pub mod ui;

pub static BATTLE_RANDOM: Random = Random::new();

pub struct Battle {
	
	pub state: BattleState,

	pub data: BattleData,
	
	player1: BattleParty,
	player2: BattleParty,
	
}

pub struct BattleData {
	battle_type: BattleType,
	trainer: Option<BattleTrainerEntry>,
	pub winner: Option<PlayerId>,
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
	
	pub fn new(player: BattleParty, entry: BattleEntry) -> Self {
		Self {
			data: BattleData {
				battle_type: entry.trainer.as_ref().map(|trainer| if trainer.gym_badge.is_some() { BattleType::GymLeader } else { BattleType::Trainer }).unwrap_or(BattleType::Wild),
				trainer: entry.trainer,
				winner: None,
			},
			player1: player,
			player2: BattleParty::new("opponent".parse().unwrap(), "opponent", entry.party.into_iter().map(|instance| BorrowedPokemon::Owned(instance)).collect(), Box::new(BattlePlayerAi::default()), entry.size),
			state: BattleState::default(),
		}
	}

	#[deprecated]
	pub fn begin(&mut self) {

		self.player1.client.user(self.data.battle_type, self.player1.as_known());
		self.player2.client.user(self.data.battle_type, self.player2.as_known());

		self.player1.client.opponents(self.player2.as_unknown());
		self.player2.client.opponents(self.player1.as_unknown());
		
		self.state = BattleState::SELECTING_START;
	}

	pub fn update(&mut self, engine: &mut Engine) {

		match &mut self.state {

			BattleState::StartWait => (),

			// Select pokemon moves / items / party switches

		    BattleState::Selecting(started) => match *started {
				false => {
					self.player1.client.start_select();
					self.player2.client.start_select();
					*started = true;
				}
				true => {

					fn fill_moves(party: &mut BattleParty, data: &mut BattleData, other: &PlayerId) {
						if !party.ready_to_move() {

							if party.client.should_forfeit() {
								data.winner = Some(*other);
							} else if let Some(mut moves) = party.client.wait_select() {
								for active in party.active.iter_mut().rev() {
									if let ActivePokemon::Some(_, queued_move) = active {
										*queued_move = moves.pop();
									}
								}
								// debug!("party with {:?} completed their moves.", party.pokemon(0));
								// *done = true;
							}
						}
					}

					fill_moves(&mut self.player1, &mut self.data, &self.player2.id);
					fill_moves(&mut self.player2, &mut self.data, &self.player1.id);

					if self.player1.ready_to_move() && self.player2.ready_to_move() {

						#[deprecated(note = "temporary")] {
							for (i, p) in self.player1.pokemon.iter().enumerate() {
								self.player2.client.add_unknown(i, PokemonUnknown::new(p.value()));
							}
							for (i, p) in self.player2.pokemon.iter().enumerate() {
								self.player1.client.add_unknown(i, PokemonUnknown::new(p.value()));
							}
						}

						// debug!("Starting move calculations");
						self.state = BattleState::MOVE_START;
					} else if self.data.winner.is_some() {
						self.state = BattleState::End;
					}
					
				}
			},
		    BattleState::Moving(move_state) => {
				match move_state {
					MoveState::Start => {
						// Despawn the player button panel
						/////// text.reset();
						*move_state = MoveState::SetupPokemon;
					}
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

										// scuffed code pog

										// retargets moves if target is None

										// debug!("fix retargeting system");

										// if targets.len() > 1 {
										// 	let mut a = None;
										// 	for (i, t) in targets.iter().enumerate() {
										// 		let p = match t {
										// 			MoveTargetInstance::Opponent(i) => other.active[*i].pokemon.as_ref(),
										// 			MoveTargetInstance::Team(i) => user.active[*i].pokemon.as_ref(),
										// 			MoveTargetInstance::User => user.active[instance.pokemon.index].pokemon.as_ref(),
										// 		};
	
										// 		if p.is_some() {
										// 			a = Some(i);
										// 		}
										// 	}
	
										// 	for t in targets.iter_mut() {
										// 		let p = match t {
										// 			MoveTargetInstance::Opponent(i) => other.active[*i].pokemon.as_ref(),
										// 			MoveTargetInstance::Team(i) => user.active[*i].pokemon.as_ref(),
										// 			MoveTargetInstance::User => user.active[instance.pokemon.index].pokemon.as_ref(),
										// 		};
	
										// 		if p.is_none() {
										// 			debug!("retarget {:?}, {:?}", t, a);
										// 			match a {
										// 				Some(active) => *t = match t {
										// 					MoveTargetInstance::Opponent(..) => MoveTargetInstance::Opponent(active),
										// 					MoveTargetInstance::Team(..) => MoveTargetInstance::Team(active),
										// 					MoveTargetInstance::User => unreachable!(),
										// 				},
										// 				None => return,
										// 			}
										// 		}
										// 	}
										// }
										
										let userp = user.active(instance.pokemon.index).unwrap();

										let targets = targets.iter().flat_map(|target| match target {
											MoveTargetInstance::Opponent(team, index) => other.active(*index),
											MoveTargetInstance::Team(index) => user.active(*index),
											MoveTargetInstance::User => Some(userp),
										}.map(|i| (target, i))).map(|(target, pokemon)| pokedex::moves::usage::pokemon::PokemonTarget {
											pokemon,
											active: *target,
										}).collect();

										let turn = userp.use_own_move(engine, *move_index, targets);

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

													fn on_damage(pokemon: &mut crate::pokedex::pokemon::instance::PokemonInstance, results: &mut Vec<BattleClientMove>, damage: crate::pokedex::pokemon::Health, effective: Effective) {
														pokemon.current_hp = pokemon.current_hp.saturating_sub(damage);
														results.push(BattleClientMove::TargetHP(pokemon.hp() as f32 / pokemon.max_hp() as f32));
														if effective != Effective::Effective {
															results.push(BattleClientMove::Effective(effective)); // ui::text::on_effective(text, &effective);
														}
														
														// renderer.flicker();
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
														
														client_results.push(BattleClientMove::Faint(t)); // queue.actions.push_front(BattleActionInstance { pokemon, action: BattleAction::Faint(Some(target_instance)) });
														target_party.active[index].replace();

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
												match self.data.battle_type {
													BattleType::Wild => {

														let index = match target {
															MoveTargetInstance::Opponent(t, i) => *i,
															_ => unreachable!(),
														};

														player_queue.push(BattleClientActionInstance {
															pokemon: instance.pokemon,
															action: BattleClientAction::Catch(index),
														});

															// queue.insert(
															// 	0,
															// 	BattleActionInstance {
															// 		pokemon: instance.pokemon,
															// 		action: BattleAction::Catch(*target),
															// 	}
															// );
															// ui::text::on_catch(&mut gui.text, target);

													},
													_ => info!("Cannot use pokeballs in trainer battles!"),
												}
												false
											},
											ItemUseType::None => true,
										} {
											debug!("to - do: test using items");
											// let level = pokemon.level;
											player_queue.push(BattleClientActionInstance {
												pokemon: instance.pokemon,
												action: BattleClientAction::UseItem(*item, *target),
											}); // ui::text::on_item(text, pokemon, item);
											// user_cli.get_renderer()[instance.pokemon.active].update_status(user.active[instance.pokemon.active].pokemon.as_ref(), level, false);
										}
									}
									BattleMove::Switch(new) => {
										debug!("to - do: dont send unknown every time");
										user.replace(instance.pokemon.index, Some(*new));
										player_queue.push(BattleClientActionInstance {
											pokemon: instance.pokemon,
											action: BattleClientAction::Switch(*new, Some(PokemonUnknown::new(user.active(instance.pokemon.index).unwrap()))),
										}); // ui::text::on_switch(text, pokemon, user.pokemon[*new].as_ref().unwrap().value());
										// debug!("{:?}", user.active);
									}
								}
							}
						}

						// debug!("{:#?}", player_queue);

						// end queue calculations

							self.player2.client.start_moves(player_queue.clone());
							self.player1.client.start_moves(player_queue);

						// debug!("sent moves");

						*move_state = MoveState::SetupPost;



						    // Some(instance) => {

								

							// 	match &mut instance.action {

							// 		BattleAction::Pokemon(battle_move) => match battle_move {

							// 		BattleAction::GainExp(..) => {
							// 		},
							// 		BattleAction::LevelUp(..) => text_update(ctx, delta, text, queue),
            				// 		BattleAction::Catch(target) => {
							// 			if !text.finished() {
							// 				text.update(ctx, delta);
							// 			} else {
							// 				let active = &mut match target.team {
							// 					Team::Player => &mut self.player,
							// 					Team::Opponent => &mut self.opponent
							// 				}.active[target.active];
							// 				match active.pokemon.take() {
							// 					pokemon::PokemonOption::Some(_, pokemon) => {
							// 						active.dequeue();
							// 						if let Err(_) = crate::storage::data_mut().party.try_push(pokemon.owned()) {
							// 							warn!("Player party is full!");
							// 						}
							// 					},
							// 					_ => (),
							// 				}
							// 				queue.current = None;
							// 			}
							// 		}
							// 	}
					},
					MoveState::SetupPost => {
						*move_state = MoveState::Post;
					},
					MoveState::Post => {
						*move_state = MoveState::End;
					}
					MoveState::End => {

						if self.player1.client.wait_finish_turn() && self.player2.client.wait_finish_turn() {
							if self.player2.all_fainted() {
								self.data.winner = Some(self.player1.id);
								self.state = BattleState::End;
							} else if self.player1.all_fainted() {
								self.data.winner = Some(self.player2.id);
								self.state = BattleState::End;
							} else if self.player1.any_replace().is_some() || self.player2.any_replace().is_some() {

								fn replace_faint(party: &mut BattleParty, other_client: &mut dyn BattleClient) {
									if let Some(active) = party.any_replace() {
										if party.any_inactive() {
											if let Some(new) = party.client.wait_faint(active) {
												party.replace(active, Some(new));
												other_client.opponent_faint_replace(active, Some(new));
											}
										} else {
											party.replace(active, None);
										}
									}
								}

								replace_faint(&mut self.player1, self.player2.client.as_mut());
								replace_faint(&mut self.player2, self.player1.client.as_mut());
								
							} else  {
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

	pub fn move_queue(player1: &mut BattleParty, player2: &mut BattleParty) -> Vec<BattleActionInstance> {

		use std::cmp::Reverse;

		#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
		enum Priority {
			First(ActivePokemonIndex),
			Second(Reverse<u8>, Reverse<u16>, ActivePokemonIndex), // priority, speed, pokemon <- fix last, player always goes first
		}

		fn insert(map: &mut std::collections::BTreeMap<Priority, BattleActionInstance>, party: &mut BattleParty) {
			for index in 0..party.active.len() {
				if let Some(queued_move) = party.active[index].use_move() {
					if let Some(pokemon) = party.active(index) {
						let index = ActivePokemonIndex { team: party.id, index };
						map.insert(
							match queued_move {
								BattleMove::Move(..) => Priority::Second(Reverse(0), Reverse(pokemon.base.get(StatType::Speed)), index),
								_ => Priority::First(index),
							}, 
							BattleActionInstance { pokemon: index, action: queued_move }
						);
					}
				}
			}
		}

		let mut map = std::collections::BTreeMap::new();

		insert(&mut map, player1);
		insert(&mut map, player2);

		map.into_iter().map(|(_, i)| i).collect() // into_values

	}

	pub fn update_data(self, player: &mut PlayerSave) -> Option<(PlayerId, bool)> {

		let trainer = self.data.trainer.is_some();

		if let Some(winner) = self.data.winner {
			if player.id == winner {
				if let Some(trainer) = self.data.trainer {
					player.worth += trainer.worth as u32;
					if let Some(badge) = trainer.gym_badge {
						player.world.badges.insert(badge);
					}
				}
			}
		}

		self.data.winner.map(|winner| (winner, trainer))
		
	}
	
}