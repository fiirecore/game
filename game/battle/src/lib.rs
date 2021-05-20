// #![feature(map_into_keys_values)] // for move queue fn ~line 558

extern crate firecore_game as game;

use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

use game::{
	deps::Random,
	util::{
		Entity,
		Completable,
		Reset,
	},
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	battle::{
		BattleEntry,
		BattleTrainerEntry,
		BattleTeam,
	},
	pokedex::{
		moves::{
			MoveCategory,
			target::{
				MoveTarget,
				MoveTargetInstance,
			},
		},
		pokemon::{
			instance::{
				BorrowedPokemon,
				MoveResults,
			},
			party::MoveableParty,
		},
		texture::PokemonTexture,
		item::ItemUseType,
	},
	storage::player::PlayerSave,
	macroquad::prelude::warn,
};

use crate::{
	state::{
		BattleState,
		MoveState,
		MoveQueue
	},
	pokemon::{
		BattleParty,
		ActivePokemonIndex,
		ActivePokemonArray,
		BattleAction,
		BattleActionInstance,
		BattleMove,
		ai::BattleAi,
	},
	ui::{
		BattleGui,
		BattleGuiPosition,
		battle_party_gui,
	},
};

pub mod state;
pub mod manager;

pub mod pokemon;
pub mod ui;

pub static BATTLE_RANDOM: Random = Random::new();

pub struct Battle {

	pub data: BattleData,
	
	player: BattleParty,
	opponent: BattleParty, // to - do: move input handling (ai, on screen, over network)
	ai: BattleAi,
	pub state: BattleState,
	
}

pub struct BattleData {
	battle_type: BattleType,
	trainer: Option<BattleTrainerEntry>,
	pub winner: Option<BattleTeam>,
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
	
	pub fn new(player: MoveableParty, entry: BattleEntry) -> Option<Self> {		
		if !(
			player.is_empty() || 
			entry.party.is_empty() ||
			// Checks if player has any pokemon in party that aren't fainted (temporary)
			player.iter().flatten().filter(|pokemon| pokemon.value().current_hp != 0).next().is_none()
		) {
			Some(
				Self {
					data: BattleData {
						battle_type: entry.trainer.as_ref().map(|trainer| if trainer.is_gym_leader { BattleType::GymLeader } else { BattleType::Trainer }).unwrap_or(BattleType::Wild),
						trainer: entry.trainer,
						winner: None,
					},
					player: BattleParty::new(player, entry.size, PokemonTexture::Back, BattleGuiPosition::Bottom),
					opponent: BattleParty::new(entry.party.into_iter().map(|instance| Some(BorrowedPokemon::Owned(instance))).collect(), entry.size, PokemonTexture::Front, BattleGuiPosition::Top),
					state: BattleState::default(),
					ai: BattleAi::Random,
				}
			)
		} else {
			None
		}
	}

	pub fn start_moves(&mut self) {	
		self.ai.moves(&mut self.opponent.active, &self.player.active);
		self.state = BattleState::Moving(MoveState::Start);
	}

