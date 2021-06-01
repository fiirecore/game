// #![feature(map_into_keys_values)] // for move queue fn ~line 558

use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

use crate::{
	deps::{
		Random,
		rhai::Engine,
	},
	util::{
		Entity,
		Completable,
		Reset,
	},
	battle_glue::{
		BattleEntry,
		BattleTrainerEntry,
	},
	pokedex::{
		types::Effective,
		pokemon::{
			instance::BorrowedPokemon,
			stat::StatType,
			party::MoveableParty,
		},
		moves::{
			usage::{
				MoveResult,
				PokemonTarget,
			},
			target::{
				Team,
				MoveTargetInstance,
			}
		},
		item::ItemUseType,
		texture::PokemonTexture,
	},
	storage::player::PlayerSave,
	tetra::Context,
	log::{info, warn},
};

use crate::battle::{
	state::{
		BattleState,
		MoveState,
		MoveQueue
	},
	pokemon::{
		BattleParty,
		ActivePokemonArray,
		ActivePokemonIndex,
		BattleAction,
		BattleActionInstance,
		BattleMove,
	},
	player::{
		BattlePlayer,
		BattlePlayerAction,
	},
	ui::{
		BattleGui,
		BattleGuiPosition,
	},
};

pub mod state;
pub mod manager;

pub mod pokemon;
pub mod player;

pub mod ui;

pub static BATTLE_RANDOM: Random = Random::new();

pub struct Battle {
	
	pub state: BattleState,

	pub data: BattleData,
	
	player: BattlePlayer,
	opponent: BattlePlayer, // to - do: move input handling (ai, on screen, over network)
	
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
	
	pub fn new(ctx: &mut Context, player_gui: Box<dyn BattlePlayerAction>, player: MoveableParty, entry: BattleEntry) -> Option<Self> {		
		if !(
			player.is_empty() || 
			entry.party.is_empty() ||
			// Checks if player has any pokemon in party that aren't fainted (temporary)
			player.iter().flatten().filter(|pokemon| !pokemon.value().fainted()).next().is_none()
		) {
			Some(
				Self {
					data: BattleData {
						battle_type: entry.trainer.as_ref().map(|trainer| if trainer.gym_badge.is_some() { BattleType::GymLeader } else { BattleType::Trainer }).unwrap_or(BattleType::Wild),
						trainer: entry.trainer,
						winner: None,
					},
					player: BattlePlayer {
						player: player_gui,
						party: BattleParty::new(ctx, player, entry.size, PokemonTexture::Back, BattleGuiPosition::Bottom),
					},
					opponent: BattlePlayer {
						player: Box::new(player::ai::BattlePlayerAi),
						party: BattleParty::new(ctx, entry.party.into_iter().map(|instance| Some(BorrowedPokemon::Owned(instance))).collect(), entry.size, PokemonTexture::Front, BattleGuiPosition::Top),
					},
					state: BattleState::default(),
				}
			)
		} else {
			None
		}
	}

