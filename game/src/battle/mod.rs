// #![feature(map_into_keys_values)] // for move queue fn

use serde::{Deserialize, Serialize};

use crate::{
	deps::{
		Random,
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
			target::{
				Team,
				MoveTargetInstance,
			}
		},
		item::ItemUseType,
	}, 
	storage::player::PlayerSave,
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
		BattleParty,
		ActivePokemonArray,
		ActivePokemonIndex,
		BattleActionInstance,
		BattleMove,
		BattleClientMove,
		BattleClientAction,
		BattleClientActionInstance,
	},
	client::{
		BattleClient,
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
	
	player: BattleParty,
	opponent: BattleParty,

	faints: Vec<ActivePokemonIndex>,
	
}

pub struct BattleData {
	battle_type: BattleType,
	trainer: Option<BattleTrainerEntry>,
	pub winner: Option<Team>,
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
			player,
			opponent: BattleParty::new("opponent", entry.party.into_iter().map(|instance| Some(BorrowedPokemon::Owned(instance))).collect(), entry.size),
			state: BattleState::default(),
			faints: Vec::new(),
		}
	}

	pub fn begin<'a>(&mut self, player: &'a mut dyn BattleClient, opponent: &'a mut dyn BattleClient) {
		player.begin(&self.data, self.player.as_known(), self.opponent.as_unknown());
		opponent.begin(&self.data, self.opponent.as_known(), self.player.as_unknown());
		self.state = BattleState::SELECTING_START;
	}

	pub fn update<'a>(&mut self, engine: &mut Engine, player_cli: &'a mut dyn BattleClient, opponent_cli: &'a mut dyn BattleClient) {

		match &mut self.state {

			BattleState::StartWait => (),

			// Select pokemon moves / items / party switches

		    BattleState::Selecting(started, pdone, odone) => match *started {
				false => {
					player_cli.start_select();
					opponent_cli.start_select();
					*started = true;
				}
				true => {

					fn fill_moves(done: &mut bool, cli: &mut dyn BattleClient, party: &mut BattleParty) {
						if !*done {
							if let Some(mut moves) = cli.wait_select() {
								for active in party.active.iter_mut().rev().filter(|active| active.pokemon.is_active()) {
									active.queued_move = moves.pop();
								}
								// debug!("party with {:?} completed their moves.", party.pokemon(0));
								*done = true;
							}
						}
					}

					fill_moves(pdone, player_cli, &mut self.player);
					fill_moves(odone, opponent_cli, &mut self.opponent);

					if *pdone && *odone {

						#[deprecated(note = "temporary")] {
							use pokemon::PokemonUnknown;
							for (i, p) in self.player.pokemon.iter().enumerate() {
								if let Some(p) = p {
									opponent_cli.add_unknown(i, PokemonUnknown::new(p.value()));
								}
								for (i, p) in self.opponent.pokemon.iter().enumerate() {
									if let Some(p) = p {
										player_cli.add_unknown(i, PokemonUnknown::new(p.value()));
									}
								}
							}
						}



						// debug!("Starting move calculations");
						self.state = BattleState::MOVE_START;
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
						*move_state = MoveState::Pokemon(Self::move_queue(&mut self.player.active, &mut self.opponent.active));
					},
					MoveState::Pokemon(queue) => {

						let mut player_queue = Vec::with_capacity(queue.len());

						for instance in queue {
							let (user, other) = match instance.pokemon.team {
								Team::Player => (&mut self.player, &mut self.opponent),
								Team::Opponent => (&mut self.opponent, &mut self.player),
							};
							if user.active[instance.pokemon.index].pokemon.is_active() {
								match &mut instance.action {
									BattleMove::Move(move_index, targets) => {

										// scuffed code pog

										// retargets moves if target is None

										debug!("fix retargeting system");

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
										
										let userp = user.active[instance.pokemon.index].pokemon.as_ref().unwrap();

										let targets = targets.iter().flat_map(|target| match target {
											MoveTargetInstance::Opponent(index) => other.active.get(*index).map(|active| active.pokemon.as_ref()).flatten(),
											MoveTargetInstance::Team(index) => user.active.get(*index).map(|active| active.pokemon.as_ref()).flatten(),
											MoveTargetInstance::User => Some(userp),
										}.map(|i| (target, i))).map(|(target, pokemon)| pokedex::moves::usage::pokemon::PokemonTarget {
											pokemon,
											active: *target,
										}).collect();

										let turn = userp.use_own_move(engine, *move_index, targets);

										let mut target_results = Vec::with_capacity(turn.results.len());

										for (target_instance, result) in turn.results {

											let mut client_results = Vec::new();

											let (user, other) = match instance.pokemon.team {
												Team::Player => (&mut self.player, &mut self.opponent),
												Team::Opponent => (&mut self.opponent, &mut self.player),
											};
											
											{

												let user = user.active[instance.pokemon.index].pokemon.as_mut().unwrap();

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

													let target = target_party.pokemon_mut(index).unwrap();

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

														let t = match instance.pokemon.team {
															Team::Player => match target_instance {
																MoveTargetInstance::Opponent(index) => ActivePokemonIndex { team: Team::Opponent, index },
																MoveTargetInstance::Team(index) => ActivePokemonIndex { team: Team::Player, index },
																MoveTargetInstance::User => ActivePokemonIndex { team: Team::Player, index: instance.pokemon.index },
															},
															Team::Opponent => match target_instance {
																MoveTargetInstance::Opponent(index) => ActivePokemonIndex { team: Team::Player, index },
																MoveTargetInstance::Team(index) => ActivePokemonIndex { team: Team::Opponent, index },
																MoveTargetInstance::User => ActivePokemonIndex { team: Team::Opponent, index: instance.pokemon.index },
															}
														};
														
														client_results.push(BattleClientMove::Faint(t)); // queue.actions.push_front(BattleActionInstance { pokemon, action: BattleAction::Faint(Some(target_instance)) });
														self.faints.push(t);

														target_party.replace(index, None);

													}
													
													// target_cli.status.update_gui(Some((target_pokemon.level, target_pokemon)), false);
													
												}
												None => client_results.push(BattleClientMove::Miss), // ui::text::on_miss(text, user.active[instance.pokemon.active].pokemon.as_ref().unwrap()),
											}

											// client_results.extend(user_results);

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
												user.active[instance.pokemon.index].pokemon.as_mut().unwrap().execute_item_script(script);
												true
											},
											ItemUseType::Pokeball => {
												match self.data.battle_type {
													BattleType::Wild => {

														let index = match target {
															MoveTargetInstance::Opponent(i) => *i,
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

														// return; // To - do: remove returns
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
											action: BattleClientAction::Switch(*new, Some(pokemon::PokemonUnknown::new(user.pokemon(instance.pokemon.index).unwrap()))),
										}); // ui::text::on_switch(text, pokemon, user.pokemon[*new].as_ref().unwrap().value());
										// debug!("{:?}", user.active);
									}
								}
							}
						}

						// debug!("{:#?}", player_queue);

						// end queue calculations

						let o = player_queue.clone().into_iter().map(|i| i.into_other()).collect();

						opponent_cli.start_moves(o);

						player_cli.start_moves(player_queue);

						// debug!("sent moves");

						*move_state = MoveState::SetupPost;



						    // Some(instance) => {

								

							// 	match &mut instance.action {

							// 		BattleAction::Pokemon(battle_move) => match battle_move {

							// 			BattleMove::Move(.., move_target) => {

							// 				fn vec_if<'a>(target: &'a mut pokemon::ActivePokemon, target_cli: &'a mut pokemon::gui::ActivePokemonRenderer) -> Vec<(&'a mut pokemon::ActivePokemon, &'a mut pokemon::gui::ActivePokemonRenderer)> {
							// 					if target_cli.renderer.flicker.flickering() || target_cli.status.health_moving() {
							// 						vec![(target, target_cli)]
							// 					} else {
							// 						Vec::new()
							// 					}
							// 				}

							// 				let targets = match move_target {
							// 					MoveTargetInstance::User => {
							// 						vec_if(&mut user.active[instance.pokemon.active], &mut user_cli.get_renderer()[instance.pokemon.active])
							// 					}
							// 					MoveTargetInstance::Opponent(index) => {
							// 						vec_if(&mut other.active[*index], &mut other_cli.get_renderer()[*index])
							// 					},
							// 					MoveTargetInstance::Team(index) => {
							// 						vec_if(&mut user.active[*index], &mut user_cli.get_renderer()[*index])
							// 					},
							// 					MoveTargetInstance::Opponents => {
							// 						let mut targets = Vec::with_capacity(other.active.len());
							// 						for (i, target) in other.active.iter_mut().enumerate() {
							// 							let cli = &mut other_cli.get_renderer()[i];
							// 							if cli.renderer.flicker.flickering() || cli.status.health_moving() {
							// 								targets.push((target, cli));															
							// 							}
							// 						}
							// 						targets
													
							// 					}
							// 					MoveTargetInstance::AllButUser => {
							// 						let mut targets = Vec::with_capacity(user.active.len() - 1 + other.active.len());
							// 						for (index, target) in user.active.iter_mut().enumerate() {
							// 							let cli = &mut user_cli.get_renderer()[index];
							// 							if index != instance.pokemon.active && (cli.renderer.flicker.flickering() || cli.status.health_moving()) {
							// 								targets.push((target, cli));
							// 							}
							// 						}
							// 						for (i, target) in other.active.iter_mut().enumerate() {
							// 							let cli = &mut other_cli.get_renderer()[i];
							// 							if cli.renderer.flicker.flickering() || cli.status.health_moving() {
							// 								targets.push((target, cli));															
							// 							}
							// 						}
							// 						targets
							// 					}
							// 				};

							// 				if !text.finished() {
							// 					text.update(ctx, delta);
							// 				} else if targets.is_empty() {
							// 					queue.current = None;
							// 				}

							// 				for (_, target) in targets {
												
							// 					if text.current > 0 || text.can_continue {
							// 						target.renderer.flicker.update(delta);
							// 						target.status.update_hp(delta);
							// 					}
							// 				}									
							// 			}
							// 		}
							// 		// BattleAction::Effective(..) => text_update(delta, gui, queue),
							// 		BattleAction::Faint(..) => {
							// 			if let Some(assailant) = assailant {
                            //                 if assailant.team == Team::Player {
                            //                     let experience = {
                            //                         let instance = user.active[instance.pokemon.active].pokemon.as_ref().unwrap();
                            //                         instance.pokemon.value().exp_from(instance.level) as f32 * 
                            //                         match self.is_wild {
                            //                             true => 1.0,
                            //                             false => 1.5,
                            //                         } *
                            //                         7.0
                            //                     } as crate::pokedex::pokemon::Experience;
                            //                     let (assailant_party, index) = (&mut match assailant.team {
                            //                         Team::Player => &mut self.player,
                            //                         Team::Opponent => &mut self.opponent,
                            //                     }, assailant.active);
                            //                     if let Some(assailant_pokemon) = assailant_party.active[index].pokemon.as_mut() {
                            //                         let level = assailant_pokemon.level;
                            //                         if let Some((level, moves)) = assailant_pokemon.add_exp(experience) {
                            //                             queue.actions.push_front(BattleActionInstance { pokemon: *assailant, action: BattleAction::LevelUp(level, moves) });
                            //                         }
                            //                         queue.actions.push_front(BattleActionInstance { pokemon: *assailant, action: BattleAction::GainExp(level, experience) });
                            //                     }
                            //                 }
                            //             }
							// 		}
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

						// self.player.run_replace();
						// self.opponent.run_replace();

						// if started { stuff } else start and do calculations and add text
						if player_cli.wait_finish_turn() && opponent_cli.wait_finish_turn() {
							if self.opponent.all_fainted() {
								self.data.winner = Some(Team::Player);
								self.state = BattleState::End;
							} else if self.player.all_fainted() {
								self.data.winner = Some(Team::Opponent);
								self.state = BattleState::End;
							} else if !self.faints.is_empty() {
								let mut i = 0;
								while i != self.faints.len() {
									let active = &self.faints[i];
									match active.team {
										Team::Player => {
											if self.opponent.any_inactive() {
												if let Some(new) = player_cli.wait_faint(active.index) {
													let index = active.index;
													self.player.replace(index, Some(new));
													opponent_cli.opponent_faint_replace(index, Some(new));
													self.faints.remove(i);
												} else {
													i += 1;
												}
											} else {
												self.player.replace(active.index, None);
												self.faints.remove(i);
											}
										},
										Team::Opponent => {
											if self.opponent.any_inactive() {
												if let Some(new) = opponent_cli.wait_faint(active.index) {
													// if let Some(pokemon) = self.opponent.pokemon(active.index) {
													// 	if !pokemon.fainted() {
															self.opponent.replace(active.index, Some(new));
															player_cli.opponent_faint_replace(active.index, Some(new));
															self.faints.remove(i);
													// 	}
													// }
												} else {
													i += 1;
												}
											} else {
												self.opponent.replace(active.index, None);
												self.faints.remove(i);
											}
										}
									} 
								}
							} else  {
								self.state = BattleState::SELECTING_START;
							}
						}

						// Once the text is finished, despawn it
						// text.despawn();

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

	pub fn move_queue(player: &mut ActivePokemonArray, opponent: &mut ActivePokemonArray) -> Vec<BattleActionInstance> {

		use std::cmp::Reverse;

		#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
		enum Priority {
			First(ActivePokemonIndex),
			Second(Reverse<u8>, Reverse<u16>, ActivePokemonIndex), // priority, speed, pokemon <- fix last, player always goes first
		}

		fn insert(map: &mut std::collections::BTreeMap<Priority, BattleActionInstance>, team: Team, active: &mut ActivePokemonArray) {
			for (index, active) in active.iter_mut().enumerate() {
				if let (Some(pokemon), Some(battle_move)) = (active.pokemon.as_ref(), active.queued_move.take()) {
					let index = ActivePokemonIndex { team, index };
					map.insert(
						match battle_move {
							BattleMove::Move(..) => Priority::Second(Reverse(0), Reverse(pokemon.base.get(StatType::Speed)), index),
							_ => Priority::First(index),
						}, 
						BattleActionInstance { pokemon: index, action: battle_move }
					);
				}
			}
		}

		let mut map = std::collections::BTreeMap::new();

		insert(&mut map, Team::Player, player);
		insert(&mut map, Team::Opponent, opponent);

		map.into_iter().map(|(_, i)| i).collect() // into_values

	}

	pub fn update_data(self, player: &mut PlayerSave) -> Option<(Team, bool)> {

		let trainer = self.data.trainer.is_some();

		if let Some(winner) = self.data.winner {
			match winner {
			    Team::Player => {
					if let Some(trainer) = self.data.trainer {
						player.worth += trainer.worth as u32;
						if let Some(badge) = trainer.gym_badge {
							player.world.badges.insert(badge);
						}
					}		
				}
			    Team::Opponent => (),
			}
		}

		self.data.winner.map(|winner| (winner, trainer))
		
	}
	
}