	// input happens here too!
	pub fn update(&mut self, delta: f32, gui: &mut BattleGui, party_gui: &mut PartyGui, bag: &mut BagGui) {

		gui.bounce.update(delta);

		match &mut self.state {

			BattleState::Begin => {
				if let Some(pokemon) = self.player.active[0].pokemon.as_ref() {
					gui.panel.run(&mut None, pokemon, &self.opponent.active);
				}
				self.state = BattleState::SELECTING;
				self.update(delta, gui, party_gui, bag);
			}

			// Select pokemon moves / items / party switches

		    BattleState::Selecting(index) => {

				// Check if there is an active slot at the current index

				match self.player.active.get_mut(*index) {
				    Some(active) => match active.pokemon.as_mut() {
						Some(pokemon) => match gui.panel.is_alive() {
							true => match active.queued_move.is_none() {
								true => {

									// Checks if a move is queued from an action done in the GUI
	
									if bag.alive {
										bag.input();
										if let Some(item) = bag.take_selected_despawn() {
											let target = match item.value().use_type {
												ItemUseType::Pokeball => Some(MoveTargetInstance::Opponent(BATTLE_RANDOM.gen_range(0, self.opponent.active.len()))),
												_ => None,
											};
											active.queued_move = Some(BattleMove::UseItem(item, target));
										}
									} else if party_gui.alive {
										party_gui.input();
										party_gui.update(delta);
										if let Some(selected) = party_gui.selected.take() {
											party_gui.despawn();
											active.queued_move = Some(BattleMove::Switch(selected));
										}
									} else {
										if let Some(panels) = gui.panel.input(pokemon) {
											match panels {
												ui::panels::BattlePanels::Main => {
													match gui.panel.battle.cursor {
														0 => gui.panel.active = ui::panels::BattlePanels::Fight,
														1 => bag.spawn(false),
														2 => ui::battle_party_gui(party_gui, &self.player, true),
														3 => if self.data.battle_type == BattleType::Wild {
															// closer.spawn(self, &mut gui.text);
															self.state = BattleState::End; // To - do: "Got away safely!" - run text and conditions
														},
														_ => unreachable!(),
													}
												}
												ui::panels::BattlePanels::Fight => match pokemon.moves.get(gui.panel.fight.moves.cursor) {
												    Some(instance) => match instance.get() {
												        Some(move_ref) => active.queued_move = Some(BattleMove::Move(
															gui.panel.fight.moves.cursor,
															match move_ref.value().target {
															    MoveTarget::Player => todo!("moves that target self"),
															    MoveTarget::Opponent => MoveTargetInstance::Opponent(gui.panel.fight.targets.cursor),
															}
														)),
												        None => warn!("Pokemon is out of Power Points for this move!")
												    }
												    None => warn!("Could not get move at cursor!"),
												}
											}
										}
									}
								}
								false => {
									*index += 1;
									gui.panel.despawn();
								}
							}
							false => {
								gui.panel.run(&mut active.last_move, pokemon, &self.opponent.active);
							}
						},
						None => *index += 1,
					}
				    None => self.start_moves(),
				}
			},
		    BattleState::Moving(move_state) => {
				match move_state {
					MoveState::Start => {
						// Despawn the player button panel
						gui.panel.despawn();
						gui.text.reset();
						*move_state = MoveState::SetupPokemon;
					}
					MoveState::SetupPokemon => {
						// Queue pokemon moves					
						*move_state = MoveState::Pokemon(MoveQueue::new(Self::move_queue(&mut self.player.active, &mut self.opponent.active)));
					},
					MoveState::Pokemon(queue) => {

						// Check if there is a current action active

						match queue.current.as_mut() {
						    None => {
								match queue.actions.pop_front() {
									Some(instance) => {
										let (user, other) = match instance.pokemon.team {
											BattleTeam::Player => (&mut self.player, &mut self.opponent),
											BattleTeam::Opponent => (&mut self.opponent, &mut self.player),
										};

										if let Some(pokemon) = user.active[instance.pokemon.active].pokemon.as_mut() {
											gui.text.clear();
											gui.text.spawn();
											match &instance.action {
												BattleAction::Pokemon(battle_move) => match battle_move {
													BattleMove::Move(move_index, target) => {

														let (user_pkmn, target, active, index) = match target {
															MoveTargetInstance::Opponent(index) => (pokemon, &mut other.active, *index, ActivePokemonIndex { team: instance.pokemon.team.other(), active: *index }),
														};

														let target = match target[active].pokemon.is_active() {
															true => &mut target[active],
															false => {
																let mut indexes = Vec::with_capacity(target.len() - 1);
																for target in target.iter_mut().enumerate() {
																	if target.1.pokemon.is_active() {
																		indexes.push(target.0)
																	}
																}
																&mut target[indexes[BATTLE_RANDOM.gen_range(0, indexes.len())]]
															}
														};

														let pokemon = target.pokemon.as_mut().unwrap();
														if let Some(result) = user_pkmn.use_move(*move_index, pokemon) {
															if result.result == MoveResults::Damage {
																target.renderer.flicker();
															}
															let pokemon_move = result.move_ref;
															ui::text::on_move(&mut gui.text, pokemon_move, user_pkmn);

															let effective = pokemon.effective(pokemon_move.pokemon_type);
															
															if effective != game::pokedex::pokemon::types::Effective::Effective && pokemon_move.category != MoveCategory::Status {
																ui::text::on_effective(&mut gui.text, &effective);
																// queue.actions.push_front(BattleActionInstance { pokemon: index, action: BattleAction::Effective(effective) })
															}

															if pokemon.fainted() {
																queue.actions.push_front(BattleActionInstance { pokemon: index, action: BattleAction::Faint(Some(instance.pokemon)) });
															}
															
															target.status.update_gui(Some((pokemon.data.level, pokemon)), false);
														} else {
															ui::text::on_fail(&mut gui.text, format!("{} could not move!", pokemon));
														}
													}
													BattleMove::UseItem(item, target) => {
														let item = item.value();
														if match &item.use_type {
															ItemUseType::Script(script) => {
																pokemon.execute_item_script(script);
																true
															},
															ItemUseType::Pokeball => {
																match self.data.battle_type {
																	BattleType::Wild => {
																		if let Some(target) = *target {
																			match target {
																				MoveTargetInstance::Opponent(index) => queue.actions.push_front(
																					BattleActionInstance {
																						pokemon: instance.pokemon,
																						action: BattleAction::Catch(
																							ActivePokemonIndex { team: BattleTeam::Opponent, active: index }
																						),
																					}
																				),
																			}
																			return; // To - do: remove returns
																			// ui::text::on_catch(&mut gui.text, target);
																		}
																	},
																	_ => game::macroquad::prelude::info!("Cannot use pokeballs in trainer battles!"),
																}
																false
															},
															ItemUseType::None => true,
														} {
															let level = pokemon.data.level;
															ui::text::on_item(&mut gui.text, pokemon, item);
															user.active[instance.pokemon.active].update_status(level, false);
														}
													}
													BattleMove::Switch(new) => {
														ui::text::on_switch(&mut gui.text, pokemon, user.pokemon[*new].as_ref().unwrap().value());
													}
												}
											    BattleAction::Faint(assailant) => {
													ui::text::on_faint(&mut gui.text, self.data.battle_type, instance.pokemon.team, pokemon);
													user.active[instance.pokemon.active].renderer.faint();

													if let Some(assailant) = assailant {
														if assailant.team == BattleTeam::Player {
															let experience = (
																user.active[instance.pokemon.active].pokemon.as_ref().unwrap().raw_exp_from() as f32 * 
																match self.data.battle_type {
																	BattleType::Wild => 1.0,
																	_ => 1.5,
																} *
																7.0
															) as game::pokedex::pokemon::Experience;
															let (assailant_party, index) = (&mut match assailant.team {
																BattleTeam::Player => &mut self.player,
																BattleTeam::Opponent => &mut self.opponent,
															}, assailant.active);
															if let Some(assailant_pokemon) = assailant_party.active[index].pokemon.as_mut() {
																let level = assailant_pokemon.data.level;
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
													user.active[instance.pokemon.active].update_status(*level, false);
												}
												BattleAction::LevelUp(level, moves) => {
													ui::text::on_level_up(&mut gui.text, pokemon, *level);
													if let Some(_) = moves {
														ui::text::on_fail(&mut gui.text, format!("To - do: handle moves on level up"));
													}
												}
												BattleAction::Catch(index) => {
													if let Some(target) = match index.team {
														BattleTeam::Player => &user.active[index.active],
														BattleTeam::Opponent => &other.active[index.active],
													}.pokemon.as_ref() {
														ui::text::on_catch(&mut gui.text, target);
													}
												}
											}
											queue.current = Some(BattleActionInstance { pokemon: instance.pokemon, action: instance.action });
											self.update(delta, gui, party_gui, bag);
										}
									},
									None => {
										*move_state = MoveState::SetupPost;
									}
								}
							},
						    Some(instance) => {

								let (user, other) = match instance.pokemon.team {
									BattleTeam::Player => (&mut self.player, &mut self.opponent),
									BattleTeam::Opponent => (&mut self.opponent, &mut self.player),
								};

								match &mut instance.action {

									BattleAction::Pokemon(battle_move) => match battle_move {

										BattleMove::Move(.., move_target) => {

											let target = match move_target {
												MoveTargetInstance::Opponent(index) => &mut other.active[*index],
											};

											if target.renderer.flicker.flickering() || target.status.health_moving() || !gui.text.is_finished() {
												gui.text.update(delta);
												if gui.text.current > 0 || gui.text.can_continue {
													target.renderer.flicker.update(delta);
													target.status.update_hp(delta);
												}
											} else {
												queue.current = None;
	
											}											
										}
										BattleMove::UseItem(..) => {
											if !gui.text.is_finished() {
												gui.text.update(delta)
											} else if user.active[instance.pokemon.active].status.health_moving() {
												user.active[instance.pokemon.active].status.update_hp(delta);
											} else {
												queue.current = None;
											}
										},
										BattleMove::Switch(new) => {
											if gui.text.is_finished() {
												queue.current = None;
											} else {

												gui.text.update(delta);

												if gui.text.current() == 1 && user.pokemon[*new].is_some() {
													user.replace(instance.pokemon.active, *new);
												}

											}
										}
									}
									// BattleAction::Effective(..) => text_update(delta, gui, queue),
									BattleAction::Faint(..) => {
										// let party = match target.team {
										// 	BattleTeam::Player => &mut self.player,
										// 	BattleTeam::Opponent => &mut self.opponent,
										// };
										if user.active[instance.pokemon.active].renderer.faint.fainting() {
											user.active[instance.pokemon.active].renderer.faint.update(delta);
										} else if !gui.text.is_finished() {
											gui.text.update(delta);
										} else {
											match instance.pokemon.team {
												BattleTeam::Player => {
													match party_gui.alive {
														true => {
															party_gui.input();
															party_gui.update(delta);
															if let Some(selected) = party_gui.selected.take() {
																if self.player.pokemon[selected].as_ref().map(|instance| instance.value().current_hp != 0).unwrap_or_default() {
																	self.player.queue_replace(instance.pokemon.active, selected);
																	party_gui.despawn();
																	queue.current = None;
																}
															}
														},
														false => match self.player.active[instance.pokemon.active].pokemon.is_active() {
															true => if self.player.any_inactive() {
																battle_party_gui(party_gui, &self.player, false);
															} else {
																self.player.remove_pokemon(instance.pokemon.active);
																queue.current = None;
															},
															false => queue.current = None,
														} 
													}
												}
												BattleTeam::Opponent => {
													let available: Vec<usize> = self.opponent.pokemon.iter()
														.enumerate()
														.filter(|(_, pokemon)| pokemon.as_ref().map(|instance| !instance.value().fainted()).unwrap_or(false))
														.map(|(index, _)| index)
														.collect();
	
													if !available.is_empty() {
														self.opponent.queue_replace(instance.pokemon.active, available[BATTLE_RANDOM.gen_range(0, available.len())]);
													} else {
														self.opponent.remove_pokemon(instance.pokemon.active);
													}
	
													queue.current = None;
												}
											}
										}
									}
									BattleAction::GainExp(..) => {
										let user = &mut user.active[instance.pokemon.active];
										if !gui.text.is_finished() || user.status.exp_moving() {
											gui.text.update(delta);
											if gui.text.current > 0 || gui.text.can_continue {
												user.status.update_exp(delta, user.pokemon.as_ref().unwrap());
											}
										} else {
											queue.current = None;
										}
									},
									BattleAction::LevelUp(..) => text_update(delta, gui, queue),
            						BattleAction::Catch(target) => {
										if !gui.text.is_finished() {
											gui.text.update(delta);
										} else {
											let active = &mut match target.team {
												BattleTeam::Player => &mut self.player,
												BattleTeam::Opponent => &mut self.opponent
											}.active[target.active];
											match active.pokemon.take() {
												pokemon::PokemonOption::Some(_, pokemon) => {
													active.update();
													if let Err(_) = game::storage::data_mut().party.try_push(pokemon.owned()) {
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
						self.player.run_replace();
						self.opponent.run_replace();
						// if started { stuff } else start and do calculations and add text
						self.state = if self.opponent.all_fainted() {
							self.data.winner = Some(BattleTeam::Player);
							BattleState::End
						} else if self.player.all_fainted() {
							self.data.winner = Some(BattleTeam::Opponent);
							BattleState::End
						} else {
							BattleState::SELECTING
						};
						// Once the text is finished, despawn it
						gui.text.despawn();
					},
				}
			},
    		BattleState::End => {
				bag.despawn();
				party_gui.despawn();
				gui.panel.despawn();
			},
		}
	}
	
	pub fn render(&self, gui: &BattleGui) {
		gui.background.render(0.0);
		for active in self.opponent.active.iter() {
			active.renderer.render(game::macroquad::prelude::Vec2::ZERO, game::graphics::WHITE);
			active.status.render(0.0, 0.0);
		}
		match &self.state {
			BattleState::Begin | BattleState::End => (),
		    BattleState::Selecting(index) => {
				for (current, active) in self.player.active.iter().enumerate() {
					if current.eq(index) {
						active.renderer.render(game::macroquad::prelude::Vec2::new(0.0, gui.bounce.offset), game::graphics::WHITE);
						active.status.render(0.0, -gui.bounce.offset);
					} else {
						active.renderer.render(game::macroquad::prelude::Vec2::ZERO, game::graphics::WHITE);
						active.status.render(0.0, 0.0);
					}
				}
				gui.render_panel();
				gui.panel.render();
			},
			BattleState::Moving( .. ) => {
				for active in self.player.active.iter() {
					active.renderer.render(game::macroquad::prelude::Vec2::ZERO, game::graphics::WHITE);
					active.status.render(0.0, 0.0);
				}
				gui.render_panel();
				gui.text.render();
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

		fn insert(map: &mut std::collections::BTreeMap<Priority, BattleActionInstance>, team: BattleTeam, active: &mut ActivePokemonArray) {
			for (index, active) in active.iter_mut().enumerate() {
				if let (Some(pokemon), Some(battle_move)) = (active.pokemon.as_ref(), active.queued_move.take()) {
					let index = ActivePokemonIndex { team, active: index };
					map.insert(
						match battle_move {
							BattleMove::Move(..) => Priority::Second(Reverse(0), Reverse(pokemon.base.speed), index),
							_ => Priority::First(index),
						}, 
						BattleActionInstance { pokemon: index, action: BattleAction::Pokemon(battle_move) }
					);
				}
			}
		}

		let mut map = std::collections::BTreeMap::new();

		insert(&mut map, BattleTeam::Player, player);
		insert(&mut map, BattleTeam::Opponent, opponent);

		map.into_iter().map(|(_, i)| i).collect() // into_values

	}

	pub fn update_data(self, player: &mut PlayerSave) -> Option<(BattleTeam, bool)> {

		let trainer = self.data.trainer.is_some();

		if let Some(winner) = self.data.winner {
			match winner {
			    BattleTeam::Player => {
					if let Some(trainer) = self.data.trainer {
						player.worth += trainer.worth as u32;
					}		
				}
			    BattleTeam::Opponent => {
				}
			}
		}

		self.data.winner.map(|winner| (winner, trainer))
		
	}
	
}

fn text_update(delta: f32, gui: &mut BattleGui, queue: &mut MoveQueue) {
	if !gui.text.is_finished() {
		gui.text.update(delta);
	} else {
		queue.current = None;
	}
}