	// input happens here too!
	pub fn update(&mut self, ctx: &Context, delta: f32, engine: &mut Engine, gui: &mut BattleGui) {

		gui.bounce.update(delta);

		match &mut self.state {

			BattleState::Begin => {
				self.state = BattleState::SELECTING_START;
				self.update(ctx, delta, engine, gui);
			}

			// Select pokemon moves / items / party switches

		    BattleState::Selecting(team, active) => match team {
				Team::Player => if self.player.player.moves(ctx, delta, &self.data, active, &mut self.player.party, &self.opponent.party.active) {
					*team = Team::Opponent;
				}
				Team::Opponent => if self.opponent.player.moves(ctx, delta, &self.data, active, &mut self.opponent.party, &self.player.party.active) {
					self.state = BattleState::Moving(MoveState::Start)
				}
			},
		    BattleState::Moving(move_state) => {
				match move_state {
					MoveState::Start => {
						// Despawn the player button panel
						gui.text.reset();
						*move_state = MoveState::SetupPokemon;
					}
					MoveState::SetupPokemon => {
						// Queue pokemon moves					
						*move_state = MoveState::Pokemon(MoveQueue::new(Self::move_queue(&mut self.player.party.active, &mut self.opponent.party.active)));
					},
					MoveState::Pokemon(queue) => {

						// Check if there is a current action active

						match queue.current.as_mut() {
						    None => {
								match queue.actions.pop_front() {
									Some(instance) => {

										let (user, other) = match instance.pokemon.team {
											Team::Player => (&mut self.player, &mut self.opponent),
											Team::Opponent => (&mut self.opponent, &mut self.player),
										};
										if let Some(pokemon) = user.party.active[instance.pokemon.active].pokemon.as_mut() { // To - do:
											gui.text.clear();
											gui.text.spawn();
											match &instance.action {
												BattleAction::Pokemon(battle_move) => match battle_move {
													BattleMove::Move(move_index, target_instance) => {

														// scuffed code pog

														let target_instance = match target_instance {
															MoveTargetInstance::Opponent(index) => match other.party.active[*index].pokemon.is_active() {
																true => *target_instance,
																false => {

																	if other.party.all_fainted() {
																		return;
																	}

																	let mut indexes = Vec::with_capacity(other.party.active.len() - 1);
																	for target in other.party.active.iter_mut().enumerate() {
																		if target.1.pokemon.is_active() {
																			indexes.push(target.0)
																		}
																	}
																	let index = indexes[BATTLE_RANDOM.gen_range(0, indexes.len())];
																	MoveTargetInstance::Opponent(index)
																}
															},
															i => *i,
														};

														let targets = match target_instance {
															MoveTargetInstance::User => {
																vec![PokemonTarget { pokemon: user.party.active[instance.pokemon.active].pokemon.as_ref().unwrap(), instance: target_instance }]
															},
															MoveTargetInstance::Opponent(index) => {
																other.party.active[index].pokemon.as_ref().map(|pokemon| vec![PokemonTarget { pokemon, instance: target_instance }]).unwrap_or_default()	
															}
															MoveTargetInstance::Team(index) => {
																user.party.active[index].pokemon.as_ref().map(|pokemon| vec![PokemonTarget { pokemon, instance: target_instance }]).unwrap_or_default()
															}
															MoveTargetInstance::Opponents => {
																other.party.active.iter().enumerate().map(|(index, active)| active.pokemon.as_ref().map(|pokemon| PokemonTarget { pokemon, instance: MoveTargetInstance::Opponent(index) })).flatten().collect()
															}
															MoveTargetInstance::AllButUser => {
																let mut targets = Vec::with_capacity(user.party.active.len() - 1 + other.party.active.len());
																for (index, active) in other.party.active.iter().enumerate() {
																	if let Some(pokemon) = active.pokemon.as_ref() {
																		targets.push(PokemonTarget { pokemon, instance: MoveTargetInstance::Opponent(index) });
																	}
																}
																for (index, active) in user.party.active.iter().enumerate() {
																	if index != instance.pokemon.active {
																		if let Some(pokemon) = active.pokemon.as_ref() {
																			targets.push(PokemonTarget { pokemon, instance: MoveTargetInstance::Team(index) });
																		}
																	}
																}
																targets
															}
														};

														let turn = user.party.active[instance.pokemon.active].pokemon.as_ref().unwrap().use_own_move(engine, *move_index, targets);
														
														{
															let user_pokemon = user.party.active[instance.pokemon.active].pokemon.as_ref().unwrap();
	
															ui::text::on_move(&mut gui.text, turn.pokemon_move, user_pokemon);

														}

														for (target_instance, result) in turn.results {
															match result {
																Some(result) => {

																	{

																		let user = user.party.active[instance.pokemon.active].pokemon.as_mut().unwrap();

																		match &result {
																			MoveResult::Drain(_, heal, _) => {
																				user.current_hp = (user.current_hp + *heal).min(user.base.hp());
																			}
																			_ => (),
																		}

																	}

																	let target = match target_instance {
																		MoveTargetInstance::Opponent(index) => &mut other.party.active[index],
																		MoveTargetInstance::User => &mut user.party.active[instance.pokemon.active],
																		MoveTargetInstance::Team(index) => &mut user.party.active[index],
																		MoveTargetInstance::AllButUser | MoveTargetInstance::Opponents => unreachable!(),
																	};

																	let target_pokemon = target.pokemon.as_mut().unwrap();

																	fn on_damage(pokemon: &mut crate::pokedex::pokemon::instance::PokemonInstance, renderer: &mut ui::pokemon::PokemonRenderer, gui: &mut BattleGui, damage: crate::pokedex::pokemon::Health, effective: Effective) {
																		if effective != Effective::Effective {
																			ui::text::on_effective(&mut gui.text, &effective);
																		}
																		pokemon.current_hp = pokemon.current_hp.saturating_sub(damage);
																		renderer.flicker();
																	}

																	match result {
																		MoveResult::Damage(damage, effective) => {
																			on_damage(target_pokemon, &mut target.renderer, gui, damage, effective);
																		},
																		MoveResult::Status(effect) => {
																			target_pokemon.status = Some(effect);
																		},
																		MoveResult::Drain(damage, _, effective) => {
																			on_damage(target_pokemon, &mut target.renderer, gui, damage, effective);
																		},
																		MoveResult::StatStage(stat, stage) => {
																			target_pokemon.base.change_stage(stat, stage);
																			ui::text::on_stat_stage(&mut gui.text, target_pokemon, stat, stage)
																		}
																		MoveResult::Todo => {
																			ui::text::on_fail(&mut gui.text, vec![format!("Cannot use move on {}", target_pokemon.name()), format!("Move {} is unimplemented", turn.pokemon_move.name)]);
																		},
																	}

																	let pokemon = match target_instance {
																		MoveTargetInstance::Opponent(index) => ActivePokemonIndex { team: instance.pokemon.team.other(), active: index },
																		MoveTargetInstance::Team(index) => ActivePokemonIndex { team: instance.pokemon.team, active: index },
																		MoveTargetInstance::User => instance.pokemon,
																		MoveTargetInstance::AllButUser | MoveTargetInstance::Opponents => unreachable!(),
																	};
																	

																	if target_pokemon.fainted() {
																		queue.actions.push_front(BattleActionInstance { pokemon, action: BattleAction::Faint(Some(instance.pokemon)) });
																	}
																	
																	target.status.update_gui(Some((target_pokemon.level, target_pokemon)), false);
																	
																}
																None => ui::text::on_miss(&mut gui.text, user.party.active[instance.pokemon.active].pokemon.as_ref().unwrap()),
															}
														}
													}
													BattleMove::UseItem(item, target) => {
														let item = item.unwrap();
														if match &item.usage {
															ItemUseType::Script(script) => {
																pokemon.execute_item_script(script);
																true
															},
															ItemUseType::Pokeball => {
																match self.data.battle_type {
																	BattleType::Wild => {
																		queue.actions.push_front(
																			BattleActionInstance {
																				pokemon: instance.pokemon,
																				action: BattleAction::Catch(*target),
																			}
																		);
																		return; // To - do: remove returns
																			// ui::text::on_catch(&mut gui.text, target);
																	},
																	_ => info!("Cannot use pokeballs in trainer battles!"),
																}
																false
															},
															ItemUseType::None => true,
														} {
															let level = pokemon.level;
															ui::text::on_item(&mut gui.text, pokemon, item);
															user.party.active[instance.pokemon.active].update_status(level, false);
														}
													}
													BattleMove::Switch(new) => {
														ui::text::on_switch(&mut gui.text, pokemon, user.party.pokemon[*new].as_ref().unwrap().value());
													}
												}
											    BattleAction::Faint(assailant) => {
													ui::text::on_faint(&mut gui.text, self.data.battle_type, instance.pokemon.team, pokemon);
													user.party.active[instance.pokemon.active].renderer.faint();

													if let Some(assailant) = assailant {
														if assailant.team == Team::Player {
															let experience = {
																let instance = user.party.active[instance.pokemon.active].pokemon.as_ref().unwrap();
																instance.pokemon.unwrap().exp_from(instance.level) as f32 * 
																match self.data.battle_type {
																	BattleType::Wild => 1.0,
																	_ => 1.5,
																} *
																7.0
															} as crate::pokedex::pokemon::Experience;
															let (assailant_party, index) = (&mut match assailant.team {
																Team::Player => &mut self.player,
																Team::Opponent => &mut self.opponent,
															}, assailant.active);
															if let Some(assailant_pokemon) = assailant_party.party.active[index].pokemon.as_mut() {
																let level = assailant_pokemon.level;
																if let Some((level, moves)) = assailant_pokemon.add_exp(experience) {
																	queue.actions.push_front(BattleActionInstance { pokemon: *assailant, action: BattleAction::LevelUp(level, moves) });
																}
																queue.actions.push_front(BattleActionInstance { pokemon: *assailant, action: BattleAction::GainExp(level, experience) });
															}
														}
													}
												},
												BattleAction::GainExp(level, experience) => { // To - do: experience spreading
													ui::text::on_gain_exp(&mut gui.text, pokemon, *experience);
													user.party.active[instance.pokemon.active].update_status(*level, false);
												}
												BattleAction::LevelUp(level, moves) => {
													ui::text::on_level_up(&mut gui.text, pokemon, *level);
													if let Some(_) = moves {
														ui::text::on_fail(&mut gui.text, vec![format!("To - do: handle moves on level up")]);
													}
												}
												BattleAction::Catch(index) => {
													if let Some(target) = match index.team {
														Team::Player => &user.party.active[index.active],
														Team::Opponent => &other.party.active[index.active],
													}.pokemon.as_ref() {
														ui::text::on_catch(&mut gui.text, target);
													}
												}
											}
											queue.current = Some(BattleActionInstance { pokemon: instance.pokemon, action: instance.action });
											self.update(ctx, delta, engine, gui);
										}
									},
									None => {
										*move_state = MoveState::SetupPost;
									}
								}
							},
						    Some(instance) => {

								let (user, other) = match instance.pokemon.team {
									Team::Player => (&mut self.player, &mut self.opponent),
									Team::Opponent => (&mut self.opponent, &mut self.player),
								};

								match &mut instance.action {

									BattleAction::Pokemon(battle_move) => match battle_move {

										BattleMove::Move(.., move_target) => {

											fn vec_if(target: &mut pokemon::ActivePokemon) -> Vec<&mut pokemon::ActivePokemon> {
												if target.renderer.flicker.flickering() || target.status.health_moving() {
													vec![target]
												} else {
													Vec::new()
												}
											}

											let targets = match move_target {
												MoveTargetInstance::User => {
													vec_if(&mut user.party.active[instance.pokemon.active])
												}
												MoveTargetInstance::Opponent(index) => {
													vec_if(&mut other.party.active[*index])
												},
												MoveTargetInstance::Team(index) => {
													vec_if(&mut user.party.active[*index])
												},
												MoveTargetInstance::Opponents => {
													let mut targets = Vec::with_capacity(other.party.active.len());
													for target in other.party.active.iter_mut() {
														if target.renderer.flicker.flickering() || target.status.health_moving() {
															targets.push(target);															
														}
													}
													targets
													
												}
												MoveTargetInstance::AllButUser => {
													let mut targets = Vec::with_capacity(user.party.active.len() - 1 + other.party.active.len());
													for (index, target) in user.party.active.iter_mut().enumerate() {
														if index != instance.pokemon.active && (target.renderer.flicker.flickering() || target.status.health_moving()) {
															targets.push(target);
														}
													}
													for target in other.party.active.iter_mut() {
														if target.renderer.flicker.flickering() || target.status.health_moving() {
															targets.push(target);															
														}
													}
													targets
												}
											};

											if !gui.text.finished() {
												gui.text.update(ctx, delta);
											} else if targets.is_empty() {
												queue.current = None;
											}

											for target in targets {
												if gui.text.current > 0 || gui.text.can_continue {
													target.renderer.flicker.update(delta);
													target.status.update_hp(delta);
												}
											}									
										}
										BattleMove::UseItem(..) => {
											if !gui.text.finished() {
												gui.text.update(ctx, delta)
											} else if user.party.active[instance.pokemon.active].status.health_moving() {
												user.party.active[instance.pokemon.active].status.update_hp(delta);
											} else {
												queue.current = None;
											}
										},
										BattleMove::Switch(new) => {
											if gui.text.finished() {
												queue.current = None;
											} else {

												gui.text.update(ctx, delta);

												if gui.text.current() == 1 && user.party.pokemon[*new].is_some() {
													user.party.replace(instance.pokemon.active, *new);
												}

											}
										}
									}
									// BattleAction::Effective(..) => text_update(delta, gui, queue),
									BattleAction::Faint(..) => {
										// let party = match target.team {
										// 	Team::Player => &mut self.player,
										// 	Team::Opponent => &mut self.opponent,
										// };
										if user.party.active[instance.pokemon.active].renderer.faint.fainting() {
											user.party.active[instance.pokemon.active].renderer.faint.update(delta);
										} else if !gui.text.finished() {
											gui.text.update(ctx, delta);
										} else {
											if user.player.faint(ctx, delta, &self.data, instance.pokemon.active, &mut user.party) {
												queue.current = None;
											}
										}
									}
									BattleAction::GainExp(..) => {
										let user = &mut user.party.active[instance.pokemon.active];
										if !gui.text.finished() || user.status.exp_moving() {
											gui.text.update(ctx, delta);
											if gui.text.current > 0 || gui.text.can_continue {
												user.status.update_exp(delta, user.pokemon.as_ref().unwrap());
											}
										} else {
											queue.current = None;
										}
									},
									BattleAction::LevelUp(..) => text_update(ctx, delta, gui, queue),
            						BattleAction::Catch(target) => {
										if !gui.text.finished() {
											gui.text.update(ctx, delta);
										} else {
											let active = &mut match target.team {
												Team::Player => &mut self.player,
												Team::Opponent => &mut self.opponent
											}.party.active[target.active];
											match active.pokemon.take() {
												pokemon::PokemonOption::Some(_, pokemon) => {
													active.update();
													if let Err(_) = crate::storage::data_mut().party.try_push(pokemon.owned()) {
														warn!("Player party is full!");
													}
												},
												_ => (),
											}
											queue.current = None;
										}
									}
								}
							}
						}
					},
					MoveState::SetupPost => {
						*move_state = MoveState::Post;
					},
					MoveState::Post => {
						*move_state = MoveState::End;
					}
					MoveState::End => {
						self.player.party.run_replace();
						self.opponent.party.run_replace();
						// if started { stuff } else start and do calculations and add text
						self.state = if self.opponent.party.all_fainted() {
							self.data.winner = Some(Team::Player);
							BattleState::End
						} else if self.player.party.all_fainted() {
							self.data.winner = Some(Team::Opponent);
							BattleState::End
						} else {
							BattleState::SELECTING_START
						};
						// Once the text is finished, despawn it
						gui.text.despawn();
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
	
	pub fn render(&self, ctx: &mut Context, gui: &BattleGui) {
		use crate::{graphics::ZERO, tetra::{math::Vec2, graphics::Color}};
		gui.background.draw(ctx, 0.0);
		for active in self.opponent.party.active.iter() {
			active.renderer.draw(ctx, ZERO, Color::WHITE);
			active.status.draw(ctx, 0.0, 0.0);
		}
		match &self.state {
			BattleState::Begin | BattleState::End => (),
		    BattleState::Selecting(_, index) => {
				for (current, active) in self.player.party.active.iter().enumerate() {
					if &current == index {
						active.renderer.draw(ctx, Vec2::new(0.0, gui.bounce.offset), Color::WHITE);
						active.status.draw(ctx, 0.0, -gui.bounce.offset);
					} else {
						active.renderer.draw(ctx, ZERO, Color::WHITE);
						active.status.draw(ctx, 0.0, 0.0);
					}
				}
				gui.draw_panel(ctx);
				// gui.panel.draw(ctx);
				self.player.player.draw(ctx);
			},
			BattleState::Moving( .. ) => {
				for active in self.player.party.active.iter() {
					active.renderer.draw(ctx, ZERO, Color::WHITE);
					active.status.draw(ctx, 0.0, 0.0);
				}
				gui.draw_panel(ctx);
				gui.text.draw(ctx);
			}
		}
	}

	pub fn move_queue(player: &mut ActivePokemonArray, opponent: &mut ActivePokemonArray) -> VecDeque<BattleActionInstance> {

		use std::cmp::Reverse;

		#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
		enum Priority {
			First(ActivePokemonIndex),
			Second(Reverse<u8>, Reverse<u16>, ActivePokemonIndex), // priority, speed, pokemon <- fix last, player always goes first
		}

		fn insert(map: &mut std::collections::BTreeMap<Priority, BattleActionInstance>, team: Team, active: &mut ActivePokemonArray) {
			for (index, active) in active.iter_mut().enumerate() {
				if let (Some(pokemon), Some(battle_move)) = (active.pokemon.as_ref(), active.queued_move.take()) {
					let index = ActivePokemonIndex { team, active: index };
					map.insert(
						match battle_move {
							BattleMove::Move(..) => Priority::Second(Reverse(0), Reverse(pokemon.base.get(StatType::Speed)), index),
							_ => Priority::First(index),
						}, 
						BattleActionInstance { pokemon: index, action: BattleAction::Pokemon(battle_move) }
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

fn text_update(ctx: &Context, delta: f32, gui: &mut BattleGui, queue: &mut MoveQueue) {
	if !gui.text.finished() {
		gui.text.update(ctx, delta);
	} else {
		queue.current = None;
	}
}